use crate::palette::Palette;
use crate::palette::Palette3;
use crate::palette_utils::make_spectrum;
use crate::palettes::Function::{CircleDown, CircleUp, Exp2, Linear1};
use image::Rgb;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

pub enum Function {
    Linear1,
    Linear3,
    Linear7,
    Quadratic,
    Exp,
    Exp2,
    CircleDown,
    CircleUp,
}

pub enum PaletteName {
    Nothing,
    BlackToWhiteExp2,
    BlackToWhiteCircleUp,
    BlackWhiteBlack,
    BlueToWhiteCircleUp,
    GrayToBlue,
    GrayToBlackCircleDown,
    PurpleToWhite,
}

pub fn new<'lt>(function: Function, from: Rgb<u8>, to: Rgb<u8>) -> Palette {
    Palette {
        spectrum: make_spectrum(function, from, to),
    }
}

pub fn new_palette_by_name<'lt>(palette_name: &PaletteName) -> Palette {
    match palette_name {
        PaletteName::BlackToWhiteExp2 => palette_black_to_white_exp2(),
        PaletteName::BlackToWhiteCircleUp => palette_black_to_white_circle_up(),
        PaletteName::BlackWhiteBlack => palette_black_white_black(),
        PaletteName::BlueToWhiteCircleUp => palette_blue_to_white_circle_up(),
        PaletteName::GrayToBlue => palette_gray_to_blue(),
        PaletteName::GrayToBlackCircleDown => palette_gray_to_black_circle_down(),
        PaletteName::PurpleToWhite => palette_purple_to_white(),
        PaletteName::Nothing => init_trivial(),
    }
}

pub fn palette_black_to_white_exp2<'lt>() -> Palette {
    new(Exp2, BLACK, WHITE)
}

pub fn palette_black_to_white_circle_up<'lt>() -> Palette {
    new(CircleUp, BLACK, WHITE)
}

pub fn palette_black_white_black<'lt>() -> Palette {
    let mut black_to_white_work = make_spectrum(Linear1, BLACK, WHITE);
    let mut white_to_black = make_spectrum(Linear1, WHITE, BLACK);
    black_to_white_work.append(&mut white_to_black);
    Palette {
        spectrum: black_to_white_work,
    }
}

pub fn palette_blue_to_white_circle_up<'lt>() -> Palette {
    new(CircleUp, Rgb([4, 13, 33]), WHITE)
}

pub fn palette_gray_to_blue<'lt>() -> Palette {
    new(CircleDown, Rgb([104, 113, 133]), Rgb([4, 13, 33]))
}

pub fn palette_gray_to_black_circle_down<'lt>() -> Palette {
    new(CircleDown, Rgb([100, 100, 100]), Rgb([0, 0, 0]))
}

pub fn palette_purple_to_white<'lt>() -> Palette {
    new(CircleUp, Rgb([20, 3, 30]), WHITE)
}

pub fn init_trivial<'lt>() -> Palette {
    Palette {
        spectrum: Vec::new(),
    }
}

pub fn palette_3_rgb() -> Palette3 {
    Palette3 {
        spectrum_red: make_spectrum(CircleUp, BLACK, RED),
        spectrum_green: make_spectrum(CircleUp, BLACK, GREEN),
        spectrum_blue: make_spectrum(CircleUp, BLACK, BLUE),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
