use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square2;
use rusty_fractals_result::palettes::palette_purple_to_white;
use rusty_fractals_result::result::ResultConfig;

const TARGET_RE: f64 = -1.40115859004747;
const TARGET_IM: f64 = -0.00000000709356;

struct NebulaTop {}

impl Fractal<Mem> for NebulaTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        // finite orbits
        length > min && iterator < max
    }
}

fn main() {
    let name = "Nebula top";

    const WIDTH: usize = 800; // 1920
    const HEIGHT: usize = 800; // 1080

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square2,
    };
    let app_config = AppConfig {
        repeat: true,
        save_images: false,
    };
    let area_config = area::AreaConfig {
        width_re: 6.0,
        center_re: TARGET_RE,
        center_im: TARGET_IM,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_purple_to_white(),
    };

    let nebula_top = NebulaTop {};
    let machine = machine::init(&calculation_config, &app_config, &result_config, &area_config);
    // todo zoom video
    let (domain_image, result_image) = machine.calculate(&nebula_top);

    window::show(name, domain_image, &result_image);
}


#[test]
fn test_math() {
    let nebula = NebulaTop {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
