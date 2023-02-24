use rusty_fractals_core::machine;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::constants::{PHOENIX_INIT_C, PHOENIX_INIT_P};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath};
use rusty_fractals_common::mem_phoenix::MemPhoenix;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square9;

struct Head {
    name: &'static str,
}

impl FractalMath<MemPhoenix> for Head {
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

impl Fractal for Head {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    let fractal_config = FractalConfig {
        iteration_min: 8,
        iteration_max: 25000,
        resolution_multiplier: Square9,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_re: 5.0,
        center_re: -0.16884290496519,
        center_im: -0.37573460559804,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let head = &Head { name: "Head" };
    machine::nebula_calculation_for(head, WIDTH, HEIGHT, fractal_config, area_config);
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::constants::PHOENIX_INIT_PHOENIX_INITIALIZER;
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_phoenix::MemPhoenix;
    use crate::Head;

    #[test]
    fn test_math() {
        let head = &Head { name: "Head" };
        let mut mp = MemPhoenix { m: Mem { re: 0.0, im: 0.0 }, prev_prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER };
        head.math(&mut mp, 1.0, 0.1);
        assert_eq!(mp.re(), 1.1);
        assert_eq!(mp.im(), -0.15);
    }
}
