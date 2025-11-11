use crate::fractal::MemType;
use crate::mem::Mem;
use std::collections::HashSet;

pub struct Mathematician {
    pub primes: HashSet<u32>,
    pub fibonacci: HashSet<u32>,
    pub perfect: HashSet<u32>,
    pub square: HashSet<u32>,
}

impl Mathematician {
    pub fn new() -> Self {
        Mathematician {
            primes: HashSet::new(),
            fibonacci: HashSet::new(),
            perfect: HashSet::new(),
            square: HashSet::new(),
        }
    }

    /**
     * (t^2 + x^2 - 2at)^2 = 4a^2 (t^2 + x^2)
     */
    pub fn is_outside_cardioid(re: f64, im: f64) -> bool {
        let a = 0.25;
        let t = re - 0.25;
        let t2 = t * t;
        let x2 = im * im;
        let left_side = t2 + x2 + 2.0 * a * t;
        left_side * left_side > 4.0 * a * a * (t2 + x2)
    }

    /**
     * circle with center at re=-1,im=0 and radius 1/4
     */
    pub fn is_outside_circle(re: f64, im: f64) -> bool {
        ((re + 1.0) * (re + 1.0)) + (im * im) > 0.0625
    }

    pub fn rotate_by(mut m: Mem, t: f64) {
        let temp = (1.0 - t * t) / (1.0 + t * t);
        m.im = (2.0 * t) / (1.0 + t * t);
        m.re = temp;
    }

    pub fn is_prime(&self, n: u32) -> bool {
        self.primes.contains(&n)
    }

    pub fn is_fibonacci(&self, n: u32) -> bool {
        self.fibonacci.contains(&n)
    }

    pub fn is_perfect(&self, n: u32) -> bool {
        self.perfect.contains(&n)
    }

    pub fn is_square(&self, n: u32) -> bool {
        self.square.contains(&n)
    }

    pub fn multiply_by(mut m: Mem, re: f64, im: f64) {
        let temp = (m.re * re) - (m.im * im);
        m.im = (m.re * im) + (re * m.im);
        m.re = temp;
    }

    pub fn plus_invert(mut m: Mem) {
        let a = m.re;
        let b = m.im;
        let quad = (a * a) + (b * b);
        m.re = m.re + (a / quad);
        m.im = m.im - (b / quad);
    }

    pub fn minus_invert(mut m: Mem) {
        let a = m.re;
        let b = m.im;
        let quad = (a * a) + (b * b);
        m.re = m.re - (a / quad);
        m.im = m.im + (b / quad);
    }

    pub fn inner_product(mut m: Mem, re: f64, im: f64) {
        m.re = m.re * re;
        m.im = m.im * im;
    }

    pub fn inverse(mut m: Mem) {
        let q = m.quad();
        m.conjugation();
        m.re /= q;
        m.im /= q;
    }

    /** (a + ib)^3 */
    pub fn binomial3(mut m: Mem) {
        let temp = (m.re * m.re * m.re) - (3.0 * m.re * m.im * m.im);
        m.im = (3.0 * m.re * m.re * m.im) - (m.im * m.im * m.im);
        m.re = temp;
    }

    /** (a + ib)^4 */
    pub fn binomial4(mut m: Mem) {
        let temp = (m.re * m.re * m.re * m.re) - (6.0 * m.re * m.re * m.re * m.im)
            + (m.im * m.re * m.im * m.im);
        m.im = (4.0 * m.re * m.re * m.re * m.im) - (4.0 * m.re * m.im * m.im * m.im);
        m.re = temp;
    }

    /** (a + ib)^5 */
    pub fn binomial5(mut m: Mem) {
        let temp = (m.re * m.re * m.re * m.re * m.re) - (10.0 * m.re * m.re * m.re * m.im * m.im)
            + (5.0 * m.re * m.im * m.im * m.im * m.im);
        m.im = (5.0 * m.re * m.re * m.re * m.re * m.im) - (10.0 * m.re * m.re * m.im * m.im * m.im)
            + (m.im * m.im * m.im * m.im * m.im);
        m.re = temp;
    }

    pub fn reciprocal(mut m: Mem) {
        let scale = m.re * m.re + m.im * m.im;
        m.re = m.re / scale;
        m.im = -m.im / scale;
    }

    pub fn circle_inversion(mut m: Mem, re: f64, im: f64) {
        let d = (re * re) + (im * im);
        m.re = re / d;
        m.im = im / d;
    }

    /**
     * Fibonacci
     */

    pub fn init_fibonacci(&mut self, max: u32) {
        println!("init_fibonacci()");
        let mut a = 0;
        let mut b = 1;
        let mut sum;
        while b <= max {
            sum = a + b;
            self.fibonacci.insert(sum);
            a = b;
            b = sum;
        }
    }

    /**
     * Perfect
     */

    pub fn init_perfect_numbers(&mut self, max: u32) {
        println!("init_perfect_numbers()");
        for i in 1..(max + 1) {
            if is_perfect_init(i) {
                self.perfect.insert(i);
            }
        }
    }

    /**
     * Primes
     */

    pub fn init_primes(&mut self, max: u32) {
        println!("init_primes()");
        for i in 1..(max + 1) {
            if is_prime_init(i) {
                self.primes.insert(i);
            }
        }
    }

    /**
     * Squares
     */

    pub fn init_squares(&mut self, max: u32) {
        println!("init_squares()");
        let mut sq;
        let investigate_to = f64::sqrt(max as f64) as u32 + 1;
        for i in 0..investigate_to {
            sq = i * i;
            self.square.insert(sq);
        }
    }
}

pub fn is_perfect_init(num: u32) -> bool {
    let mut temp = 0;
    let max = (num as f64 / 2.0) as u32;
    for i in 1..(max + 1) {
        if num % i == 0 {
            temp += i;
        }
    }
    temp == num
}

fn is_prime_init(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let investigate_to = f64::sqrt(n as f64) as u32 + 1;
    for i in (3..investigate_to).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::mathematician::Mathematician;

    #[test]
    fn test_is_outside_cardioid() {
        assert_eq!(Mathematician::is_outside_cardioid(0.0, 0.0), false);
        assert_eq!(Mathematician::is_outside_cardioid(2.0, 1.0), true);
    }

    #[test]
    fn test_is_outside_circle() {
        assert_eq!(Mathematician::is_outside_circle(-1.0, 0.0), false);
        assert_eq!(Mathematician::is_outside_circle(2.0, 1.0), true);
    }

    #[test]
    fn test_is_prime() {
        let mut m: Mathematician = Mathematician::new();
        m.init_primes(5);
        assert_eq!(m.is_prime(1), true);
        assert_eq!(m.is_prime(2), true);
        assert_eq!(m.is_prime(4), false);
        assert_eq!(m.is_prime(5), true);
    }

    #[test]
    fn test_is_fibonacci() {
        let mut m: Mathematician = Mathematician::new();
        m.init_fibonacci(5);
        assert_eq!(m.is_fibonacci(1), true);
        assert_eq!(m.is_fibonacci(2), true);
        assert_eq!(m.is_fibonacci(3), true);
        assert_eq!(m.is_fibonacci(4), false);
        assert_eq!(m.is_fibonacci(5), true);
    }

    #[test]
    fn test_is_perfect() {
        let mut m: Mathematician = Mathematician::new();
        m.init_perfect_numbers(6);
        assert_eq!(m.is_perfect(5), false);
        assert_eq!(m.is_perfect(6), true);
    }

    #[test]
    fn test_is_square() {
        let mut m: Mathematician = Mathematician::new();
        m.init_squares(4);
        assert_eq!(m.is_square(3), false);
        assert_eq!(m.is_square(4), true);
    }
}
