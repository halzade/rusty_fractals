extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use fractal_lib::mem::Mem;

fn main() {
    println!("Hello, world!");
}

fn math(m: &mut Mem, origin_re: f64, origin_im: f64) {
    m.square();
    m.conjugation();
    m.square();
    m.plus(origin_re, origin_im);
}

#[test]
fn test_math() {
    // m not mut, m should always be mound to this Mem
    let mut m = fractal_lib::mem::Mem{ re: 0.0, im: 0.0 };
    math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}