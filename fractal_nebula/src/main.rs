use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::machine;
use rusty_fractals_core::machine::Machine;
use rusty_fractals_core::fractal::{AppConfig, CalculationConfig, Math, ResultConfig};
use rusty_fractals_domain::{resolution_multiplier};
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_result::palette;
use rusty_fractals_result::fractal_result::ResultData;
use rusty_fractals_result::palettes::PALETTE_BLUE_TO_WHITE;
use rusty_fractals_common::area;
use resolution_multiplier::ResolutionMultiplier;
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
        height_y: 720,
        resolution_multiplier: SquareAlter,
    };
    let result_config = ResultConfig {
        palette: PALETTE_BLUE_TO_WHITE
    };

    info!("Fractal {}", name);

    let nebula = Nebula {};
    let area = domain_area::init(area_cfg);
    let domain = Domain {
        width: area.width_x,
        height: area.height_y,
        domain_area: area,
        domain_elements: init_domain_elements(),
        resolution_multiplier: ResolutionMultiplier::None,
    };
    let mut machine = Machine {
        area,
        domain,
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
