use crate::mem::Mem;

pub struct MemCollatz {
    pub m: Mem,
    pub it: i32,
}

impl MemCollatz {
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

    pub fn collatz_conjecture(&mut self) {
        if self.it % 2 == 1 {
            self.m.re = 3.0 * self.m.re + 1.0;
            self.m.im = 3.0 * self.m.im + 1.0;
        } else {
            self.m.re = self.m.re / 2.0;
            self.m.im = self.m.im / 2.0;
        }
        self.it += 1;
    }

    pub fn plus_collatz(&mut self, r: f64, i: f64) {
        self.m.re += (3.0 * r + 1.0) / 2.0;
        self.m.im += (3.0 * i + 1.0) / 2.0;
    }

    pub fn quad(&mut self) -> f64 {
        self.m.quad()
    }
}

pub fn new(re: f64, im: f64) -> MemCollatz {
    MemCollatz { m: Mem { re, im }, it: 0 }
}

#[cfg(test)]
mod tests {
    use crate::mem::Mem;
    use crate::mem_collatz::MemCollatz;

    #[test]
    fn test_collatz_conjecture() {
        let mut c = MemCollatz { m: Mem { re: 0.0, im: 1.0 }, it: 1 };
        c.collatz_conjecture();
        assert_eq!(c.m.re, 1.0);
        assert_eq!(c.m.im, 4.0);
        c.collatz_conjecture();
        assert_eq!(c.m.re, 0.5, "2.nd test for re");
        assert_eq!(c.m.im, 2.0, "2.nd test for im");
    }

    #[test]
    fn test_plus_collatz() {
        let mut c = MemCollatz { m: Mem { re: 0.0, im: 1.0 }, it: 1 };
        c.plus_collatz(0.0, 0.0);
        assert_eq!(c.m.re, 0.5);
        assert_eq!(c.m.im, 1.5);
    }
}
