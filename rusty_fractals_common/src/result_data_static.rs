use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crate::area::Area;

pub struct ResultDataStatic {
    pub points: Vec<Vec<Arc<Mutex<u32>>>>,
}

impl ResultDataStatic {
    pub fn translate_path_to_point_grid(&self, path: Vec<[f64; 2]>, area: &Area) {
        for [re, im] in path {
            let (x, y) = area.domain_point_to_result_pixel(re, im);
            self.add(x, y);
        }
    }

    fn add(&self, x: usize, y: usize) {
        let arc = self.points.get(x).unwrap().get(y).unwrap();
        let mut mutex_guard = arc.lock().unwrap();
        let v = mutex_guard.deref_mut();
        *v += 1;
    }

    pub fn all_points(&self) -> &Vec<Vec<Arc<Mutex<u32>>>> {
        &self.points
    }
}

pub fn init(area: &Area) -> ResultDataStatic {
    let mut vx = Vec::new();
    for _ in 0..area.width_x {
        let mut vy = Vec::new();
        for _ in 0..area.height_y {
            vy.push(Arc::new(Mutex::new(0)));
        }
        vx.push(vy);
    }
    ResultDataStatic {
        points: vx
    }
}