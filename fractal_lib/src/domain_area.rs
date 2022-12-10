use crate::fractal;
use fractal::{HEIGHT_Y, WIDTH_X};

pub struct DomainArea {
    pub numbers_re: [f64; WIDTH_X],
    pub numbers_im: [f64; HEIGHT_Y],
    border_low_re: f64,
    border_low_im: f64,
    border_high_re: f64,
    border_high_im: f64,
    plank: f64,
}

impl DomainArea {
    fn contains(&self, re: f64, im: f64) -> bool {
        re > self.border_low_re
            && re < self.border_high_re
            && im > self.border_low_im
            && im < self.border_high_im
    }

    pub fn screen_to_domain_re(&self, x: usize) -> f64 {
        self.numbers_re[x]
    }

    pub fn screen_to_domain_im(&self, y: usize) -> f64 {
        self.numbers_im[y]
    }
}

pub fn init(width_re: f64, center_re: f64, center_im: f64) -> DomainArea {
    let scr_ratio_x = WIDTH_X as f64 / HEIGHT_Y as f64;
    let width_im = width_re * scr_ratio_x;
    let plank = width_re / WIDTH_X as f64;

    let border_low_re = center_re - (width_re / 2.0);
    let border_high_re = center_re + (width_re / 2.0);
    let border_low_im = center_im - (width_im / 2.0);
    let border_high_im = center_im + (width_im / 2.0);

    println!("border_low_re  {}", border_low_re);
    println!("border_high_re {}", border_high_re);
    println!("border_low_im  {}", border_low_im);
    println!("border_high_im {}", border_high_im);

    /* Generate domain elements */
    let mut numbers_re: [f64; WIDTH_X] = [0.0; WIDTH_X];
    let mut numbers_im: [f64; HEIGHT_Y] = [0.0; HEIGHT_Y];
    for x in 0..WIDTH_X {
        numbers_re[x] = border_low_re + (plank * x as f64);
    }
    for y in 0..HEIGHT_Y {
        numbers_im[y] = border_low_im + (plank * y as f64);
    }

    DomainArea {
        numbers_re,
        numbers_im,
        border_low_re,
        border_low_im,
        border_high_re,
        border_high_im,
        plank,
    }
}

#[test]
fn test_init() {
    let area = init(1.0, 0.0, 0.0);
    assert_eq!(area.border_low_re, -0.5);
    assert_eq!(area.border_high_re, 0.5);
    assert_eq!(area.border_low_im, -0.5);
    assert_eq!(area.border_high_im, 0.5);
}

#[test]
fn test_contains() {
    let area = init(1.0, 0.0, 0.0);
    let y = area.contains(0.4, 0.4);
    let n = area.contains(0.4, 1.5);
    assert_eq!(y, true);
    assert_eq!(n, false);
}

#[test]
fn test_screen_to_domain_re() {
    let area = init(1.0, 0.0, 0.0);
    let r = area.screen_to_domain_re(500);
    assert_eq!(r, 0.125);
}

#[test]
fn test_screen_to_domain_im() {
    let area = init(1.0, 0.0, 0.0);
    let i = area.screen_to_domain_im(20);
    assert_eq!(i, -0.475);
}
