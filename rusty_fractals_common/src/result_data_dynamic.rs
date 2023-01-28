use std::sync::{Arc, Mutex};

pub struct ResultDataDynamic {
    // Dynamic Vec[re,im] calculation result data.
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the the tiny result_area.
    // Elements outside of tiny result_area are removed. Very short (calculation) paths are also removed.
    // All elements on paths are already inside result_area because they are filtered like that during the calculation.
    pub paths: Arc<Mutex<Vec<Vec<[f64; 2]>>>>,
}

impl ResultDataDynamic {
    pub fn all_paths(&self) -> Vec<Vec<[f64; 2]>> {
        self.paths.lock().unwrap().to_owned()
    }

    /*
    pub fn remove_elements_outside(&mut self) {
        println!("remove_elements_outside()");
        for mut path in self.paths {
            path.retain(|&el| self.area_result.contains(el[0], el[1]));
        }
        self.paths.retain(|path| path.len() as u32 > constants::MINIMUM_PATH_LENGTH);
    }
    */

    pub fn add_calculation_path(&self, path: Vec<[f64; 2]>) {
        self.paths.lock().unwrap().push(path);
    }
}

pub fn init() -> ResultDataDynamic {
    ResultDataDynamic {
        paths: Arc::new(Mutex::new(Vec::new()))
    }
}
