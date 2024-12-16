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
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square3;
use rusty_fractals_core::application::Application;

pub struct NebulaOfNebula<'lt> {
    app: Application<'lt>,
}

impl FractalMath<Mem> for NebulaOfNebula<'_> {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or - 2.0 * i * i * or - 4.0 * r * i * oi + or * or - oi * oi + or - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi - 2.0 * i * i * oi + 2.0 * or * oi + oi - i;
    }
}

impl FractalNebulaCommon for NebulaOfNebula<'_> {
    fn rm(&self) -> ResolutionMultiplier { self.app.resolution_multiplier }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
    fn calculate_fractal(&mut self) {
        let fm = machine::init();
        fm.calculate(self);
    }
}

impl FractalCommon for NebulaOfNebula<'_> {
    fn name(&self) -> &'static str { "Nebula of Nebula" }
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

pub static FRACTAL: Mutex<Option<NebulaOfNebula>> = Mutex::new(None);

fn main() {
    let fractal_config: FractalConfig<'static> = FractalConfig {
        iteration_min: 42,
        iteration_max: 24800,
        resolution_multiplier: Square3,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 0.5,
        center_re: 0.0,
        center_im: 0.0,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut fractal: NebulaOfNebula<'static> = NebulaOfNebula { app: application };
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
    use crate::NebulaOfNebula;

    #[test]
    fn test_math() {
        let nebula = NebulaOfNebula { app: application::init_none() };
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
