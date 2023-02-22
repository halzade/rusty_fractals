use rusty_fractals_common::area::Area;
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal::{Fractal, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;

const ITERATION_MAX: u32 = 180_000;
const ITERATION_MIN: u32 = 3000;
const AREA_SIZE: f64 = 2.5;
const TARGET_RE: f64 = -0.5;
const TARGET_IM: f64 = 0.0;
// INIT_FINEBROT_AREA_SIZE = 1.8;
// INIT_FINEBROT_TARGET_re = -1.0;
// INIT_FINEBROT_TARGET_im = 0.0;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Single;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
// const PALETTE: Palette = palette_black_to_white_exp2();

struct InfinityTop {
    name: &'static str,
}

impl FractalMath<Mem> for InfinityTop {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for InfinityTop {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data_image: &DataImage) -> (u32, u32) {
        todo!()
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    // TODO
}


#[test]
fn test_math() {
    let infinity_top = InfinityTop { name: "Infinity" };
    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity_top.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
