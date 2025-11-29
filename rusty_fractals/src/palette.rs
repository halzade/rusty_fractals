use crate::palette_utils;
use image::Rgb;

const ERROR_MESSAGE: &str = "error in color index";

pub struct Palette {
    pub spectrum: Vec<Rgb<u8>>,
}

pub struct Palette3 {
    pub spectrum_red: Vec<u8>,
    pub spectrum_green: Vec<u8>,
    pub spectrum_blue: Vec<u8>,
}

impl Palette {
    pub fn spectrum_value(&self, i: usize) -> Rgb<u8> {
        *self.spectrum.get(i).expect(ERROR_MESSAGE)
    }
}

impl Palette3 {
    pub fn spectrum_value_red(&self, i: usize) -> u8 {
        *self.spectrum_red.get(i).expect(ERROR_MESSAGE)
    }

    pub fn spectrum_value_green(&self, i: usize) -> u8 {
        *self.spectrum_green.get(i).expect(ERROR_MESSAGE)
    }

    pub fn spectrum_value_blue(&self, i: usize) -> u8 {
        *self.spectrum_blue.get(i).expect(ERROR_MESSAGE)
    }
}

pub fn init_trivial() -> Palette {
    Palette {
        spectrum: palette_utils::init_trivial(),
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::init_trivial;
    use image::Pixel;

    #[test]
    fn test_init_default() {
        let p = init_trivial();
        assert_eq!(p.spectrum.len(), 3);
    }

    #[test]
    fn test_spectrum_value() {
        let p = init_trivial();

        let rgb = p.spectrum_value(2);
        assert_eq!(rgb.channels()[0], 0);
    }
}
