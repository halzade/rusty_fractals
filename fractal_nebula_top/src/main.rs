use rusty_fractals_core::machine;
use rusty_fractals_common::fractal;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath};
use rusty_fractals_common::palettes::palette_purple_to_white;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square2;

const TARGET_RE: f64 = -1.40115859004747;
const TARGET_IM: f64 = -0.00000000709356;

struct NebulaTop {
    name: &'static str,
}

impl FractalMath<Mem> for NebulaTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for NebulaTop {
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
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square2,
        palette: palette_purple_to_white(),
    };
    let area_config = AreaConfig {
        width_re: 6.0,
        center_re: TARGET_RE,
        center_im: TARGET_IM,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let nebula_top = &NebulaTop { name: "Nebula top" };
    // todo zoom video
    machine::nebula_calculation_for(nebula_top, WIDTH, HEIGHT, fractal_config, area_config);
}


#[test]
fn test_math() {
    let nebula = NebulaTop { name: "Nebula top" };
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
