use image::Rgb;

const ERR_RED: Rgb<u8> = Rgb([255, 0, 0]);

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
        self.spectrum.get(i).unwrap_or(&ERR_RED).clone()
    }
}

impl Palette3 {
    pub fn spectrum_value_red(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_red.get(i).unwrap_or(&ERR_RED)
    }
    pub fn spectrum_value_green(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_green.get(i).unwrap_or(&ERR_RED)
    }
    pub fn spectrum_value_blue(&self, i: usize) -> &Rgb<u8> {
        self.spectrum_blue.get(i).unwrap_or(&ERR_RED)
    }
}