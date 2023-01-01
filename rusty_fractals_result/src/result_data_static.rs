use rusty_fractals_common::area::Area;
use crate::result_element_static::ResultElementStatic;
use crate::result_pixels::ResultPixels;

pub struct ResultData {
    pub pixels: Vec<Vec<ResultElementStatic>>,
    pub area_result: Area,
}

impl ResultData {
    pub fn remove_elements_outside(&mut self) {

    }

    pub fn add_calculation_path_now(&mut self, path: Vec<[f64; 2]>) {

    }
}