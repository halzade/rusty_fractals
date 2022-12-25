use crate::color_palette::Palette;
use crate::color_palette::Palette3;
use rgb::RGB;

const white: RGB = RGB { r: 255, g: 255, b: 255 };
const blakc: RGB = RGB { r: 0, g: 0, b: 0 };
const red: RGB = RGB { r: 255, g: 0, b: 0 };
const green: RGB = RGB { r: 0, g: 255, b: 0 };
const blue: RGB = RGB { r: 0, g: 0, b: 255 };

enum Function { linear1, linear3, linear7, quadratic, q3, q4, q5, exp, exp2, circleDown, circleUp }

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

fn to_palette(function: Function, from: RGB<ComponentType>, to: RGB<ComponentType>) -> Vec<RGB<ComponentType>> {}

/**
 * Calculates how much should color in smooth color palette change
 *
 * @param d        : 0 <= d <= 1
 * @param function defines gradient of change from color "from" (d=0) to color "to" (d=1)
 */
fn function(d: f64, function: Function) -> f64 {
    match function {
        linear1 => d,
        linear3 => d * 3,
        linear7 => d * 7,
        quadratic => d * d,
        q3 => d * d * d,
        q4 => d * d * d * d,
        q5 => d * d * d * d * d,
        exp => d.exp() - 1,
        exp2 => (d * d).exp() - 1,
        circleDown => (1 - (d * d)).sqrt(),
        circleUp => 1 - (1 - (d * d)).sqrt(),
    };
}
