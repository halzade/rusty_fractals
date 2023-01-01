use crate::palette::Palette;
use crate::palette::Palette3;
use crate::palette_utils::make_spectrum;
use image::Rgb;
use crate::palettes::Function::{CircleDown, CircleUp};

const white: Rgb<u8> = Rgb([255, 255, 255]);
const black: Rgb<u8> = Rgb([0, 0, 0]);
const red: Rgb<u8> = Rgb([255, 0, 0]);
const green: Rgb<u8> = Rgb([0, 255, 0]);
const blue: Rgb<u8> = Rgb([0, 0, 255]);

pub enum Function { Linear1, Linear3, Linear7, Quadratic, Exp, Exp2, CircleDown, CircleUp }

pub fn palette_black_to_white() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleUp, black, white)
    }
}

pub fn palette_blue_to_white() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleUp, Rgb([4, 13, 33]), Rgb([255, 255, 255]))
    }
}

pub fn palette_gray_to_blue() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleDown, Rgb([104, 113, 133]), Rgb([4, 13, 33]))
    }
}

pub fn palette_purple_to_white() -> Palette {
    Palette {
        spectrum: make_spectrum(CircleUp, Rgb([20, 3, 30]), white)
    }
}

pub fn palette_3_rgb() -> Palette3 {
    Palette3 {
        spectrum_red: make_spectrum(CircleUp, black, red),
        spectrum_green: make_spectrum(CircleUp, black, green),
        spectrum_blue: make_spectrum(CircleUp, black, blue),
    }
}
