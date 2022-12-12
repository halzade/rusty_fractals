pub struct FractalDomainPixel {
    pixel_value: u32,
    px: u32,
    py: u32,
    quad: f64,
    qiad: f64,
    color_value: u32,
}

impl FractalDomainPixel {
    fn color_value() -> u32 {
        color_value
    }

    fn set_color_value(&mut self, palette_colour_index: u32) {
        self.color_value = palette_colour_index;
    }

    pub fn quad() -> f64 {
        quad
    }
}