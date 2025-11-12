use rusty_fractals::fractal::MemType;
use rusty_fractals::mathematician;
use rusty_fractals::mem::Mem;

/**
 * Memory object for Euler fractal
 */
pub struct MemEuler {
    pub m: Mem,
    pub it: u32,
}

impl MemType<MemEuler> for MemEuler {
    fn new(re: f64, im: f64) -> MemEuler {
        MemEuler {
            m: Mem::new(re, im),
            it: 0,
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
        if mathematician::is_prime(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }
}

pub enum Spectra {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use crate::mem_euler::MemEuler;
    use rusty_fractals::fractal::MemType;

    #[test]
    fn test_plus() {
        let mut me = MemEuler::new(0.0, 0.0);

        me.plus(1.0, 1.1);

        assert_eq!(me.m.re, 1.0);
        assert_eq!(me.m.im, 1.1);
        assert_eq!(me.it, 0);
    }
}
