use crate::data_image::DataImage;
use crate::mem::Mem;
use crate::palettes::PaletteName;
use crate::resolution_multiplier::ResolutionMultiplier;
use std::cmp::PartialEq;

/**
 * Represents the actual mathematical object
 */
pub struct Fractal<'lt> {
    data_image: DataImage<'lt>,
}

pub struct FractalConfig<'lt> {
    // fractal config
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub fractal_type: FractalType,
    pub resolution_multiplier: ResolutionMultiplier,
    pub palette: PaletteName,
    pub palette_zero: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    // calculation config
    pub calc_type: CalculationType,
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub update_max: u32,
    pub update_min: u32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum FractalType {
    // for each domain element, count the calculation
    Mandelbrot,
    // for each calculation, count domain elements matching the intermediate-calculation results
    Nebula,
}

/**
- Orbit types for nebula fractals
*/
#[derive(PartialEq, Clone, Copy)]
pub enum OrbitType {
    // Only edges/surface of the set
    Finite,
    // include set volume
    Infinite,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CalculationType {
    StaticImage,
    InfiniteVideoZoom,
}

pub trait FractalMath<T: MemType<T>>: Sync + Send {
    fn math(&self, m: &mut T, origin_re: f64, origin_im: f64);
}

pub trait MemType<T> {
    fn new(re: f64, im: f64) -> T;
    fn quad(&self) -> f64;
    fn re(&self) -> f64;
    fn im(&self) -> f64;
}

/**
 * A fractal object for test purposes
 */
pub struct TrivialFractal {}

impl FractalMath<Mem> for TrivialFractal {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

pub fn init_trivial() -> TrivialFractal {
    TrivialFractal {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_it() {}
}
