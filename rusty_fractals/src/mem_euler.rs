use crate::fractal::MemType;
use crate::mathematician;
use crate::mem::Mem;

/**
 * Memory object for Euler fractal
 */
pub struct MemEuler {
    pub m: Mem,
    pub it: u64,
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

    pub fn leonardo(&mut self) {
        self.it += 1;
        if mathematician::is_fibonacci(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn czech(&mut self) {
        self.it += 1;
        if mathematician::is_perfect(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn chess(&mut self) {
        self.it += 1;
        if mathematician::is_square(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn pythagoras(&mut self) {
        self.it += 1;
        if mathematician::is_triangular(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn bee(&mut self) {
        self.it += 1;
        if mathematician::is_pell(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn taco(&mut self) {
        self.it += 1;
        if mathematician::is_lucas(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn manana(&mut self) {
        self.it += 1;
        if mathematician::is_catalan(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn potato(&mut self) {
        self.it += 1;
        if mathematician::is_lazy(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }

    pub fn lukas(&mut self) {
        self.it += 1;
        if mathematician::is_happy(self.it) {
            self.m.re = 0.01 / self.m.re;
            self.m.im = 0.01 / self.m.im;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fractal::MemType;
    use crate::mathematician;
    use crate::mem_euler::MemEuler;

    #[test]
    fn test_plus() {
        let mut me = MemEuler::new(0.0, 0.0);

        me.plus(1.0, 1.1);

        assert_eq!(me.m.re, 1.0);
        assert_eq!(me.m.im, 1.1);
        assert_eq!(me.it, 0);
    }

    #[test]
    fn test_euler() {
        let mut me = MemEuler::new(0.5, 0.5);
        mathematician::init_primes(2);

        me.euler();
        assert_eq!(me.it, 1);
        assert_eq!(me.m.re, 0.5);
        assert_eq!(me.m.im, 0.5);

        me.euler();
        assert_eq!(me.it, 2);
        assert_eq!(me.m.re, 0.02);
        assert_eq!(me.m.im, 0.02);
    }
}
