use rusty_fractals::fractal::MemType;
use rusty_fractals::mem::Mem;
use rusty_fractals::mathematician::Mathematician;

/**
 * Memory object for Euler fractal
 */
pub struct MemEuler {
    pub m: Mem,
    pub it: u32,
    // TODO separate primes, fibo, etc
    pub math: Mathematician,
    pub spectra: Spectra,
}

impl MemType<MemEuler> for MemEuler {
    fn new(re: f64, im: f64) -> MemEuler {
        MemEuler {
            m: Mem { re, im },
            it: 0,
            math: Mathematician {
                // TODO
                primes: Default::default(),
                fibonacci: Default::default(),
                perfect: Default::default(),
                square: Default::default(),
            },
            // TODO
            spectra: Spectra::Red,
        }
    }
    fn quad(&self) -> f64 {
        self.m.quad()
    }
    fn re(&self) -> f64 {
        self.m.re
    }
    fn im(&self) -> f64 {
        self.m.im
    }
}

impl MemEuler {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
