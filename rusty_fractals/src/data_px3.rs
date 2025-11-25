use crate::pixel::Spectra;
use crate::pixel_states::DomainElementState;
use image::Rgb;
use std::sync::RwLock;

pub struct DataPx3 {
    is_alive: RwLock<bool>,
    data3: RwLock<Data3>,
}

#[derive(Clone, Copy)]
pub struct Data3 {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value_r: u32,
    pub value_g: u32,
    pub value_b: u32,
    // Element state is decided by calculation result.
    // Alternatively: If all it's neighbours finished too long,
    // it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    pub state: DomainElementState,
    pub color: Option<Rgb<u8>>,
}

impl DataPx3 {
    pub fn get_v3(&self) -> (u32, u32, u32) {
        let d = self.data3.read().unwrap();
        (d.value_r, d.value_g, d.value_b)
    }

    pub fn set_c(&self, sp: Spectra, c: usize) {
        match sp {
            // todo
            Spectra::Red => {
                // self.data3.write().unwrap().color_r = Some(c);
            }
            Spectra::Green => {
                // self.data3.write().unwrap().color_g = Some(c);
            }
            Spectra::Blue => {
                // self.data3.write().unwrap().color_b = Some(c);
            }
        }
    }

    pub fn set_c3(&self) {
        let d = self.data3.read().unwrap();
        let (r, g, b) = (d.value_r, d.value_g, d.value_b);
        self.data3.write().unwrap().color = Some(Rgb([r as u8, g as u8, b as u8]));
    }
}
