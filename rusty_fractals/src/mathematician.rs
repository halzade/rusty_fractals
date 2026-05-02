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
    lucas: RwLock<HashSet<u64>>,
    lazy: RwLock<HashSet<u64>>,
    happy: RwLock<HashSet<u64>>,
}

static MATHEMATICIAN: Lazy<Mathematician> = Lazy::new(Mathematician::new);

impl Mathematician {
    fn new() -> Self {
        Self {
            primes: RwLock::new(HashSet::new()),
            fibonacci: RwLock::new(HashSet::new()),
            perfect: RwLock::new(HashSet::new()),
            square: RwLock::new(HashSet::new()),
            triangular: RwLock::new(HashSet::new()),
            lucas: RwLock::new(HashSet::new()),
            lazy: RwLock::new(HashSet::new()),
            happy: RwLock::new(HashSet::new()),
        }
    }
}

/**
 * (t^2 + x^2 - 2at)^2 = 4a^2 (t^2 + x^2)
 */
pub const fn is_outside_cardioid(re: f64, im: f64) -> bool {
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
pub const fn is_outside_circle(re: f64, im: f64) -> bool {
    // precise diameter squared is 0.0625
    ((re + 1.0) * (re + 1.0)) + (im * im) > 0.062
}

pub const fn is_outside_top_circle(re: f64, im: f64) -> bool {
    let cx = -0.122561166876;
    let cy = 0.744861766619;
    let r2 = 0.00909;

    let dx = re - cx;
    let dy = im - cy;

    dx * dx + dy * dy > r2
}

pub const fn is_outside_bottom_circle(re: f64, im: f64) -> bool {
    let cx = -0.122561166876;
    let cy = -0.744861766619;
    let r2 = 0.00909;

    let dx = re - cx;
    let dy = im - cy;

    dx * dx + dy * dy > r2
}

pub fn rotate_by(m: &mut Mem, t: f64) {
    let t2 = t * t;
    let common = 1.0 + t2;
    let temp = t.mul_add(-t, 1.0) / common;
    m.im = (2.0 * t) / common;
    m.re = temp;
}

pub fn is_prime(n: u64) -> bool {
    MATHEMATICIAN.primes.read().map(|p| p.contains(&n)).unwrap_or(false)
}

pub fn is_fibonacci(n: u64) -> bool {
    MATHEMATICIAN.fibonacci.read().map(|f| f.contains(&n)).unwrap_or(false)
}

pub fn is_perfect(n: u64) -> bool {
    MATHEMATICIAN.perfect.read().map(|p| p.contains(&n)).unwrap_or(false)
}

pub fn is_square(n: u64) -> bool {
    MATHEMATICIAN.square.read().map(|s| s.contains(&n)).unwrap_or(false)
}

pub fn is_triangular(n: u64) -> bool {
    MATHEMATICIAN.triangular.read().map(|t| t.contains(&n)).unwrap_or(false)
}

pub fn is_lucas(n: u64) -> bool {
    MATHEMATICIAN.lucas.read().map(|l| l.contains(&n)).unwrap_or(false)
}

pub fn is_lazy(n: u64) -> bool {
    MATHEMATICIAN.lazy.read().map(|l| l.contains(&n)).unwrap_or(false)
}

pub fn is_happy(n: u64) -> bool {
    MATHEMATICIAN.happy.read().map(|h| h.contains(&n)).unwrap_or(false)
}

pub fn multiply_by(m: &mut Mem, re: f64, im: f64) {
    let temp = m.re.mul_add(re, -(m.im * im));
    m.im = m.re.mul_add(im, re * m.im);
    m.re = temp;
}

pub fn plus_invert(m: &mut Mem) {
    let a = m.re;
    let b = m.im;
    let quad = a.mul_add(a, b * b);
    m.re += a / quad;
    m.im -= b / quad;
}

pub fn minus_invert(m: &mut Mem) {
    let a = m.re;
    let b = m.im;
    let quad = a.mul_add(a, b * b);
    m.re -= a / quad;
    m.im += b / quad;
}

pub fn inner_product(m: &mut Mem, re: f64, im: f64) {
    m.re *= re;
    m.im *= im;
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
        if let Ok(mut fib) = MATHEMATICIAN.fibonacci.write() {
            fib.insert(sum);
        }
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
        if is_perfect_init(i)
            && let Ok(mut perf) = MATHEMATICIAN.perfect.write() {
            perf.insert(i);
        }
    }
}

pub fn is_perfect_init(num: u64) -> bool {
    let mut temp = 0;
    let max = (num as f64 / 2.0) as u64;
    for i in 1..(max + 1) {
        if num.is_multiple_of(i) {
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
    if let Ok(mut primes) = MATHEMATICIAN.primes.write() {
        primes.insert(2);
    }
    for i in 3..(max + 1) {
        if is_prime_init(i)
            && let Ok(mut primes) = MATHEMATICIAN.primes.write() {
            primes.insert(i);
        }
    }
}

fn is_prime_init(n: u64) -> bool {
    if n.is_multiple_of(2) {
        return false;
    }
    let investigate_to = f64::sqrt(n as f64) as u64 + 1;
    for i in (3..investigate_to).step_by(2) {
        if n.is_multiple_of(i) {
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
        if let Ok(mut squares) = MATHEMATICIAN.square.write() {
            squares.insert(sq);
        }
    }
}

/**
 * Triangular numbers: 1, 3, 6, 10, 15
 */
pub fn init_triangular(max: u64) {
    println!("init_triangular()");
    let mut n = 1;
    while n * (n + 1) / 2 <= max {
        if let Ok(mut tri) = MATHEMATICIAN.triangular.write() {
            tri.insert(n * (n + 1) / 2);
        }
        n += 1;
    }
}

/**
 * Lucas numbers: 2, 1, 3, 4, 7, 11
 */
pub fn init_lucas(max: u64) {
    println!("init_lucas()");
    let (mut a, mut b) = (2u64, 1u64);
    while a <= max {
        if let Ok(mut lucas) = MATHEMATICIAN.lucas.write() {
            lucas.insert(a);
        }
        (a, b) = (b, a + b);
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
                .map(|c| (c.to_digit(10).unwrap_or(0) as u64).pow(2))
                .sum();
        }
        n == 1
    }
    for i in 1..=max {
        if is_happy(i)
            && let Ok(mut happy) = MATHEMATICIAN.happy.write() {
            happy.insert(i);
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
        if let Ok(mut lazy) = MATHEMATICIAN.lazy.write() {
            lazy.insert(val);
        }
        n += 1;
    }
}

pub fn clear() {
    println!("clear()");
    if let Ok(mut primes) = MATHEMATICIAN.primes.write() { primes.clear(); }
    if let Ok(mut fib) = MATHEMATICIAN.fibonacci.write() { fib.clear(); }
    if let Ok(mut perf) = MATHEMATICIAN.perfect.write() { perf.clear(); }
    if let Ok(mut sq) = MATHEMATICIAN.square.write() { sq.clear(); }
    if let Ok(mut tri) = MATHEMATICIAN.triangular.write() { tri.clear(); }
    if let Ok(mut lucas) = MATHEMATICIAN.lucas.write() { lucas.clear(); }
    if let Ok(mut lazy) = MATHEMATICIAN.lazy.write() { lazy.clear(); }
    if let Ok(mut happy) = MATHEMATICIAN.happy.write() { happy.clear(); }
}

#[cfg(test)]
mod tests {
    use crate::mathematician::{
        init_fibonacci, init_happy, init_lazy, init_lucas, init_perfect, init_primes, init_squares,
        init_triangular, is_fibonacci, is_happy, is_lazy, is_lucas, is_outside_cardioid,
        is_outside_circle, is_perfect, is_prime, is_square, is_triangular,
    };

    #[test]
    fn test_is_outside_cardioid() {
        assert!(!is_outside_cardioid(0.0, 0.0));
        assert!(is_outside_cardioid(2.0, 1.0));
    }

    #[test]
    fn test_is_outside_circle() {
        assert!(!is_outside_circle(-1.0, 0.0));
        assert!(is_outside_circle(2.0, 1.0));
    }

    #[test]
    fn test_is_prime() {
        init_primes(5);
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }

    #[test]
    fn test_is_fibonacci() {
        init_fibonacci(5);
        assert!(is_fibonacci(1));
        assert!(is_fibonacci(2));
        assert!(is_fibonacci(3));
        assert!(!is_fibonacci(4));
        assert!(is_fibonacci(5));
    }

    #[test]
    fn test_is_perfect() {
        init_perfect(6);
        assert!(!is_perfect(1));
        assert!(!is_perfect(2));
        assert!(!is_perfect(3));
        assert!(!is_perfect(4));
        assert!(!is_perfect(5));
        assert!(is_perfect(6));
    }

    #[test]
    fn test_is_square() {
        init_squares(4);
        assert!(is_square(1));
        assert!(!is_square(2));
        assert!(!is_square(3));
        assert!(is_square(4));
    }

    #[test]
    pub fn test_is_triangular() {
        init_triangular(6);
        assert!(is_triangular(1));
        assert!(!is_triangular(2));
        assert!(is_triangular(3));
        assert!(!is_triangular(4));
        assert!(!is_triangular(5));
        assert!(is_triangular(6));
    }

    #[test]
    pub fn test_is_lucas() {
        init_lucas(7);
        assert!(is_lucas(1));
        assert!(is_lucas(2));
        assert!(is_lucas(3));
        assert!(is_lucas(4));
        assert!(!is_lucas(5));
        assert!(!is_lucas(5));
        assert!(is_lucas(7));
    }

    #[test]
    pub fn test_is_lazy() {
        init_lazy(4);
        assert!(is_lazy(1));
        assert!(is_lazy(2));
        assert!(!is_lazy(3));
        assert!(is_lazy(4));
    }

    #[test]
    pub fn test_is_happy() {
        init_happy(10);
        assert!(is_happy(1));
        assert!(!is_happy(2));
        assert!(!is_happy(3));
        assert!(!is_happy(4));
        assert!(!is_happy(5));
        assert!(!is_happy(6));
        assert!(is_happy(7));
        assert!(!is_happy(8));
        assert!(!is_happy(9));
        assert!(is_happy(10));
    }
}
