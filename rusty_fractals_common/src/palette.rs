use image::Rgb;

const ERROR_MESSAGE: &str = "error in color index";

pub struct Palette {
    pub spectrum: Vec<Rgb<u8>>,
}

pub struct Palette3 {
    pub spectrum_red: Vec<Rgb<u8>>,
    pub spectrum_green: Vec<Rgb<u8>>,
    pub spectrum_blue: Vec<Rgb<u8>>,
}

impl Palette {
    pub fn spectrum_value(&self, i: usize) -> Rgb<u8> {
        *self.spectrum.get(i).expect(ERROR_MESSAGE)
    }
}

impl Palette3 {
    pub fn spectrum_value_red(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_red.get(i).expect(ERROR_MESSAGE)
    }
    pub fn spectrum_value_green(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_green.get(i).expect(ERROR_MESSAGE)
    }
    pub fn spectrum_value_blue(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_blue.get(i).expect(ERROR_MESSAGE)
    }
}