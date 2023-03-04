use std::sync::Mutex;
use std::thread;
use rusty_fractals_core::{application, machine, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::constants::{PHOENIX_INIT_C, PHOENIX_INIT_P};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal::{Conf, FractalApplication, FractalCommon, FractalConfig, FractalMath, FractalNebulaCommon};
use rusty_fractals_common::fractal;
use rusty_fractals_common::mem_phoenix::MemPhoenix;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square9;
use rusty_fractals_core::application::Application;

pub struct Head<'lt> {
    app: Application<'lt>,
}

impl FractalMath<MemPhoenix> for Head<'_> {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64) {
        mp.square();
        mp.m.re += PHOENIX_INIT_C;
        mp.m.re += PHOENIX_INIT_P * mp.prev_prev_re;
        mp.m.im += PHOENIX_INIT_P * mp.prev_prev_im;
        // previous iteration
        mp.prev_prev_re = mp.prev_re;
        mp.prev_prev_im = mp.prev_im;
        mp.prev_re = mp.m.re;
        mp.prev_im = mp.m.im;
        mp.plus(origin_re, origin_im);
    }
}

impl FractalNebulaCommon for Head<'_> {
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

impl FractalCommon for Head<'_> {
    fn name(&self) -> &'static str { "Head" }
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

pub static FRACTAL: Mutex<Option<Head>> = Mutex::new(None);

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 8,
        iteration_max: 25000,
        resolution_multiplier: Square9,
        palette: palette_blue_to_white_circle_up(),
        phantom: Default::default(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 720,
        width_re: 5.0,
        center_re: -0.16884290496519,
        center_im: -0.37573460559804,
    };
    let application: Application<'static> = application::init_nebula(area_config, fractal_config);
    let mut head: Head<'static> = Head { app: application };
    let app = window::show(&head);
    thread::spawn(move || {
        head.calculate_fractal();
        FRACTAL.lock().unwrap().replace(head);
    });
    app.run().unwrap();
}

impl FractalApplication for Head<'_> {
    fn width(&self) -> usize { self.app.width }
    fn height(&self) -> usize { self.app.height }
    fn data(&self) -> &DataImage { &self.app.data }
    fn palette(&self) -> &Palette { &self.app.palette }
    fn max(&self) -> u32 { self.app.conf.max }
    fn conf(&self) -> &Conf { &self.app.conf }
    fn conf_mut(&mut self) -> &mut Conf { &mut self.app.conf }
    fn area(&self) -> &Area { &self.app.area }
}

#[cfg(test)]
mod tests {
    use rusty_fractals_core::application;
    use rusty_fractals_common::constants::PHOENIX_INITIALIZER;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_phoenix::MemPhoenix;
    use crate::Head;

    #[test]
    fn test_math() {
        let head = Head { app: application::init_none() };
        let mut mp = MemPhoenix { m: Mem { re: 0.0, im: 0.0 }, prev_prev_re: PHOENIX_INITIALIZER, prev_prev_im: PHOENIX_INITIALIZER, prev_re: PHOENIX_INITIALIZER, prev_im: PHOENIX_INITIALIZER };
        head.math(&mut mp, 1.0, 0.1);
        assert_eq!(mp.re(), 1.1);
        assert_eq!(mp.im(), -0.15);
    }
}
