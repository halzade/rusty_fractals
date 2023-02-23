use crate::palette::Palette;
use crate::palette::Palette3;
use crate::palette_utils::make_spectrum;
use image::Rgb;
use crate::palettes::Function::{CircleDown, CircleUp, Exp2, Linear1};

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

pub struct ResultConfig {
    pub palette: Palette,
}

pub struct ResultConfigMandelbrot {
    pub palette: Palette,
    // to color insides of mandelbrot set
    pub palette_zero: Palette,
}

pub enum Function { Linear1, Linear3, Linear7, Quadratic, Exp, Exp2, CircleDown, CircleUp }

pub fn palette_black_to_white_exp2() -> Palette {
    Palette {
        spectrum: make_spectrum(Exp2, BLACK, WHITE)
    }
}

pub fn palette_bwb() -> Palette {
    let mut black_to_white_work = make_spectrum(Linear1, BLACK, WHITE);
    let mut white_to_black = make_spectrum(Linear1, WHITE, BLACK);
    black_to_white_work.append(&mut white_to_black);
    Palette {
        spectrum: black_to_white_work
    }
}

pub fn palette_blue_to_white_circle_up() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleUp, Rgb([4, 13, 33]), WHITE)
    }
}

pub fn palette_gray_to_blue() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleDown, Rgb([104, 113, 133]), Rgb([4, 13, 33]))
    }
}

pub fn palette_purple_to_white() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleUp, Rgb([20, 3, 30]), WHITE)
    }
}

pub fn palette_3_rgb() -> Palette3 {
    Palette3 {
        spectrum_red: make_spectrum(CircleUp, BLACK, RED),
        spectrum_green: make_spectrum(CircleUp, BLACK, GREEN),
        spectrum_blue: make_spectrum(CircleUp, BLACK, BLUE),
    }
}
