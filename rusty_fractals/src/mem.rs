use crate::fractal::MemType;
use crate::mathematician;

// Memory object to carry calculation results
pub struct Mem {
    pub re: f64,
    pub im: f64,
    pub it: u64,
}

impl Mem {
    pub fn plus(&mut self, r: f64, i: f64) {
        self.re += r;
        self.im += i;
    }

    pub fn square(&mut self) {
        let temp = (self.re * self.re) - (self.im * self.im);
        self.im = 2.0 * self.re * self.im;
        self.re = temp;
    }

    pub fn conjugation(&mut self) {
        self.im *= -1.0;
    }

    pub fn inverse(&mut self) {
        let q = self.quad();
        self.conjugation();
        self.re /= q;
        self.im /= q;
    }

    /** (a + ib)^3 */
    pub fn binomial3(&mut self) {
        let temp = (self.re * self.re * self.re) - (3.0 * self.re * self.im * self.im);
        self.im = (3.0 * self.re * self.re * self.im) - (self.im * self.im * self.im);
        self.re = temp;
    }

    /** (a + ib)^4 */
    pub fn binomial4(&mut self) {
        let temp = (self.re * self.re * self.re * self.re)
            - (6.0 * self.re * self.re * self.re * self.im)
            + (self.im * self.re * self.im * self.im);
        self.im = (4.0 * self.re * self.re * self.re * self.im)
            - (4.0 * self.re * self.im * self.im * self.im);
        self.re = temp;
    }

    /** (a + ib)^5 */
    pub fn binomial5(&mut self) {
        let temp = (self.re * self.re * self.re * self.re * self.re)
            - (10.0 * self.re * self.re * self.re * self.im * self.im)
            + (5.0 * self.re * self.im * self.im * self.im * self.im);
        self.im = (5.0 * self.re * self.re * self.re * self.re * self.im)
            - (10.0 * self.re * self.re * self.im * self.im * self.im)
            + (self.im * self.im * self.im * self.im * self.im);
        self.re = temp;
    }

    pub fn circle_inversion(&mut self, re: f64, im: f64) {
        let d = (re * re) + (im * im);
        self.re = re / d;
        self.im = im / d;
    }

    pub fn euler(&mut self) {
        self.it += 1;
        if mathematician::is_prime(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn leonardo(&mut self) {
        self.it += 1;
        if mathematician::is_fibonacci(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn czech(&mut self) {
        self.it += 1;
        if mathematician::is_perfect(self.it) {
            // coordinates switch
            let te = self.re;
            self.re = 0.01 / self.im;
            self.im = 0.01 / te;
        }
    }

    pub fn chess(&mut self) {
        self.it += 1;
        if mathematician::is_square(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn pythagoras(&mut self) {
        self.it += 1;
        if mathematician::is_triangular(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn bee(&mut self) {
        self.it += 1;
        if mathematician::is_pell(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn taco(&mut self) {
        self.it += 1;
        if mathematician::is_lucas(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn manana(&mut self) {
        self.it += 1;
        if mathematician::is_catalan(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn potato(&mut self) {
        self.it += 1;
        if mathematician::is_lazy(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }

    pub fn lukas(&mut self) {
        self.it += 1;
        if mathematician::is_happy(self.it) {
            self.re = 0.01 / self.re;
            self.im = 0.01 / self.im;
        }
    }
}

impl MemType<Mem> for Mem {
    fn new(re: f64, im: f64) -> Mem {
        Mem { re, im, it: 0 }
    }

    fn quad(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    fn re(&self) -> f64 {
        self.re
    }

    fn im(&self) -> f64 {
        self.im
    }
}

#[cfg(test)]
mod tests {
    use crate::fractal::MemType;
    use crate::mathematician;
    use crate::mem::Mem;

    #[test]
    fn test_plus() {
        let mut m = Mem::new(2.0, 3.0);
        m.plus(0.5, 0.4);
        assert_eq!(m.re, 2.5);
        assert_eq!(m.im, 3.4);
    }

    #[test]
    fn test_square() {
        let mut m = Mem::new(3.0, 2.0);
        m.square();
        assert_eq!(m.re, 5.0);
        assert_eq!(m.im, 12.0);
    }

    #[test]
    fn test_quad() {
        let m = Mem::new(3.0, 2.0);
        let q = m.quad();
        assert_eq!(q, 13.0);
    }

    #[test]
    fn test_conjugation() {
        let mut m = Mem::new(3.0, 2.0);
        m.conjugation();
        assert_eq!(m.re, 3.0);
        assert_eq!(m.im, -2.0);
    }

    #[test]
    fn test_inverse() {
        let mut m = Mem::new(0.5, 0.5);

        m.inverse();
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, -1.0);
    }
    #[test]
    fn test_binomial3() {
        let mut m = Mem::new(0.5, 0.5);

        m.binomial3();
        assert_eq!(m.re, -0.25);
        assert_eq!(m.im, 0.25);
    }

    #[test]
    fn test_binomial4() {
        let mut m = Mem::new(0.5, 0.5);

        m.binomial4();
        assert_eq!(m.re, -0.25);
        assert_eq!(m.im, 0.0);
    }

    #[test]
    fn test_binomial5() {
        let mut m = Mem::new(0.5, 0.5);

        m.binomial5();
        assert_eq!(m.re, -0.125);
        assert_eq!(m.im, -0.125);
    }
    #[test]
    fn test_circle_inversion() {
        let mut m = Mem::new(0.5, 0.5);

        m.circle_inversion(0.2, 0.3);
        assert_eq!(m.re, 1.5384615384615385);
        assert_eq!(m.im, 2.3076923076923075);
    }

    #[test]
    fn test_euler() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_primes(2);

        m.euler();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.5);
        assert_eq!(m.im, 0.5);

        m.euler();
        assert_eq!(m.it, 2);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_leonardo() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_fibonacci(2);

        m.leonardo();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_czech() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_perfect(2);

        m.czech();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.5);
        assert_eq!(m.im, 0.5);
    }

    #[test]
    fn test_chess() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_squares(2);

        m.chess();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_pythagoras() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_triangular(2);

        m.pythagoras();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_bee() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_pell(2);

        m.bee();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_taco() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_lucas(2);

        m.taco();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_manana() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_catalan(2);

        m.manana();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_potato() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_lazy(2);

        m.potato();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }

    #[test]
    fn test_lukas() {
        let mut m = Mem::new(0.5, 0.5);
        mathematician::init_happy(2);

        m.lukas();
        assert_eq!(m.it, 1);
        assert_eq!(m.re, 0.02);
        assert_eq!(m.im, 0.02);
    }
}
