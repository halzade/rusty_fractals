use crate::palette::Palette;

pub struct ResultConfig {
    pub palette: Palette,
}

pub struct ResultConfigMandelbrot {
    pub palette: Palette,
    // to color insides of mandelbrot set
    pub palette_zero: Palette,
}