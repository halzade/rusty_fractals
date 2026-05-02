use crate::image::pixel::Spectra;
use crate::image::pixel_states::DomainElementState;
use image::Rgb;
use parking_lot::RwLock;

pub struct DataPx3 {
    _is_alive: RwLock<bool>, // TODO
    data3: RwLock<Data3>,
}

#[derive(Clone, Copy)]
struct Data3 {
    _origin_re: f64, // TODO
    _origin_im: f64, // TODO
    value_r: u64,
    value_g: u64,
    value_b: u64,
    // Element state is decided by calculation result.
    // Alternatively: If all it's neighbours finished too long,
    // it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    _state: DomainElementState, // TODO
    color_r: u8,
    color_g: u8,
    color_b: u8,
    color: Rgb<u8>,
}

impl DataPx3 {
    pub fn get_v3(&self) -> (u64, u64, u64) {
        // TODO throw instead
        let d = self.data3.read();
        (d.value_r, d.value_g, d.value_b)
    }

    pub fn set_c(&self, sp: Spectra, spectra_color_index: u8) {
        let mut d = self.data3.write();
        match sp {
            Spectra::Red => {
                d.color_r = spectra_color_index;
            }
            Spectra::Green => {
                d.color_g = spectra_color_index;
            }
            Spectra::Blue => {
                d.color_b = spectra_color_index;
            }
        }
    }

    pub fn define_color3(&self) {
        let color = {
            let d = self.data3.read();
            Rgb([d.color_r, d.color_g, d.color_b])
        };
        self.data3.write().color = color;
    }
}
