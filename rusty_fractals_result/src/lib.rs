pub mod palette;
pub mod palettes;
mod palette_utils;
mod result_pixel;
mod perfect_color_distribution;
pub mod fractal_result;

const PATH: &str = "/Fractals/";

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
}