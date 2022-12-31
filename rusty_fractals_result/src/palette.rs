use rgb::RGB;

pub struct Palette {
    pub spectrum: Vec<RGB<u8>>,
}

pub struct Palette3 {
    pub spectrum_red: Vec<RGB<u8>>,
    pub spectrum_green: Vec<RGB<u8>>,
    pub spectrum_blue: Vec<RGB<u8>>,
}

const ERR_RED: RGB<u8> = RGB::new(255, 0, 0);

impl Palette {
    pub fn spectrum_value(&self, i: usize) -> &RGB<u8> {
        self.spectrum.get(i).unwrap_or(&ERR_RED)
    }
}

impl Palette3 {
    pub fn spectrum_value_red(&self, i: usize) -> &RGB<u8> {
        self.spectrum_red.get(i).unwrap_or(&ERR_RED)
    }
    pub fn spectrum_value_green(&self, i: usize) -> &RGB<u8> {
        self.spectrum_green.get(i).unwrap_or(&ERR_RED)
    }
    pub fn spectrum_value_blue(&self, i: usize) -> &RGB<u8> {
        self.spectrum_blue.get(i).unwrap_or(&ERR_RED)
    }
}