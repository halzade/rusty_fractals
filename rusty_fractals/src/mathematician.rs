use crate::fractal::MemType;
use crate::mem::Mem;
use fltk::utils::oncelock::Lazy;
use std::collections::HashSet;
use std::sync::RwLock;

struct Mathematician {
    primes: RwLock<HashSet<u64>>,
    fibonacci: RwLock<HashSet<u64>>,
    perfect: RwLock<HashSet<u64>>,
    square: RwLock<HashSet<u64>>,

    triangular: RwLock<HashSet<u64>>,
    pell: RwLock<HashSet<u64>>,
    lucas: RwLock<HashSet<u64>>,
    catalan: RwLock<HashSet<u64>>,
    lazy: RwLock<HashSet<u64>>,
    happy: RwLock<HashSet<u64>>,
}

static MATHEMATICIAN: Lazy<Mathematician> = Lazy::new(|| Mathematician::new());

impl Mathematician {
    fn new() -> Self {
        Mathematician {
            primes: RwLock::new(HashSet::new()),
            fibonacci: RwLock::new(HashSet::new()),
            perfect: RwLock::new(HashSet::new()),
            square: RwLock::new(HashSet::new()),
            triangular: RwLock::new(HashSet::new()),
            pell: RwLock::new(HashSet::new()),
            lucas: RwLock::new(HashSet::new()),
            catalan: RwLock::new(HashSet::new()),
            lazy: RwLock::new(HashSet::new()),
            happy: RwLock::new(HashSet::new()),
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

pub fn is_prime(n: u64) -> bool {
    MATHEMATICIAN.primes.read().unwrap().contains(&n)
}

pub fn is_fibonacci(n: u64) -> bool {
    MATHEMATICIAN.fibonacci.read().unwrap().contains(&n)
}

pub fn is_perfect(n: u64) -> bool {
    MATHEMATICIAN.perfect.read().unwrap().contains(&n)
}

pub fn is_square(n: u64) -> bool {
    MATHEMATICIAN.square.read().unwrap().contains(&n)
}

pub fn is_triangular(n: u64) -> bool {
    MATHEMATICIAN.triangular.read().unwrap().contains(&n)
}

pub fn is_pell(n: u64) -> bool {
    MATHEMATICIAN.pell.read().unwrap().contains(&n)
}

pub fn is_lucas(n: u64) -> bool {
    MATHEMATICIAN.lucas.read().unwrap().contains(&n)
}

pub fn is_catalan(n: u64) -> bool {
    MATHEMATICIAN.catalan.read().unwrap().contains(&n)
}

pub fn is_lazy(n: u64) -> bool {
    MATHEMATICIAN.lazy.read().unwrap().contains(&n)
}

pub fn is_happy(n: u64) -> bool {
    MATHEMATICIAN.happy.read().unwrap().contains(&n)
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

pub fn circle_inversion(m: &mut Mem, re: f64, im: f64) {
    let d = (re * re) + (im * im);
    m.re = re / d;
    m.im = im / d;
}

/**
 * Fibonacci
 */

pub fn init_fibonacci(max: u64) {
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

pub fn init_perfect(max: u64) {
    println!("init_perfect()");
    for i in 1..(max + 1) {
        if is_perfect_init(i) {
            MATHEMATICIAN.perfect.write().unwrap().insert(i);
        }
    }
}

pub fn is_perfect_init(num: u64) -> bool {
    let mut temp = 0;
    let max = (num as f64 / 2.0) as u64;
    for i in 1..(max + 1) {
        if num % i == 0 {
            temp += i;
        }
    }
    temp == num
}

/**
 * Primes
 */

pub fn init_primes(max: u64) {
    println!("init_primes()");
    // smallest prime
    MATHEMATICIAN.primes.write().unwrap().insert(2);
    for i in 3..(max + 1) {
        if is_prime_init(i) {
            MATHEMATICIAN.primes.write().unwrap().insert(i);
        }
    }
}

fn is_prime_init(n: u64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    let investigate_to = f64::sqrt(n as f64) as u64 + 1;
    for i in (3..investigate_to).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/**
 * Squares
 */

pub fn init_squares(max: u64) {
    println!("init_squares()");
    let mut sq;
    let investigate_to = f64::sqrt(max as f64) as u64 + 1;
    for i in 0..investigate_to {
        sq = i * i;
        MATHEMATICIAN.square.write().unwrap().insert(sq);
    }
}

/**
 * Triangular numbers: 1, 3, 6, 10, 15
 */
pub fn init_triangular(max: u64) {
    println!("init_triangular()");
    let mut n = 1;
    while n * (n + 1) / 2 <= max {
        MATHEMATICIAN
            .triangular
            .write()
            .unwrap()
            .insert(n * (n + 1) / 2);
        n += 1;
    }
}

/**
 * Pell numbers: 0, 1, 2, 5, 12, 29,
 */
pub fn init_pell(max: u64) {
    println!("init_pell()");
    let (mut a, mut b) = (0u64, 1u64);
    while a <= max {
        MATHEMATICIAN.pell.write().unwrap().insert(a);
        (a, b) = (b, 2 * b + a);
    }
}

/**
 * Lucas numbers: 2, 1, 3, 4, 7, 11
 */
pub fn init_lucas(max: u64) {
    println!("init_lucas()");
    let (mut a, mut b) = (2u64, 1u64);
    while a <= max {
        MATHEMATICIAN.lucas.write().unwrap().insert(a);
        (a, b) = (b, a + b);
    }
}

/**
 * Catalan numbers: 1, 2, 5, 14, 42, 132,
 */
pub fn init_catalan(max: u64) {
    println!("init_catalan()");
    let mut c: u64 = 1;
    let mut n = 0;
    while c <= max {
        MATHEMATICIAN.catalan.write().unwrap().insert(c);
        n += 1;
        c = c * (4 * n + 2) / (n + 2);
    }
}

/**
 * Happy numbers: 1, 7, 10, 13, 19, 23,
 */
pub fn init_happy(max: u64) {
    println!("init_happy()");
    fn is_happy(mut n: u64) -> bool {
        while n != 1 && n != 4 {
            n = n
                .to_string()
                .chars()
                .map(|c| (c.to_digit(10).unwrap() as u64).pow(2))
                .sum();
        }
        n == 1
    }
    for i in 1..=max {
        if is_happy(i) {
            MATHEMATICIAN.happy.write().unwrap().insert(i);
        }
    }
}

/**
 * Lazy (pancake) numbers: 1, 2, 4, 7, 11, 16,
 */
pub fn init_lazy(max: u64) {
    println!("init_lazy()");
    let mut n = 0;
    loop {
        let val = n * (n + 1) / 2 + 1;
        if val > max {
            break;
        }
        MATHEMATICIAN.lazy.write().unwrap().insert(val);
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::mathematician::{
        init_catalan, init_fibonacci, init_happy, init_lazy, init_lucas, init_pell, init_perfect,
        init_primes, init_squares, init_triangular, is_catalan, is_fibonacci, is_happy, is_lazy,
        is_lucas, is_outside_cardioid, is_outside_circle, is_pell, is_perfect, is_prime, is_square,
        is_triangular,
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
        assert_eq!(is_prime(3), true);
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
        init_perfect(6);
        assert_eq!(is_perfect(1), false);
        assert_eq!(is_perfect(2), false);
        assert_eq!(is_perfect(3), false);
        assert_eq!(is_perfect(4), false);
        assert_eq!(is_perfect(5), false);
        assert_eq!(is_perfect(6), true);
    }

    #[test]
    fn test_is_square() {
        init_squares(4);
        assert_eq!(is_square(1), true);
        assert_eq!(is_square(2), false);
        assert_eq!(is_square(3), false);
        assert_eq!(is_square(4), true);
    }

    #[test]
    pub fn test_is_triangular() {
        init_triangular(6);
        assert_eq!(is_triangular(1), true);
        assert_eq!(is_triangular(2), false);
        assert_eq!(is_triangular(3), true);
        assert_eq!(is_triangular(4), false);
        assert_eq!(is_triangular(5), false);
        assert_eq!(is_triangular(6), true);
    }

    #[test]
    pub fn test_is_pell() {
        init_pell(5);
        assert_eq!(is_pell(1), true);
        assert_eq!(is_pell(2), true);
        assert_eq!(is_pell(3), false);
        assert_eq!(is_pell(4), false);
        assert_eq!(is_pell(5), true);
    }

    #[test]
    pub fn test_is_lucas() {
        init_lucas(7);
        assert_eq!(is_lucas(1), true);
        assert_eq!(is_lucas(2), true);
        assert_eq!(is_lucas(3), true);
        assert_eq!(is_lucas(4), true);
        assert_eq!(is_lucas(5), false);
        assert_eq!(is_lucas(5), false);
        assert_eq!(is_lucas(7), true);
    }

    #[test]
    pub fn test_is_catalan() {
        init_catalan(5);
        assert_eq!(is_catalan(1), true);
        assert_eq!(is_catalan(2), true);
        assert_eq!(is_catalan(3), false);
        assert_eq!(is_catalan(4), false);
        assert_eq!(is_catalan(5), true);
    }

    #[test]
    pub fn test_is_lazy() {
        init_lazy(4);
        assert_eq!(is_lazy(1), true);
        assert_eq!(is_lazy(2), true);
        assert_eq!(is_lazy(3), false);
        assert_eq!(is_lazy(4), true);
    }

    #[test]
    pub fn test_is_happy() {
        init_happy(10);
        assert_eq!(is_happy(1), true);
        assert_eq!(is_happy(2), false);
        assert_eq!(is_happy(3), false);
        assert_eq!(is_happy(4), false);
        assert_eq!(is_happy(5), false);
        assert_eq!(is_happy(6), false);
        assert_eq!(is_happy(7), true);
        assert_eq!(is_happy(8), false);
        assert_eq!(is_happy(9), false);
        assert_eq!(is_happy(10), true);
    }
}
