use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{
    FractalCommon, FractalConfig, FractalMath, FractalNebulaCommon,
};
use rusty_fractals_common::fractal_data::FractalData;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square11;
use rusty_fractals_core::application::Application;
use rusty_fractals_core::{application, machine, window};
use std::sync::Mutex;
use std::thread;

pub struct Lotus<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for Lotus<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl FractalNebulaCommon for Lotus<'_> {
    fn rm(&self) -> ResolutionMultiplier {
        self.app.resolution_multiplier
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(
        &self,
        area: &Area,
        iteration_min: u32,
        iteration_max: u32,
        origin_re: f64,
        origin_im: f64,
        data: &DataImage,
        is_wrap: bool,
    ) -> (u32, u32) {
        fractal::calculate_path(
            self,
            self,
            area,
            iteration_min,
            iteration_max,
            origin_re,
            origin_im,
            data,
            is_wrap,
        )
    }
    fn calculate_fractal(&mut self) {
        let fm = machine::init();
        fm.calculate(self);
    }
}

impl FractalCommon for Lotus<'_> {
    fn name(&self) -> &'static str {
        "Lotus"
    }
    fn update(&mut self) {
        let c = self.conf_mut();
        c.max += 150;
        println!("iteration_max = {}", c.max);
    }
    fn zoom_in(&self) {
        self.app.zoom_in()
    }

    fn data_fractal(&self) -> &FractalData {
        &self.app.data_fractal
    }

    fn width(&self) -> usize {
        self.app.width
    }

    fn height(&self) -> usize {
        self.app.height
    }

    fn data_image(&self) -> &DataImage<'static> {
        &self.app.data_image
    }

    fn palette(&self) -> &Palette {
        &self.app.palette
    }

    fn min(&self) -> u32 {
        // TODO
        self.app.data_fractal.data.lock().unwrap().min
    }

    fn max(&self) -> u32 {
        // TODO
        self.app.data_fractal.data.lock().unwrap().max
    }

    fn area(&self) -> &Area {
        &self.app.area
    }

    fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        &self
            .app
            .recalculate_pixels_positions_for_next_calculation(is_mandelbrot);
    }

    fn move_target(&self, x: usize, y: usize) {
        &self.app.move_target(x, y);
    }

    fn zoom_and_recalculate(&self) {
        &self.app.zoom_in();
        &self.app.zoom_in_recalculate_pixel_positions(true);
    }
}

pub static FRACTAL: Mutex<Option<Lotus>> = Mutex::new(None);

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square11,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: 0.0, //  0.67748277351478,
        center_im: 0.0, // -1.18770078111202,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: Lotus<'static> = Lotus { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        fractal.calculate_fractal();
        FRACTAL.lock().unwrap().replace(fractal);
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::Lotus;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_core::application;

    #[test]
    fn test_math() {
        let lotus = Lotus {
            app: application::init_none(),
        };
        let mut m = Mem { re: 0.0, im: 0.0 };
        lotus.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
