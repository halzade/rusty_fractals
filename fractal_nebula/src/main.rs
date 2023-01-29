use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Math};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::{Square5};
use rusty_fractals_domain::{domain};
use rusty_fractals_result::palettes::{palette_blue_to_white};
use rusty_fractals_result::result::ResultConfig;

struct Nebula {}

impl Math<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name = "Nebula";

    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square5,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_cfg = area::AreaConfig {
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white(),
    };

    println!("Fractal {}", name);

    let nebula = Nebula {};
    let area = area::init(&area_cfg);
    let domain = domain::init(&area);
    let machine = machine::init(&calculation_config, &app_config, &result_config);

    let (domain_image, result_image) = machine.calculate(&nebula, &domain, &area);

    window::show(name, domain_image, result_image);
}

#[test]
fn test_math() {
    let nebula = Nebula {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
