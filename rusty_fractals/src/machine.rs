use crate::application::Application;
use crate::area::Area;
use crate::constants::CALCULATION_BOUNDARY;
use crate::data_image::DataImage;
use crate::data_px::{active_new, hibernated_deep_black};
use crate::fractal::CalculationType::StaticImage;
use crate::fractal::FractalType::MandelbrotType;
use crate::fractal::{
    init_trivial_config, CalculationType, FractalConfig, FractalMath, FractalType, MemType,
    OrbitType, TrivialFractal,
};
use crate::fractal_log::now;
use crate::fractal_stats::Stats;
use crate::mem::Mem;
use crate::palette::Palette;
use crate::palettes::new_palette_by_name;
use crate::perfect_colour_distribution::{
    perfectly_colour_mandelbrot_values, perfectly_colour_nebula_values,
};
use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{FinishedSuccess, FinishedTooLong, FinishedTooShort};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{area, data_image, fractal, fractal_stats, pixel_states};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/**
 * Machine owns all data
 */
pub struct Machine<'lt, F, M>
where
    F: FractalMath<M>,
    M: MemType<M>,
{
    /*
     * Fractal related values
     */
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
    /*
     * Application related values
     */
    pub app_ref: Option<Arc<Mutex<Application>>>,
    /*
     * Machine (Self) related values
     */
    last_partial_refresh: Arc<Mutex<Option<Instant>>>,
    // use to create new memories
    m: M,
}

pub fn init<F, M>(config: &FractalConfig, fractal: F) -> Machine<'static, F, M>
where
    F: FractalMath<M>,
    M: MemType<M>,
{
    let area: Area = area::init(config);
    Machine {
        fractal,
        name: config.name,
        data_image: data_image::init(config.data_image_type, &area),
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
        // machine values
        last_partial_refresh: Arc::new(Mutex::new(None)),
        m: M::new(0.0, 0.0),
    }
}

pub fn init_trivial() -> Machine<'static, TrivialFractal, Mem> {
    let conf = init_trivial_config();
    let fractal = fractal::init_trivial_fractal();
    init(&conf, fractal)
}

impl<'lt, F, M> Machine<'lt, F, M>
where
    F: FractalMath<M>,
    M: MemType<M>,
{
    pub fn set_application_ref(&mut self, app_ref: Arc<Mutex<Application>>) {
        self.app_ref = Some(app_ref);
    }

    pub fn execute_calculation(&self) {
        println!("trigger_calculation()");

        let is_mandelbrot = self.fractal_type == MandelbrotType;
        let is_image = self.calc_type == StaticImage;

        if is_mandelbrot {
            if is_image {
                // Hard fractal image
                self.calculate_mandelbrot();
            } else {
                // Hard fractal video
                self.calculate_mandelbrot_zoom();
            }
        } else {
            if is_image {
                // Fine fractal image
                self.calculate_nebula();
            } else {
                // Fine fractal video
                self.calculate_nebula_zoom();
            }
        }
    }

    /* --------------------------------------
     * Methods for Nebula fractal calculation
     * ----------------------------------- */

    /**
     * Calculate the whole Nebula fractal
     */
    pub fn calculate_nebula(&self) {
        println!("calculate_nebula()");

        let coordinates_xy = shuffled_calculation_coordinates();

        // calculation for a center of each pixel
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy);
            // window refresh
            // need to paint full image to show progress from other unfinished chunks
            self.paint_partial_calculation_results_states(false);
        });

        self.data_image.recalculate_pixels_states();

        // wrap
        // calculate for many other elements within the pixels
        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                // calculation
                self.chunk_calculation_with_wrap(&xy);
                // window refresh
                // need to paint full image to show progress from other unfinished chunks
                self.paint_partial_calculation_results_states(true); // only every 100+ ms
            });
        }
        perfectly_colour_nebula_values(&self.data_image, &self.palette);

        self.paint_final_calculation_result_colors();
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
        // application::paint_image_result(&self.data_image);

        self.recalculate_pixels_positions_for_next_calculation();
        // application::paint_image_result(&self.data_image);
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

    /**
     * domain on which the image is calculate is split to 20 x 20 = 400 chunks
     * this method returns numbers from 0 to 20
     */
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

        let mut m = M::new(origin_re, origin_im);

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

            let mut m = M::new(origin_re, origin_im);

            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                self.fractal.math(&mut m, origin_re, origin_im);
                if self.area.contains(m.re(), m.im()) {
                    path.push([m.re(), m.im()]);
                }
            }

            // if iteration_max increased, ignore possible extension of previous calculation paths
            // path elements are going to migrate out of the screen very soon
            // removed last_iteration, last_visited_re, last_visited_im

            self.stats.paths_new_points_amount_add(*&path.len());

            if self.data_image.is_dynamic() {
                self.data_image.save_path(path, is_wrap);
            } else {
                self.data_image
                    .translate_path_to_point_grid(path, &self.area, is_wrap);
            }
        }
        (iterator, length)
    }

    pub fn state_from_path_length(&self, iterator: u32, path_length: u32) -> DomainElementState {
        // path length considered only within Area
        if path_length < self.iteration_min {
            // 0 to min-1
            return FinishedTooShort;
        }
        if iterator == self.iteration_max {
            // divergent calculation
            // some of the path elements may be outside of Area
            return FinishedTooLong;
        }
        // min to max-1
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
            self.paint_partial_calculation_results_states(false);
            self.stats.update(&self.data_image, it);
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
            self.paint_partial_calculation_results_states(false);
            self.stats.update(&self.data_image, it);
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
            self.paint_partial_calculation_results_states(false);
        });
        self.data_image.recalculate_pixels_states();
        perfectly_colour_mandelbrot_values(&self.data_image, &self.palette, &self.palette_zero);
        self.paint_final_calculation_result_colors();
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

        let mut m = M::new(origin_re, origin_im);

        let mut iterator = 0;
        while m.quad() < cb && iterator < self.iteration_max {
            self.fractal.math(&mut m, origin_re, origin_im);
            iterator += 1;
        }
        (iterator, m.quad())
    }

    /* -------------------
     * Application methods
     * -----------------*/

    /**
     * This method will paint only final image colors, no pixel states
     */
    pub fn paint_final_calculation_result_colors(&self) {
        let app = self
            .app_ref
            .as_ref()
            .unwrap()
            .lock()
            .expect("Failed to lock application reference");

        app.paint_final_calculation_result_colors(&self.data_image);
    }

    /**
     * Paint partial results to show pixel states
     * The pixel states, which are finished show color instead
     */
    pub fn paint_partial_calculation_results_states(&self, paint_path: bool) {
        // ms_min have serious impact on parallelization and speed of calculation,
        // don't use less than 100
        let ms_min = 250;

        let now = Instant::now();

        let mut last_called = self
            .last_partial_refresh
            .lock()
            .expect("Failed to lock last_called");

        let called_in_past_enough = last_called.map_or(true, |last| {
            now.duration_since(last) >= Duration::from_millis(ms_min)
        });

        if called_in_past_enough {
            let app = self
                .app_ref
                .as_ref()
                .unwrap()
                .lock()
                .expect("Failed to lock application reference");

            app.paint_partial_calculation_result_states(&self.data_image, paint_path, &self.area);
        }

        *last_called = Some(now);
    }
}

/* --------------
 * Static methods
 * ----------- */

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
    use crate::pixel_states::DomainElementState::{FinishedSuccess, FinishedTooLong};
    use crate::{machine, pixel_states};
    use pixel_states::DomainElementState::FinishedTooShort;

    #[test]
    fn test_state_from_path_length() {
        let machine = machine::init_trivial();

        let zero = machine.state_from_path_length(0, 0);
        let short_out = machine.state_from_path_length(1, 0);
        let short_in = machine.state_from_path_length(1, 1);
        let convergent_out = machine.state_from_path_length(2, 0);
        let convergent_in1 = machine.state_from_path_length(2, 1);
        let convergent_in2 = machine.state_from_path_length(2, 2);
        let divergent_out = machine.state_from_path_length(3, 0);
        let divergent_in1 = machine.state_from_path_length(3, 1);
        let divergent_in2 = machine.state_from_path_length(3, 2);
        let divergent_in3 = machine.state_from_path_length(3, 3);

        assert_eq!(zero, FinishedTooShort);
        assert_eq!(short_out, FinishedTooShort);
        assert_eq!(short_in, FinishedSuccess);

        assert_eq!(convergent_out, FinishedTooShort);
        assert_eq!(convergent_in1, FinishedSuccess);
        assert_eq!(convergent_in2, FinishedSuccess);

        assert_eq!(divergent_out, FinishedTooShort);
        assert_eq!(divergent_in1, FinishedTooLong);
        assert_eq!(divergent_in2, FinishedTooLong);
        assert_eq!(divergent_in3, FinishedTooLong);
    }

    #[test]
    fn test_chunk_boundaries() {
        let machine = machine::init_trivial();

        let (re_left, re_right, im_top, im_bot) = machine.chunk_boundaries(&[0, 0]);

        assert_eq!(re_left, 0);
        assert_eq!(re_right, 1);

        assert_eq!(im_top, 0);
        assert_eq!(im_bot, 1);
    }

    #[test]
    fn test_calculate_path_xy() {
        let machine = machine::init_trivial();

        // test condition
        let (s, _, _) = machine.data_image.state_origin_at(0, 0);
        assert_eq!(pixel_states::is_active_new(s), true);

        // test result
        machine.calculate_path_xy(0, 0);
        let (s, _, _) = machine.data_image.state_origin_at(0, 0);

        assert_eq!(pixel_states::is_active_new(s), false);
        assert_eq!(pixel_states::is_finished_any(s), true);
    }

    #[test]
    fn test_chunk_calculation_mandelbrot<'lt>() {
        let machine = machine::init_trivial();

        let xy = [0, 0];

        machine.chunk_calculation_mandelbrot(&xy);
        let (s, _, _) = machine.data_image.state_origin_at(0, 0);

        println!("state: {:?}", s);

        assert_eq!(pixel_states::is_active_new(s), false);
        assert_eq!(pixel_states::is_finished_any(s), true);
    }

    #[test]
    fn test_calculate_path() {
        // prepare test data
        let machine = machine::init_trivial();

        // execute test
        let (iterator, length) = machine.calculate_path(0.7, 0.7, false);

        assert_eq!(iterator, 2); // trivial iteration_max = 3
        assert_eq!(length, 0);
    }
}
