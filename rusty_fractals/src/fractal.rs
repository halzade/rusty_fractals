use crate::fractal::FractalCalculationType::{DynamicSequenceNebula, StaticSequenceMandelbrot};
use crate::fractal::OrbitType::Finite;
use crate::mem::Mem;
use crate::palettes::PaletteName;
use crate::palettes::PaletteName::Nothing;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Single;
use std::cmp::PartialEq;
use FractalCalculationType::StaticImageMandelbrot;

pub struct FractalConfig {
    // fractal config
    pub name: &'static str,
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub fractal_calc_type: FractalCalculationType,
    // calculation config
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub resolution_multiplier: ResolutionMultiplier,
    pub palette: PaletteName,
    pub palette_zero: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    // calculation update config
    pub update_max: u32,
    pub update_min: u32,
}

impl FractalConfig {
    pub fn is_dynamic(&self) -> bool {
        self.fractal_calc_type == DynamicSequenceNebula
    }

    pub fn is_mandelbrot(&self) -> bool {
        self.fractal_calc_type == StaticImageMandelbrot
            || self.fractal_calc_type == StaticSequenceMandelbrot
    }
}

/**
- Orbit types for nebula fractals
*/
#[derive(PartialEq, Clone, Copy)]
pub enum OrbitType {
    // Ignore orbits for Mandelbrot like calculations
    Ignore,
    // Only edges/surface of the set
    Finite,
    // include set volume
    // this config implies humongous amount of data
    Infinite,
}

/**
 * Mandelbrot fractal
 * - for each domain element, count the calculations
 *
 * Nebula fractal
 * - for each calculation, count domain elements matching the intermediate-calculation results
 *
 * Euler fractal
 * - split primes, Fibonacci's and other calculation sequences to RGB spectra
 */
#[derive(PartialEq, Clone, Copy)]
pub enum FractalCalculationType {
    /** Nebula fractals
     * - drop calculation path to px grid immediately
     * - can't read the longest path because of that
     * - static data for image
     */
    StaticImageNebula,
    /**
     * - dynamic data for zoom sequence
     */
    DynamicSequenceNebula,
    /**
     * Mandelbrot like fractals
     * - use static data for both image and zoom sequence
     */
    StaticImageMandelbrot,
    StaticSequenceMandelbrot,
    /**
     * Euler like fractals
     * - wip
     */
    StaticSpectralImageEuler,
}

pub trait FractalMath<M>: Sync + Send {
    fn math(&self, m: &mut M, origin_re: f64, origin_im: f64);
}

pub trait MemType<M>: Sync + Send {
    fn new(re: f64, im: f64) -> M;
    fn quad(&self) -> f64;
    fn re(&self) -> f64;
    fn im(&self) -> f64;
}

/**
 * A fractal object for test purposes
 * Can't add generic because it is trivial
 */
pub struct TrivialFractal;

impl FractalMath<Mem> for TrivialFractal {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

pub fn init_trivial_fractal() -> TrivialFractal {
    TrivialFractal {}
}

/**
 * The smallest possible set to calculate upon is 20 x 20 = 400 px, because of chunks
 */
pub fn init_trivial_static_config() -> FractalConfig {
    FractalConfig {
        name: "Static",
        orbits: Finite,
        fractal_calc_type: StaticImageMandelbrot,
        iteration_min: 1,
        iteration_max: 3, // path length too short = 0,1, convergent = 2, divergent = 3
        resolution_multiplier: Single,

        palette: Nothing,
        palette_zero: Nothing,

        width_x: 20, // 1 chunk is 1 px
        height_y: 20,
        width_re: 1.0,
        center_re: 0.0,
        center_im: 0.0,

        update_max: 1,
        update_min: 0,
    }
}

pub const fn init_trivial_dynamic_config() -> FractalConfig {
    FractalConfig {
        name: "Dynamic",
        fractal_calc_type: DynamicSequenceNebula,
        iteration_min: 1,
        iteration_max: 3, // path length too short = 0,1, convergent = 2, divergent = 3
        resolution_multiplier: Single,

        palette: Nothing,
        palette_zero: Nothing,

        width_x: 20, // 1 chunk is 1 px
        height_y: 20,
        width_re: 1.0,
        center_re: 0.0,
        center_im: 0.0,

        orbits: Finite,
        update_max: 1,
        update_min: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::fractal::{init_trivial_fractal, FractalMath};
    use crate::mem::Mem;

    #[test]
    fn test_math() {
        let f = init_trivial_fractal();
        let mut m = Mem { re: 0.0, im: 0.0 };

        f.math(&mut m, 0.0, 0.0);

        assert_eq!(m.re, 0.0);
        assert_eq!(m.im, 0.0);
    }
}
