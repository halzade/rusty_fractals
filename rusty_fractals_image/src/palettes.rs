use crate::palette::Palette;
use crate::palette::Palette3;
use rgb::RGB;

const white: RGB = RGB { r: 255, g: 255, b: 255 };
const blakc: RGB = RGB { r: 0, g: 0, b: 0 };
const red: RGB = RGB { r: 255, g: 0, b: 0 };
const green: RGB = RGB { r: 0, g: 255, b: 0 };
const blue: RGB = RGB { r: 0, g: 0, b: 255 };

pub enum Function { linear1, linear3, linear7, quadratic, q3, q4, q5, exp, exp2, circleDown, circleUp }

pub const PALETTE_BLACK_TO_WHITE: Palette = Palette {
    spectrum: to_palette(
        circleUp, black, white,
    )
};
pub const PALETTE_BLUE_TO_WHITE: Palette = Palette {
    spectrum: to_palette(
        circleUp,
        RGB::new(4, 13, 33),
        RGB::new(255, 255, 255),
    )
};
pub const PALETTE_GRAY_TO_BLUE: Palette = Palette {
    spectrum: to_palette(
        circleDown,
        RGB::new(104, 113, 133),
        RGB::new(4, 13, 33),
    )
};
pub const PALETTE_PURPLE_TO_WHITE: Palette = Palette {
    spectrum: to_palette(
        circleUp,
        RGB::new(20, 3, 30),
        white,
    )
};
pub const PALETTE_3_RGB: Palette3 = Palette3 {
    spectrum_r: to_palette(
        circleUp, black, red,
    ),
    spectrum_g: to_palette(
        circleUp, black, green,
    ),
    spectrum_b: to_palette(
        circleUp, black, blue,
    ),
};
