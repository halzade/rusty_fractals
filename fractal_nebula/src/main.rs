use rusty_fractals_core::{machine};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath, Update, Conf};
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal_stats::Stats;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square9;

struct Nebula {
    name: &'static str,
}

impl Nebula {}

impl FractalMath<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for Nebula {
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

impl Update for Nebula {
    fn update(&self, conf: &mut Conf, stats: &mut Stats) {
        conf.max += 150;
        if stats.not_enough_pixels_best_value {
            conf.max += 20_000;
            println!("increase ITERATION_MAX, not enough Points");
        }
        if stats.less_pixels_best_value {
            conf.max += 2_000;
            println!("increase ITERATION_MAX, bit less Points");
        }
        if stats.too_many_paths_total {
            conf.min += 1;
            println!("increase a bit ITERATION_MIN, too many paths total");
        }
        stats.print();
        stats.clean();
    }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square9,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let nebula = &Nebula { name: "Nebula" };
    machine::nebula_calculation_for(nebula, fractal_config, area_config);
}

#[test]
fn test_math() {
    let nebula = Nebula { name: "Nebula" };
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
