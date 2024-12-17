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

pub fn init_default() -> FractalData {
    FractalData {
        data: Mutex::new(Data { min: 1, max: 10 }),
    }
}
#[cfg(test)]
mod tests {
    use crate::fractal_data::init_default;

    #[test]
    fn test_conf_add() {
        let fd = init_default();

        fd.conf_add(1, 2);

        assert_eq!(fd.data.lock().unwrap().min, 2);
        assert_eq!(fd.data.lock().unwrap().max, 12);
    }
}
