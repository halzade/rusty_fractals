use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_core::mem_phoenix::MemPhoenix;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_result::result::ResultConfig;

const PHOENIX_INIT_C: f64 = 0.35;
const PHOENIX_INIT_P: f64 = -0.25;
const PHOENIX_INIT_PHOENIX_INITIALIZER: f64 = 1.0;

struct GloriousHead {}

impl Fractal<MemPhoenix> for GloriousHead {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64) {
        mp.m.square();

        mp.m.re += mp.c;
        mp.m.re += mp.p * mp.prev_prev_re;
        mp.m.im += mp.p * mp.prev_prev_im;

        // previous iteration
        mp.prev_prev_re = mp.prev_re;
        mp.prev_prev_im = mp.prev_im;
        mp.prev_re = mp.m.re;
        mp.prev_im = mp.m.im;

        mp.m.plus(origin_re, origin_im);
    }

    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        // finite orbits
        length > min && iterator < max
    }
}

fn main() {
    let name = "Glorious Head";

    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    // TODO increase all these values 1000x
    let calculation_config = CalculationConfig {
        iteration_min: 8,
        iteration_max: 2500,
        resolution_multiplier: Single,
    };
    let app_config = AppConfig {
        repeat: true,
        save_images: false,
    };
    let area_config = AreaConfig {
        width_re: 4.5,
        center_re: -0.16884290496519,
        center_im: -0.37573460559804,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let glorious_head = GloriousHead {};
    let machine = machine::init(&calculation_config, &app_config, &result_config, &area_config);

    // TODO
    // let (domain_image, result_image) = machine.calculate(&glorious_head);
    // window::show(name, domain_image, &result_image);
}

#[test]
fn test_math() {
    let glorious_head = GloriousHead {};
    let m = Mem { re: 0.0, im: 0.0 };
    let mut mp = MemPhoenix { m, c: PHOENIX_INIT_C, p: PHOENIX_INIT_P, prev_prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER };

    glorious_head.math(&mut mp, 1.0, 0.1);

    assert_eq!(mp.re(), 1.1);
    assert_eq!(mp.im(), -0.15);
}
