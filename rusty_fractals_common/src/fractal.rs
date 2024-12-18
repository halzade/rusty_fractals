use crate::area::{Area, AreaConfig};
use crate::calc::{CalculationConfig, OrbitType};
use crate::constants::CALCULATION_BOUNDARY;
use crate::data_image::DataImage;
use crate::mem::Mem;
use crate::palette::Palette;
use crate::resolution_multiplier::ResolutionMultiplier;
use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::thread;

pub struct Fractal {}

pub struct FractalConfig<'lt> {
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub resolution_multiplier: ResolutionMultiplier,
    pub palette: Palette<'lt>,
    pub phantom: PhantomData<&'lt bool>,
}

pub struct MandelbrotConfig<'lt> {
    pub iteration_max: u32,
    // color classic mandelbrot values
    pub palette: Palette<'lt>,
    // color insides of mandelbrot set
    pub palette_zero: Palette<'lt>,
    pub phantom: PhantomData<&'lt bool>,
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

pub fn calculate_fractal_new_thread<'lt, M: MemType<M>>(
    fractal: &dyn FractalMath<M>,
    fractal_config: FractalConfig,
    area_config: AreaConfig,
    calc_config: CalculationConfig,
) {
    let engine = thread::spawn(move || {
        // TODO calculate_fractal(fractal_config, area_config, calc_config);
    });
}

pub fn calculate_mandelbrot_new_thread<'lt, M: MemType<M>>(
    fractal: &dyn FractalMath<M>,
    fractal_config: FractalConfig,
    area_config: AreaConfig,
    calc_config: CalculationConfig,
) {
    thread::spawn(move || {
        // TODO fractal.calculate_mandelbrot();
    });
}

pub fn path_test(
    calculation_config: CalculationConfig,
    min: u32,
    max: u32,
    length: u32,
    iterator: u32,
) -> bool {
    if calculation_config.orbits == OrbitType::Finite {
        // only the edges of mandelbrot set
        length > min && iterator < max
    } else {
        // also contains the inside of mandelbrot set
        length > min && iterator == max
    }
}

pub fn calculate_path<'lt, T: MemType<T>>(
    fractal: &impl FractalMath<T>,
    area: &Area,
    iteration_min: u32,
    iteration_max: u32,
    origin_re: f64,
    origin_im: f64,
    data_image: &DataImage,
    calc_config: CalculationConfig,
    is_wrap: bool,
) -> (u32, u32) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut m: T = T::new(origin_re, origin_im);
    let mut iterator = 0;
    let mut length = 0;
    while m.quad() < cb && iterator < iteration_max {
        // Investigate if this is a good calculation path
        // Don't create path data yet. Too many origins don't produce good data
        // Most of the long and expensive calculations end up inside Mandelbrot set, useless
        // It is 1.68x faster to calculate path twice, and to record exclusively the good paths
        fractal.math(&mut m, origin_re, origin_im);
        if area.contains(m.re(), m.im()) {
            // this becomes important for zoom, when only a small amount
            // of calculation path elements is contained withing tiny area
            length += 1;
        }
        iterator += 1;
    }

    if path_test(calc_config, iteration_min, iteration_max, length, iterator) {
        // This origin produced good data
        // Record the calculation path
        let mut m: T = T::new(origin_re, origin_im);
        let mut path: Vec<[f64; 2]> = Vec::new();
        for _ in 0..iterator {
            fractal.math(&mut m, origin_re, origin_im);
            if area.contains(m.re(), m.im()) {
                path.push([m.re(), m.im()]);
            }
        }

        // if iteration_max increased, ignore possible extension of previous calculation paths
        // path elements are going to migrate out of the screen shortly
        // removed last_iteration, last_visited_re, last_visited_im
        if data_image.is_dynamic() {
            data_image.save_path(path, is_wrap);
        } else {
            data_image.translate_path_to_point_grid(path, area, is_wrap);
        }
        // TODO stats.paths_new_points_amount += path.size();
    }
    (iterator, length)
}

pub fn calculate_mandelbrot_path<T: MemType<T>>(
    fractal_math: &impl FractalMath<T>,
    iteration_max: u32,
    origin_re: f64,
    origin_im: f64,
) -> (u32, f64) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut m: T = T::new(origin_re, origin_im);
    let mut iterator = 0;
    while m.quad() < cb && iterator < iteration_max {
        fractal_math.math(&mut m, origin_re, origin_im);
        iterator += 1;
    }
    (iterator, m.quad())
}

#[cfg(test)]
mod tests {
    use crate::fractal::calculate_path;
    use crate::{area, calc, data_image, fractal};

    #[test]
    fn test_calculate_path() {
        // prepare test data
        let area = area::init_trivial();
        let data_image = data_image::init_trivial();
        let fractal = fractal::init_trivial();
        let calc_config = calc::init_trivial();

        // execute test
        calculate_path(
            &fractal,
            &area,
            1,
            5,
            0.0,
            0.0,
            &data_image,
            calc_config,
            false,
        );
    }
}
