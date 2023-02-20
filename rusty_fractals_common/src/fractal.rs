use crate::area::Area;
use crate::constants::CALCULATION_BOUNDARY;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::{mem, mem_collatz, mem_phoenix};
use crate::mem::Mem;
use crate::mem_collatz::MemCollatz;
use crate::mem_phoenix::MemPhoenix;
use crate::result_data_static::ResultDataStatic;

pub struct CalculationConfig {
    pub iteration_min: u32,
    pub iteration_max: u32,
    pub resolution_multiplier: ResolutionMultiplier,
}

pub struct AppConfig {
    pub repeat: bool,
    pub save_images: bool,
}

pub trait Fractal: Sync {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool;
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32);
}

pub trait MathMem: Sync {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64);
}

pub trait MathPhoenix: Sync {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64);
}

pub trait MathCollatz: Sync {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64);
}


pub fn finite_orbits(min: u32, max: u32, length: u32, iterator: u32) -> bool {
    length > min && iterator < max
}

pub fn infinite_orbits(min: u32, max: u32, length: u32, iterator: u32) -> bool {
    length > min && iterator == max
}


pub fn calculate_path_mem(fractal: &impl Fractal, fractal_mem: &impl MathMem, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut m: Mem = mem::new(origin_re, origin_im);
    let mut iterator = 0;
    let mut length = 0;
    while m.quad() < cb && iterator < iteration_max {
        // Investigate if this is a good calculation path
        // Don't create path data yet. Too many origins don't produce good data
        // Most of the long and expensive calculations end up inside Mandelbrot set, useless
        // It is 1.68x faster to calculate path twice, and to record exclusively the good paths
        fractal_mem.math(&mut m, origin_re, origin_im);
        if area.contains(m.re, m.im) {
            // this becomes important for zoom, when only a small amount
            // of calculation path elements is contained withing tiny area
            length += 1;
        }
        iterator += 1;
    }
    if fractal.path_test(iteration_min, iteration_max, length, iterator) {
        // This origin produced good data
        // Record the calculation path
        let mut m = mem::new(origin_re, origin_im);
        let mut path: Vec<[f64; 2]> = Vec::new();
        for _ in 0..iterator {
            fractal_mem.math(&mut m, origin_re, origin_im);
            if area.contains(m.re, m.im) {
                path.push([m.re, m.im]);
            }
        }
        result_static.translate_path_to_point_grid(path, area);
        // TODO stats.paths_new_points_amount += path.size();
    }
    (iterator, length)
}

pub fn calculate_path_phoenix(fractal: &impl Fractal, fractal_phoenix: &impl MathPhoenix, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut mp: MemPhoenix = mem_phoenix::new(origin_re, origin_im);
    let mut iterator = 0;
    let mut length = 0;
    while mp.quad() < cb && iterator < iteration_max {
        fractal_phoenix.math(&mut mp, origin_re, origin_im);
        if area.contains(mp.re(), mp.im()) {
            length += 1;
        }
        iterator += 1;
    }
    if fractal.path_test(iteration_min, iteration_max, length, iterator) {
        let mut mp = mem_phoenix::new(origin_re, origin_im);
        let mut path: Vec<[f64; 2]> = Vec::new();
        for _ in 0..iterator {
            fractal_phoenix.math(&mut mp, origin_re, origin_im);
            if area.contains(mp.re(), mp.im()) {
                path.push([mp.re(), mp.im()]);
            }
        }
        result_static.translate_path_to_point_grid(path, area);
    }
    (iterator, length)
}

pub fn calculate_path_collatz(fractal: &impl Fractal, fractal_collatz: &impl MathCollatz, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut mc: MemCollatz = mem_collatz::new(origin_re, origin_im);
    let mut iterator = 0;
    let mut length = 0;
    while mc.quad() < cb && iterator < iteration_max {
        fractal_collatz.math(&mut mc, origin_re, origin_im);
        if area.contains(mc.re(), mc.im()) {
            length += 1;
        }
        iterator += 1;
    }
    if fractal.path_test(iteration_min, iteration_max, length, iterator) {
        let mut mc = mem_collatz::new(origin_re, origin_im);
        let mut path: Vec<[f64; 2]> = Vec::new();
        for _ in 0..iterator {
            fractal_collatz.math(&mut mc, origin_re, origin_im);
            if area.contains(mc.re(), mc.im()) {
                path.push([mc.re(), mc.im()]);
            }
        }
        result_static.translate_path_to_point_grid(path, area);
    }
    (iterator, length)
}

pub fn calculate_iterations_mandelbrot(fractal: &impl Fractal, fractal_collatz: &impl MathCollatz, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, f64) {
    let cb = CALCULATION_BOUNDARY as f64;
    let mut mc: MemCollatz = mem_collatz::new(origin_re, origin_im);
    let mut iterator = 0;
    while mc.quad() < cb && iterator < iteration_max {
        fractal_collatz.math(&mut mc, origin_re, origin_im);
        iterator += 1;
    }
    (iterator, mc.quad())
}

/*
pub fn update(mut stats: Stats) {
    // TODO ITERATION_MAX += 150;

    stats.update(0); // TODO

    if stats.not_enough_pixels_best_value {
        // ("increase ITERATION_MAX, not enough Points");
        // TODO ITERATION_MAX += 20_000;
    }
    if stats.less_pixels_best_value {
        // TODO ITERATION_MAX += 2_000;
        // ("increase ITERATION_MAX, bit less Points");
    }
    if stats.too_many_paths_total {
        // ("increase a bit ITERATION_MIN, too many paths total");
        // TODO ITERATION_min += 1;
    }

    stats.print();
    stats.clean();
}
*/
