extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod mem;
pub mod collatz_mem;
pub mod domain;
pub mod domain_area;
pub mod domain_element;
pub mod fractal;
pub mod fractal_stats;
pub mod pixel_states;
pub mod resolution_multiplier;

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
    pretty_env_logger::init();

    info!("such information");
    warn!("o_O");
    error!("much error");

    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

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