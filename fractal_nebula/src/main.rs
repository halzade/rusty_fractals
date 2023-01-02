use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::machine::Machine;
use rusty_fractals_core::fractal::{AppConfig, CalculationConfig, Math, ResultConfig};
use rusty_fractals_domain::{resolution_multiplier};
use rusty_fractals_domain::domain::{Domain, init_domain_elements};
use rusty_fractals_result::palettes::palette_blue_to_white;
use rusty_fractals_common::area;
use resolution_multiplier::ResolutionMultiplier::SquareAlter;
use log::{info};

struct Nebula {}

impl Math<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name = "Nebula";

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_cfg = area::AreaConfig {
        width_re: 7.0,
        center_re: 0.0,
        center_im: 0.0,
        width_x: 1280,
        height_y: 720
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white()
    };

    info!("Fractal {}", name);

    let nebula = Nebula {};
    let domain_area = area::init(area_cfg);
    let domain = Domain {
        width: domain_area.width_x,
        height: domain_area.height_y,
        domain_area: &domain_area,
        domain_elements: init_domain_elements(&domain_area),
        resolution_multiplier: SquareAlter,
    };
    let mut machine = Machine {
        area: &domain_area,
        domain: &domain,
        calculation_config,
        app_config,
        result_config,
    };

    machine.calculate(&nebula);

    info!("Finished.");
}


#[test]
fn test_math() {
    let nebula = Nebula {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
