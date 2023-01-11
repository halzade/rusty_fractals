use rusty_fractals_common::area::Area;
use crate::result_pixels::ResultPixels;
use rusty_fractals_common::constants;

pub struct ResultData {
    // Dynamic Vec[re,im] calculation result data.
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the the tiny result_rea.
    // Elements outside of tiny result_rea are removed. Very short PATHS are also removed.
    // All elements on (calculation) path are already inside displayed result_area because they are filtered like that during the calculation.
    pub paths: Vec<Vec<[f64; 2]>>
}

impl ResultData {
    /*
    pub fn remove_elements_outside(&mut self) {
        println!("remove_elements_outside()");
        for mut path in self.paths {
            path.retain(|&el| self.area_result.contains(el[0], el[1]));
        }
        self.paths.retain(|path| path.len() as u32 > constants::MINIMUM_PATH_LENGTH);
    }
    */

    pub fn add_calculation_path(&mut self, path: Vec<[f64; 2]>) {
        self.paths.push(path);
    }
}