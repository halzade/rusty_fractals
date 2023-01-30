use rusty_fractals_core::{machine, window};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square11;
use rusty_fractals_result::palettes::{palette_blue_to_white_circle_up};
use rusty_fractals_result::result::ResultConfig;

struct Lotus {}

impl Fractal<Mem> for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        // finite orbits
        length > min && iterator < max
    }
}

fn main() {
    let name = "Lotus";

    const WIDTH: usize = 1280;
    const HEIGHT: usize = 1000;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square11,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_config = AreaConfig {
        width_re: 3.5,
        center_re: 0.0, //  0.67748277351478,
        center_im: 0.0, // -1.18770078111202,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let lotus = Lotus {};
    let machine = machine::init(&calculation_config, &app_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&lotus);

    window::show(name, domain_image, result_image);
}
