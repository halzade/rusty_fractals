use crate::fractal::MemType;
use crate::mem::Mem;

pub struct MemCollatz {
    pub m: Mem,
    pub num: i128,
}

impl MemCollatz {
    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }

    pub fn collatz_conjecture(&mut self) {
        // all other math methods ignore num
        if (self.num % 2) == 0 {
            self.num /= 2;
            self.m.re = self.m.re / 2.0;
            self.m.im = self.m.im / 2.0;
        } else {
            self.num = 3 * self.num + 1;
            self.m.re = 3.0 * self.m.re + 1.0;
            self.m.im = 3.0 * self.m.im + 1.0;
        }
    }

    pub fn plus_collatz(&mut self, r: f64, i: f64) {
        self.m.re += (3.0 * r + 1.0) / 2.0;
        self.m.im += (3.0 * i + 1.0) / 2.0;
    }
}

impl MemType<MemCollatz> for MemCollatz {
    fn new(re: f64, im: f64) -> MemCollatz {
        MemCollatz {
            m: Mem { re, im },
            // 1348 steps
            num: 989_345_275_647,

            // TODO
            // 1563 steps
            // num: 7_887_663_552_367,

            // 2456 steps
            // num: 28_019_077_177_231_758_495,
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

#[cfg(test)]
mod tests {
    use crate::mem::Mem;
    use crate::mem_collatz::MemCollatz;

    #[test]
    fn test_collatz_conjecture() {
        let mut c = MemCollatz {
            m: Mem { re: 0.0, im: 1.0 },
            num: 1,
        };
        c.collatz_conjecture();
        assert_eq!(c.m.re, 1.0);
        assert_eq!(c.m.im, 4.0);
        c.collatz_conjecture();
        assert_eq!(c.m.re, 0.5, "2.nd test for re");
        assert_eq!(c.m.im, 2.0, "2.nd test for im");
    }

    #[test]
    fn test_plus_collatz() {
        let mut c = MemCollatz {
            m: Mem { re: 0.0, im: 1.0 },
            num: 1,
        };
        c.plus_collatz(0.0, 0.0);
        assert_eq!(c.m.re, 0.5);
        assert_eq!(c.m.im, 1.5);
    }
}
