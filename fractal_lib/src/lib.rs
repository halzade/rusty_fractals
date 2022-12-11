pub mod mem;
pub mod mem_collatz;
pub mod domain;
pub mod domain_area;
pub mod domain_element;
pub mod fractal;
pub mod fractal_stats;
pub mod pixel_states;
pub mod resolution_multiplier;
pub mod color_palette;
pub mod color_palettes;
pub mod mem_euler;
pub mod mem_phoenix;
mod mathematician;

const PATH: &str = "/Fractals/";

/**
 * How many pixels around specific element will be investigated for optimization.
 * If there is nothing interesting going on around specific pixel, the pixel will be ignored.
 */
const NEIGHBOURS: i32 = 3;
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}