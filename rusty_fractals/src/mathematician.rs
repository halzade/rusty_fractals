use crate::fractal::MemType;
use crate::mem::Mem;
use fltk::utils::oncelock::Lazy;
use std::collections::HashSet;
use std::sync::RwLock;

struct Mathematician {
    primes: RwLock<HashSet<u32>>,
    fibonacci: RwLock<HashSet<u32>>,
    perfect: RwLock<HashSet<u32>>,
    square: RwLock<HashSet<u32>>,
}

static MATHEMATICIAN: Lazy<Mathematician> = Lazy::new(|| Mathematician::new());

impl Mathematician {
    fn new() -> Self {
        Mathematician {
            primes: RwLock::new(HashSet::new()),
            fibonacci: RwLock::new(HashSet::new()),
            perfect: RwLock::new(HashSet::new()),
            square: RwLock::new(HashSet::new()),
        }
    }
}

/**
 * (t^2 + x^2 - 2at)^2 = 4a^2 (t^2 + x^2)
 */
pub fn is_outside_cardioid(re: f64, im: f64) -> bool {
    // precise value a = 0.25
    let a = 0.24;
    let t = re - 0.24;
    let t2 = t * t;
    let x2 = im * im;
    let left_side = t2 + x2 + 2.0 * a * t;
    left_side * left_side > 4.0 * a * a * (t2 + x2)
}

/**
 * circle with center at re=-1,im=0 and radius 1/4
 */
pub fn is_outside_circle(re: f64, im: f64) -> bool {
    // precise diameter squared is 0.0625
    ((re + 1.0) * (re + 1.0)) + (im * im) > 0.062
}

pub fn rotate_by(m: &mut Mem, t: f64) {
    let temp = (1.0 - t * t) / (1.0 + t * t);
    m.im = (2.0 * t) / (1.0 + t * t);
    m.re = temp;
}

pub fn is_prime(n: u32) -> bool {
    MATHEMATICIAN.primes.read().unwrap().contains(&n)
}

pub fn is_fibonacci(n: u32) -> bool {
    MATHEMATICIAN.fibonacci.read().unwrap().contains(&n)
}

pub fn is_perfect(n: u32) -> bool {
    MATHEMATICIAN.perfect.read().unwrap().contains(&n)
}

pub fn is_square(n: u32) -> bool {
    MATHEMATICIAN.square.read().unwrap().contains(&n)
}

pub fn multiply_by(m: &mut Mem, re: f64, im: f64) {
    let temp = (m.re * re) - (m.im * im);
    m.im = (m.re * im) + (re * m.im);
    m.re = temp;
}

pub fn plus_invert(m: &mut Mem) {
    let a = m.re;
    let b = m.im;
    let quad = (a * a) + (b * b);
    m.re = m.re + (a / quad);
    m.im = m.im - (b / quad);
}

pub fn minus_invert(m: &mut Mem) {
    let a = m.re;
    let b = m.im;
    let quad = (a * a) + (b * b);
    m.re = m.re - (a / quad);
    m.im = m.im + (b / quad);
}

pub fn inner_product(m: &mut Mem, re: f64, im: f64) {
    m.re = m.re * re;
    m.im = m.im * im;
}

pub fn inverse(m: &mut Mem) {
    let q = m.quad();
    m.conjugation();
    m.re /= q;
    m.im /= q;
}

/** (a + ib)^3 */
pub fn binomial3(m: &mut Mem) {
    let temp = (m.re * m.re * m.re) - (3.0 * m.re * m.im * m.im);
    m.im = (3.0 * m.re * m.re * m.im) - (m.im * m.im * m.im);
    m.re = temp;
}

/** (a + ib)^4 */
pub fn binomial4(m: &mut Mem) {
    let temp = (m.re * m.re * m.re * m.re) - (6.0 * m.re * m.re * m.re * m.im)
        + (m.im * m.re * m.im * m.im);
    m.im = (4.0 * m.re * m.re * m.re * m.im) - (4.0 * m.re * m.im * m.im * m.im);
    m.re = temp;
}

/** (a + ib)^5 */
pub fn binomial5(m: &mut Mem) {
    let temp = (m.re * m.re * m.re * m.re * m.re) - (10.0 * m.re * m.re * m.re * m.im * m.im)
        + (5.0 * m.re * m.im * m.im * m.im * m.im);
    m.im = (5.0 * m.re * m.re * m.re * m.re * m.im) - (10.0 * m.re * m.re * m.im * m.im * m.im)
        + (m.im * m.im * m.im * m.im * m.im);
    m.re = temp;
}

pub fn reciprocal(m: &mut Mem) {
    let scale = m.re * m.re + m.im * m.im;
    m.re = m.re / scale;
    m.im = -m.im / scale;
}

pub fn circle_inversion(m: &mut Mem, re: f64, im: f64) {
    let d = (re * re) + (im * im);
    m.re = re / d;
    m.im = im / d;
}

/**
 * Fibonacci
 */

pub fn init_fibonacci(max: u32) {
    println!("init_fibonacci()");
    let mut a = 0;
    let mut b = 1;
    let mut sum;
    while b <= max {
        sum = a + b;
        MATHEMATICIAN.fibonacci.write().unwrap().insert(sum);
        a = b;
        b = sum;
    }
}

/**
 * Perfect
 */

pub fn init_perfect_numbers(max: u32) {
    println!("init_perfect_numbers()");
    for i in 1..(max + 1) {
        if is_perfect_init(i) {
            MATHEMATICIAN.perfect.write().unwrap().insert(i);
        }
    }
}

/**
 * Primes
 */

pub fn init_primes(max: u32) {
    println!("init_primes()");
    // smallest prime
    MATHEMATICIAN.primes.write().unwrap().insert(2);
    for i in 3..(max + 1) {
        if is_prime_init(i) {
            MATHEMATICIAN.primes.write().unwrap().insert(i);
        }
    }
}

/**
 * Squares
 */

pub fn init_squares(max: u32) {
    println!("init_squares()");
    let mut sq;
    let investigate_to = f64::sqrt(max as f64) as u32 + 1;
    for i in 0..investigate_to {
        sq = i * i;
        MATHEMATICIAN.square.write().unwrap().insert(sq);
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
    use crate::mathematician::{
        init_fibonacci, init_perfect_numbers, init_primes, init_squares, is_fibonacci,
        is_outside_cardioid, is_outside_circle, is_perfect, is_prime, is_square,
    };

    #[test]
    fn test_is_outside_cardioid() {
        assert_eq!(is_outside_cardioid(0.0, 0.0), false);
        assert_eq!(is_outside_cardioid(2.0, 1.0), true);
    }

    #[test]
    fn test_is_outside_circle() {
        assert_eq!(is_outside_circle(-1.0, 0.0), false);
        assert_eq!(is_outside_circle(2.0, 1.0), true);
    }

    #[test]
    fn test_is_prime() {
        init_primes(5);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_fibonacci() {
        init_fibonacci(5);
        assert_eq!(is_fibonacci(1), true);
        assert_eq!(is_fibonacci(2), true);
        assert_eq!(is_fibonacci(3), true);
        assert_eq!(is_fibonacci(4), false);
        assert_eq!(is_fibonacci(5), true);
    }

    #[test]
    fn test_is_perfect() {
        init_perfect_numbers(6);
        assert_eq!(is_perfect(5), false);
        assert_eq!(is_perfect(6), true);
    }

    #[test]
    fn test_is_square() {
        init_squares(4);
        assert_eq!(is_square(3), false);
        assert_eq!(is_square(4), true);
    }
}
