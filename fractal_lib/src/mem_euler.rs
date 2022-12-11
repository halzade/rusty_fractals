use crate::mem::Mem;

/**
 * Memory object for Euler fractal
 */
struct MemEuler {
    pub m: Mem,
    pub it: u32,
    spectra: Spectra,
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

    fn euler(&mut self) {
        self.it += 1;
        if Mathematician.isPrime(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }
}

enum Spectra { Red, Green, Blue }
