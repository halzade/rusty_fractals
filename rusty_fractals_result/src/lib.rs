pub mod palette;
pub mod palettes;
mod palette_utils;
mod result_pixel;
mod perfect_color_distribution;

const PATH: &str = "/Fractals/";

/**
 * How many pixels around specific element will be investigated for optimization.
 * If there is nothing interesting going on around specific pixel, the pixel will be ignored.
 */
const ZOOM: f64 = 0.98;
const COLORING_THRESHOLD: i32 = 3;

const REPEAT: bool = false;

fn lib() {

    let mut m = mem::Mem { re: 0.0, im: 0.0 };

    m.square();
    m.plus(1.0, 1.0);
    m.conjugation();
    m.quad();

    let cores: usize = num_cpus::get();
    let iteration: i32 = 0;

    let domain_area = domain_area::init(
        fractal::INIT_FINEBROT_AREA_SIZE,
        fractal::INIT_FINEBROT_TARGET_RE,
        fractal::INIT_FINEBROT_TARGET_IM,
    );
}