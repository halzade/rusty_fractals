use crate::palette::Palette;
use crate::palette::Palette3;
use crate::palette_utils::make_spectrum;
use crate::palettes::Function::{CircleUp, Exp2, Linear1};
use image::{Pixel, Rgb};

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
const GOLD: Rgb<u8> = Rgb([255, 215, 0]);

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

#[derive(Clone, Copy, Debug)]
pub enum PaletteName {
    Nothing,
    BlackToWhiteExp2,
    BlackToWhiteCircleUp,
    BlackWhiteBlack,
    BlackWBWB,
    BlueToWhiteCircleUp,
    PurpleToWhite,
    LinearGold,
    LinearRed,
    LinearBlue,
    LinearGray,
    NonlinearVolt,
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
        PaletteName::BlackWBWB => palette_bwbwb(),
        PaletteName::BlueToWhiteCircleUp => palette_blue_to_white_circle_up(),
        PaletteName::PurpleToWhite => palette_purple_to_white(),
        PaletteName::LinearGold => palette_linear_gold(),
        PaletteName::LinearRed => palette_linear_red(),
        PaletteName::LinearBlue => palette_linear_blue(),
        PaletteName::LinearGray => palette_linear_gray(),
        PaletteName::NonlinearVolt => palette_nonlinear_volt(),
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

pub fn palette_bwbwb<'lt>() -> Palette {
    let mut bwbwb_work = make_spectrum(Linear1, BLACK, WHITE);
    bwbwb_work.append(&mut make_spectrum(Linear1, WHITE, BLACK));
    bwbwb_work.append(&mut make_spectrum(Linear1, BLACK, WHITE));
    bwbwb_work.append(&mut make_spectrum(Linear1, WHITE, BLACK));
    Palette {
        spectrum: bwbwb_work,
    }
}

pub fn palette_blue_to_white_circle_up<'lt>() -> Palette {
    new(CircleUp, Rgb([4, 13, 33]), WHITE)
}

pub fn palette_linear_blue<'lt>() -> Palette {
    new(Linear1, Rgb([104, 113, 133]), Rgb([4, 13, 33]))
}

pub fn palette_linear_gray<'lt>() -> Palette {
    new(Linear1, Rgb([100, 100, 100]), Rgb([0, 0, 0]))
}

pub fn palette_nonlinear_volt<'lt>() -> Palette {
    new(CircleUp, Rgb([32, 15, 48]), WHITE)
}

pub fn palette_purple_to_white<'lt>() -> Palette {
    new(CircleUp, Rgb([20, 3, 30]), WHITE)
}

pub fn palette_linear_gold<'lt>() -> Palette {
    new(Linear1, Rgb([4, 13, 33]), GOLD)
}

pub fn palette_linear_red<'lt>() -> Palette {
    new(Linear1, BLACK, RED)
}

pub fn init_trivial<'lt>() -> Palette {
    Palette {
        spectrum: Vec::new(),
    }
}

pub fn palette_3_rgb() -> Palette3 {
    Palette3 {
        spectrum_red: strip_spectrum(make_spectrum(CircleUp, BLACK, RED), 0),
        spectrum_green: strip_spectrum(make_spectrum(CircleUp, BLACK, GREEN), 1),
        spectrum_blue: strip_spectrum(make_spectrum(CircleUp, BLACK, BLUE), 2),
    }
}
fn strip_spectrum(spectrum: Vec<Rgb<u8>>, channel: usize) -> Vec<u8> {
    spectrum
        .into_iter()
        .map(|pixel| pixel.channels()[channel])
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::palettes::new;
    use crate::palettes::Function::Linear1;
    use image::Rgb;

    #[test]
    fn test_new() {
        let n = new(Linear1, Rgb([100, 100, 100]), Rgb([103, 103, 103]));
        assert_eq!(n.spectrum.len(), 4);
    }
}
