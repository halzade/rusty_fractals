use rusty_fractals_common::mem::Mem;
use rusty_fractals_core::mathematician::Mathematician;

/**
 * Memory object for Euler fractal
 */
pub struct MemEuler {
    pub m: Mem,
    pub it: u32,
    // TODO separete primes, fibo, etc
    pub math: Mathematician,
    pub spectra: Spectra,
}

impl MemEuler {
    pub fn re(&self) -> f64 {
        self.m.re
    }

    pub fn im(&self) -> f64 {
        self.m.im
    }

    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }

    pub fn euler(&mut self) {
        self.it += 1;
        if self.math.is_prime(&self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }
}

// TODO
pub enum Spectra { Red, Green, Blue }
