use rusty_fractals_common::area::Area;
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal::{Fractal, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;

const NAME: &str = "Fat Star Magnific";
const ITERATION_MAX: u32 = 81_000;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 3.5;
const TARGET_RE: f64 = 0.0;
const TARGET_IM: f64 = 0.0;
// TODO
// const INIT_FINEBROT_AREA_SIZE : f64= 0.15;
// const INIT_FINEBROT_TARGET_re : f64= 0.5425;
// const INIT_FINEBROT_TARGET_im : f64= -0.31;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Single;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
// const PALETTE: Palette = palette_black_to_white_exp2();

struct FatStarMagnific {
    name: &'static str,
}

impl FractalMath<Mem> for FatStarMagnific {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for FatStarMagnific {
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
    let fat_star_magnific = FatStarMagnific { name: "Fat Star" };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_magnific.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
