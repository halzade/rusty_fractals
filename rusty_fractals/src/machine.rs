use crate::application::Application;
use crate::area::Area;
use crate::constants::CALCULATION_BOUNDARY;
use crate::data_image::DataImage;
use crate::data_image::DataType::Static;
use crate::data_px::{active_new, hibernated_deep_black};
use crate::fractal::CalculationType::StaticImage;
use crate::fractal::FractalType::MandelbrotType;
use crate::fractal::{
    CalculationType, FractalConfig, FractalMath, FractalType, OrbitType, TrivialFractal,
};
use crate::fractal_log::now;
use crate::fractal_stats::Stats;
use crate::mem::Mem;
use crate::palette::Palette;
use crate::palettes::new_palette_by_name;
use crate::perfect_colour_distribution::perfectly_colour_nebula_values;
use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{FinishedSuccess, FinishedTooLong, FinishedTooShort};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{
    application, area, data_image, fractal, fractal_stats, palettes, perfect_colour_distribution,
    pixel_states,
};
use fltk::enums::Color;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/**
 * Machine owns all data
 */
pub struct Machine<'lt, F>
where
    F: FractalMath,
{
    pub name: &'lt str,
    pub fractal: F,
    pub fractal_type: FractalType,
    // area config
    pub area: Area,
    pub width_x: usize,  // width x in pixels
    pub height_y: usize, // with y in pixels
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    // display config
    pub data_image: DataImage,
    pub palette: Palette,
    // mandelbrot specific
    // used to color the (black) inside of Mandelbrot set
    pub palette_zero: Palette,
    // calculation config
    pub calc_type: CalculationType,
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub update_max: u32,
    pub update_min: u32,
    // calculation statistics for video zoom
    pub stats: Stats,
    //  nebula specific - use multiple numbers for each screen pixel
    pub resolution_multiplier: ResolutionMultiplier,
    pub app_ref: Option<Arc<Mutex<Application>>>,
}

pub fn init<F: FractalMath>(config: &FractalConfig, fractal: F) -> Machine<'static, F> {
    let area: Area = area::init(config);
    Machine {
        fractal,
        name: config.name,
        data_image: data_image::init(Static, &area), // TODO Dynamic
        area,
        width_x: config.width_x,
        height_y: config.height_y,
        width_re: config.width_re,
        center_re: config.center_re,
        center_im: config.center_im,
        // mandelbrot fractals calculate from 0
        // nebula fractals include only calculations longer then min
        iteration_min: config.iteration_min,
        iteration_max: config.iteration_max,
        palette: new_palette_by_name(&config.palette),
        palette_zero: new_palette_by_name(&config.palette_zero),
        calc_type: config.calc_type,
        resolution_multiplier: config.resolution_multiplier,
        fractal_type: config.fractal_type,
        orbits: OrbitType::Finite,
        update_max: config.update_max,
        update_min: config.update_min,
        stats: fractal_stats::init(),
        // application reference
        app_ref: None,
    }
}

pub fn init_trivial() -> Machine<'static, TrivialFractal> {
    Machine {
        fractal: fractal::init_trivial(),
        name: "Trivial Fractal",
        data_image: data_image::init_trivial(),
        area: area::init_trivial(),
        width_x: 2,
        height_y: 2,
        width_re: 2.0,
        center_re: 0.0,
        center_im: 0.0,
        palette: palettes::init_trivial(),
        palette_zero: palettes::init_trivial(),
        calc_type: StaticImage,
        orbits: OrbitType::Finite,
        iteration_min: 0,
        iteration_max: 10,
        update_max: 3,
        update_min: 1,
        resolution_multiplier: ResolutionMultiplier::Single,
        fractal_type: MandelbrotType,
        stats: fractal_stats::init(),
        app_ref: None,
    }
}

impl<'lt, F: FractalMath> Machine<'lt, F> {
    pub fn execute_calculation(&mut self, app_ref: Arc<Mutex<Application>>) {
        println!("trigger_calculation()");

        let is_mandelbrot = self.fractal_type == MandelbrotType;
        let is_image = self.calc_type == StaticImage;

        if is_mandelbrot {
            if is_image {
                // Fine fractal image
                self.calculate_mandelbrot();
            } else {
                // Fine fractal video
                self.calculate_mandelbrot_zoom();
            }
        } else {
            if is_image {
                // Hard fractal image
                self.calculate_nebula();
            } else {
                // Hard fractal video
                self.calculate_nebula_zoom();
            }
        }

        let app = app_ref.lock().unwrap();
        app.window_repaint(Color::Green);
    }

    /**
     * Calculate the whole Nebula fractal
     */
    pub fn calculate_nebula(&self) {
        println!("calculate_nebula()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy);
            // window refresh
            // application::paint_image_calculation_progress(xy, &self.data_image);
        });
        self.data_image.recalculate_pixels_states();

        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                // calculation
                self.chunk_calculation_with_wrap(&xy);
                // window refresh
                // application::paint_image_calculation_progress(xy, &self.data_image);
                // application::paint_path(&self.area, &self.data_image);
            });
        }
        perfectly_colour_nebula_values(&self.data_image, &self.palette);
        // TODO application::paint_image_result(&self.data_image);

        // let app = self.app_ref.as_ref().unwrap().lock().unwrap();
        // app.window_repaint(Color::Green);
    }

    // in sequence executes as 20x20 parallel for each image part/chunk
    fn chunk_calculation(&self, xy: &[u32; 2]) {
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y);
            }
        }
    }

    fn chunk_calculation_with_wrap(&self, xy: &[u32; 2]) {
        if self.resolution_multiplier == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        let plank = self.area.plank();
        for x in x_from..x_to {
            for y in y_from..y_to {
                if self.data_image.is_on_mandelbrot_horizon(x, y) {
                    let (origin_re, origin_im) = self.data_image.origin_at(x, y);
                    let wrap = self.data_image.wrap(
                        origin_re,
                        origin_im,
                        self.resolution_multiplier,
                        plank,
                    );
                    // within the same pixel
                    for [re, im] in wrap {
                        self.calculate_path(re, im, true);
                    }
                }
            }
        }
    }

    fn calculate_path_xy(&self, x: usize, y: usize) {
        let (state, origin_re, origin_im) = self.data_image.state_origin_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = self.calculate_path(origin_re, origin_im, false);
            let state = self.state_from_path_length(iterator, path_length);
            self.data_image.set_pixel_state(x, y, state);
        }
    }

    pub fn move_target(&self, x: usize, y: usize) {
        self.area.move_target(x, y);
    }

    pub fn zoom_in_recalculate_pixel_positions(&self) {
        self.area.zoom_in();
        application::paint_image_result(&self.data_image);

        self.recalculate_pixels_positions_for_next_calculation();
        application::paint_image_result(&self.data_image);
    }

    pub fn zoom_in(&self) {
        self.area.zoom_in();
    }

    // This is called after calculation finished, a zoom-in was called and new area measures recalculated
    pub fn recalculate_pixels_positions_for_next_calculation(&self) {
        println!("recalculate_pixels_positions_for_next_calculation()");
        // Scan all elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, subsequent calculations will be skipped.
        let area = &self.area;
        let cre = area.data.lock().unwrap().center_re;
        let cim = area.data.lock().unwrap().center_im;

        let (cx, cy) = area.point_to_pixel(cre, cim);

        now("1. move top left to center");
        for y in 0..cy {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("2. move top right to center");
        for y in 0..cy {
            for x in (cx..self.width_x).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("3. move bottom left to center");
        for y in (cy..self.height_y).rev() {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("4. move bottom right to center");
        for y in (cy..self.height_y).rev() {
            for x in (cx..self.width_x).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        // Create new elements on positions where no px moved to
        now("fill empty places");
        let mut c_moved = 0;
        let mut c_created = 0;

        let res = area.screen_to_domain_re_copy();
        let ims = area.screen_to_domain_im_copy();

        for y in 0..self.height_y {
            for x in 0..self.width_x {
                let mut mo_px = self.data_image.mo_px_at(x, y);
                if mo_px.is_none() {
                    c_created += 1;

                    let re = res[x];
                    let im = ims[y];

                    if self.data_image.all_neighbors_finished_bad(
                        x,
                        y,
                        self.fractal_type == MandelbrotType,
                    ) {
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        mo_px.replace(hibernated_deep_black(re, im));
                    } else {
                        mo_px.replace(active_new(re, im));
                    }
                } else {
                    c_moved += 1;
                }
            }
        }
        println!("moved:     {}", c_moved);
        println!("created:   {}", c_created);
        assert!(c_moved > 0);
        assert!(c_created > 0);
    }

    pub fn chunk_boundaries(&self, xy: &[u32; 2]) -> (usize, usize, usize, usize) {
        let chunk_size_x = (self.width_x / 20) as u32;
        let chunk_size_y = (self.height_y / 20) as u32;
        (
            (xy[0] * chunk_size_x) as usize,
            ((xy[0] + 1) * chunk_size_x) as usize,
            (xy[1] * chunk_size_y) as usize,
            ((xy[1] + 1) * chunk_size_y) as usize,
        )
    }

    pub fn path_test(&self, length: u32, iterator: u32) -> bool {
        if self.orbits == OrbitType::Finite {
            // only the edges of mandelbrot set
            length > self.iteration_min && iterator < self.iteration_max
        } else {
            // also contains the inside of mandelbrot set
            length > self.iteration_min && iterator == self.iteration_max
        }
    }

    pub fn calculate_path(&self, origin_re: f64, origin_im: f64, is_wrap: bool) -> (u32, u32) {
        let cb = CALCULATION_BOUNDARY as f64;

        let mut m = Mem::new(origin_re, origin_im);

        let mut iterator = 0;
        let mut length = 0;
        while m.quad() < cb && iterator < self.iteration_max {
            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and to record exclusively the good paths
            self.fractal.math(&mut m, origin_re, origin_im);
            if self.area.contains(m.re(), m.im()) {
                // this becomes important for zoom, when only a small amount
                // of calculation path elements is contained withing tiny area
                length += 1;
            }
            iterator += 1;
        }

        if self.path_test(length, iterator) {
            // This origin produced good data
            // Record the calculation path

            let mut m = Mem::new(origin_re, origin_im);

            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                self.fractal.math(&mut m, origin_re, origin_im);
                if self.area.contains(m.re(), m.im()) {
                    path.push([m.re(), m.im()]);
                }
            }

            // if iteration_max increased, ignore possible extension of previous calculation paths
            // path elements are going to migrate out of the screen shortly
            // removed last_iteration, last_visited_re, last_visited_im
            if self.data_image.is_dynamic() {
                self.data_image.save_path(path, is_wrap);
            } else {
                self.data_image
                    .translate_path_to_point_grid(path, &self.area, is_wrap);
            }
            // TODO stats.paths_new_points_amount += path.size();
        }
        (iterator, length)
    }

    pub fn state_from_path_length(&self, iterator: u32, path_length: u32) -> DomainElementState {
        if path_length < self.update_min {
            return FinishedTooShort;
        }
        if iterator == self.update_max {
            return FinishedTooLong;
        }
        FinishedSuccess
    }

    /* --------------------------------------------
     * Methods for infinite zoom video calculations
     * ------------------------------------------ */

    pub fn calculate_nebula_zoom(&self) {
        println!("calculate_nebula_zoom()");
        for it in 1.. {
            println!("{}:", it);
            self.calculate_nebula();

            // prepare next frame
            self.zoom_in();
            self.recalculate_pixels_positions_for_next_calculation();
            // TODO self.stats.update(&self.data_image, it);
        }
    }

    pub fn calculate_mandelbrot_zoom(&self) {
        println!("calculate_mandelbrot_zoom()");
        for it in 1.. {
            println!("{}:", it);
            self.calculate_mandelbrot();

            // prepare next frame
            self.zoom_in();
            self.recalculate_pixels_positions_for_next_calculation();
            // TODO self.stats.update(&self.data_image, it);
        }
    }

    /* ------------------------------------------
     * Methods for Mandelbrot fractal calculation
     * --------------------------------------- */

    /**
     * Whole Mandelbrot calculation
     */
    pub fn calculate_mandelbrot(&self) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();

        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(xy);
            // window refresh
            // application::paint_image_calculation_progress(xy, &self.data_image);
        });
        self.data_image.recalculate_pixels_states();
        perfect_colour_distribution::perfectly_colour_mandelbrot_values(
            &self.data_image,
            &self.palette,
            &self.palette_zero,
        );
        application::paint_image_result(&self.data_image);
    }

    fn chunk_calculation_mandelbrot(&self, xy: &[u32; 2]) {
        let (x_from, x_to, y_from, y_to) = self.chunk_boundaries(xy);
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (state, origin_re, origin_im) = self.data_image.state_origin_at(x, y);
                // TODO, calculate only ActiveNew elements, copy quad and quid
                if !pixel_states::is_finished_any(state) {
                    // calculation
                    let (iterator, quad) = self.calculate_mandelbrot_path(origin_re, origin_im);
                    let state = self.state_from_path_length(iterator, iterator);
                    self.data_image.set_pixel_mandelbrot(
                        x,
                        y,
                        iterator,
                        quad,
                        state,
                        self.iteration_max,
                    );
                }
            }
        }
    }

    pub fn calculate_mandelbrot_path(&self, origin_re: f64, origin_im: f64) -> (u32, f64) {
        let cb = CALCULATION_BOUNDARY as f64;

        let mut m = Mem::new(origin_re, origin_im);

        let mut iterator = 0;
        while m.quad() < cb && iterator < self.iteration_max {
            self.fractal.math(&mut m, origin_re, origin_im);
            iterator += 1;
        }
        (iterator, m.quad())
    }
}

/**
 * Creates x,y pairs for calculation.
 * Then shuffles them, it looks better when rendering
 */
pub fn shuffled_calculation_coordinates() -> Vec<[u32; 2]> {
    let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();

    // 400 little subdivisions of the screen
    for x in 0..20 {
        for y in 0..20 {
            coordinates_xy.push([x, y]);
        }
    }
    coordinates_xy.shuffle(&mut thread_rng());
    coordinates_xy
}

#[cfg(test)]
mod tests {
    use crate::{fractal, machine};

    #[test]
    fn test_calculate_path_xy() {
        let fractal = fractal::init_trivial();
        let machine = machine::init_trivial();
        machine.calculate_path_xy(0, 0);

        // TODO
    }

    #[test]
    fn test_chunk_calculation_mandelbrot<'lt>() {
        let mm = machine::init_trivial();

        let x = 0;
        let y = 0;
        let xy = [x, y];

        mm.chunk_calculation_mandelbrot(&xy);

        // TODO
    }

    #[test]
    fn test_calculate_path() {
        // prepare test data
        let fractal = fractal::init_trivial();
        let machine = machine::init_trivial();

        // execute test
        let (iterator, length) = machine.calculate_path(0.0, 0.0, false);

        assert_eq!(iterator, 5);
        assert_eq!(length, 0);
    }
}
