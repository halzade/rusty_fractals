use std::sync::Mutex;

pub struct FractalData {
    pub data: Mutex<Data>,
}

pub struct Data {
    pub min: u32,
    pub max: u32,
}

impl FractalData {
    pub fn conf_add(&self, min: u32, max: u32) {
        match self.data.lock() {
            Ok(mut d) => {
                d.min += min;
                d.max += max;
            }
            Err(e) => {
                println!("FractalData.conf_add(): {}", e);
            }
        }
    }
}