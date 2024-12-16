use std::sync::Mutex;
use std::thread;
use rusty_fractals_core::{application, machine, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalCommon, FractalConfig, FractalMath, FractalNebulaCommon};
use rusty_fractals_common::fractal;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_core::application::Application;

pub struct InfinityTop<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for InfinityTop<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl FractalNebulaCommon for InfinityTop<'_> {
    fn rm(&self) -> ResolutionMultiplier { self.app.resolution_multiplier }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
    fn calculate_fractal(&mut self) {
        let fm = machine::init();
        fm.calculate(self);
    }
}

impl FractalCommon for InfinityTop<'_> {
    fn name(&self) -> &'static str { "InfinityTop" }
    fn update(&mut self) {
        let c = self.conf_mut();
        c.max += 150;
        println!("iteration_max = {}", c.max);
    }
    fn move_zoom_recalculate(&mut self, x: usize, y: usize) {
        self.app.move_target_zoom_in_recalculate_pixel_positions(x, y, true);
        self.calculate_fractal_new_thread(&FRACTAL);
    }
    fn move_target_zoom_in_recalculate(x: usize, y: usize) {
        FRACTAL.lock().unwrap().as_mut().unwrap().move_zoom_recalculate(x, y);
    }
}

pub static FRACTAL: Mutex<Option<InfinityTop>> = Mutex::new(None);

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 3000,
        iteration_max: 30_000, // 180_000
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    // TODO
    // INIT_FINEBROT_AREA_SIZE = 1.8;
    // INIT_FINEBROT_TARGET_re = -1.0;
    // INIT_FINEBROT_TARGET_im = 0.0;
    let area_config = AreaConfig {
        width_x: 600,
        height_y: 600,
        width_re: 2.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: InfinityTop<'static> = InfinityTop { app: application };
    let app = window::show(&fractal);
    thread::spawn(move || {
        fractal.calculate_fractal();
        FRACTAL.lock().unwrap().replace(fractal);
    });
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_core::application;
    use crate::InfinityTop;

    #[test]
    fn test_math() {
        let infinity = InfinityTop { app: application::init_none() };
        let mut m = Mem { re: 0.0, im: 0.0 };
        infinity.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
