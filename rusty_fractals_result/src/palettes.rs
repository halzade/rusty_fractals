use crate::palette::Palette;
use crate::palette::Palette3;
use crate::palette_utils::make_spectrum;
use image::Rgb;

const white: Rgb = Rgb([255, 255, 255]);
const blakc: Rgb = Rgb([0, 0, 0]);
const red: Rgb = Rgb([255, 0, 0]);
const green: Rgb = Rgb([0, 255, 0]);
const blue: Rgb = Rgb([0, 0, 255]);

pub enum Function { linear1, linear3, linear7, quadratic, q3, q4, q5, exp, exp2, circleDown, circleUp }

pub const PALETTE_BLACK_TO_WHITE: Palette = Palette {
    spectrum: make_spectrum(
        circleUp, black, white,
    )
};
pub const PALETTE_BLUE_TO_WHITE: Palette = Palette {
    spectrum: make_spectrum(
        circleUp,
        Rgb::new(4, 13, 33),
        Rgb::new(255, 255, 255),
    )
};
pub const PALETTE_GRAY_TO_BLUE: Palette = Palette {
    spectrum: make_spectrum(
        circleDown,
        Rgb::new(104, 113, 133),
        Rgb::new(4, 13, 33),
    )
};
pub const PALETTE_PURPLE_TO_WHITE: Palette = Palette {
    spectrum: make_spectrum(
        circleUp,
        Rgb::new(20, 3, 30),
        white,
    )
};

pub const PALETTE_3_RGB: Palette3 = Palette3 {
    spectrum_red: make_spectrum(
        circleUp, black, red,
    ),
    spectrum_green: make_spectrum(
        circleUp, black, green,
    ),
    spectrum_blue: make_spectrum(
        circleUp, black, blue,
    ),
};
