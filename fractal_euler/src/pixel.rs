/* For simplicity Euler Fractal uses only three explicitly defined spectra */
pub enum Spectra { Red, Green, Blue }

struct PixelsEuler {
    width: u32,
    height: u32,
    elements_red: Vec<Vec<u32>>,
    elements_green: Vec<Vec<u32>>,
    elements_blue: Vec<Vec<u32>>,
}

impl PixelsEuler {
    pub fn add(&mut self, x: usize, y: usize, spec: Spectra) {
        match spec {
            Spectra::Red => self.elements_red[x][y] += 1,
            Spectra::Green => self.elements_green[x][y] += 1,
            Spectra::Blue => self.elements_blue[x][y] += 1,
        }
    }

    pub fn clear(&mut self) {
        let width = self.width as usize;
        let height = self.height as usize;
        for y in 0..width {
            for x in 0..height {
                self.elements_red[x][y] = 0;
                self.elements_green[x][y] = 0;
                self.elements_blue[x][y] = 0;
            }
        }
    }

    pub fn value_at(&mut self, x: usize, y: usize, spec: Spectra) -> u32 {
        match spec {
            Spectra::Red => self.elements_red[x][y],
            Spectra::Green => self.elements_green[x][y],
            Spectra::Blue => self.elements_blue[x][y],
        }
    }

    fn set(&mut self, x: usize, y: usize, spec: Spectra, color_value: u32) {
        match spec {
            Spectra::Red => self.elements_red[x][y] = color_value,
            Spectra::Green => self.elements_green[x][y] = color_value,
            Spectra::Blue => self.elements_blue[x][y] = color_value,
        }
    }
}
