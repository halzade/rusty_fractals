use crate::pixel_states::DomainElementState;
use image::Rgb;
use std::sync::RwLock;

pub struct DataPx3 {
    is_alive: RwLock<bool>,
    data: RwLock<Data3>,
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
    pub color_r: Option<Rgb<u8>>,
    pub color_g: Option<Rgb<u8>>,
    pub color_b: Option<Rgb<u8>>,
}
