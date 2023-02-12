use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_result::result::ResultConfig;

struct Infinity {}

impl Fractal<Mem> for Infinity {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        // infinite orbits
        length > min && iterator == max
    }
}


fn main() {
    let name = "Infinity";

    const WIDTH: usize = 1000; // 1920
    const HEIGHT: usize = 1000; // 1080

    let calculation_config = CalculationConfig {
        iteration_min: 3000,
        iteration_max: 30_000, // 180_000
        resolution_multiplier: Single, //Square9,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_config = AreaConfig {
        width_re: 2.6,
        center_re: -0.5,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let infinity = Infinity {};
    let machine = machine::init(&calculation_config, &app_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&infinity);

    window::show(name, domain_image, &result_image);
}

#[test]
fn test_math() {
    let infinity = Infinity {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
