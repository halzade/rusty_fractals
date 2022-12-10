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
    let center_re = 0.0;
    let center_im = 0.0;
    let width_x = 1.0;
    let da = init(width_x, center_re, center_im);
    assert_eq!(da.border_low_re, -0.5);
    assert_eq!(da.border_high_re, 0.5);
    assert_eq!(da.border_low_im, -0.5);
    assert_eq!(da.border_high_im, 0.5);
}

#[test]
fn test_contains() {}

#[test]
fn test_screen_to_domain_re() {}

#[test]
fn test_screen_to_domain_im() {}
