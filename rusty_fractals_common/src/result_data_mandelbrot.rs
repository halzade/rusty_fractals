use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crate::area::Area;

pub struct MandelbrotResultPixel {
    value: u32,
    quad: f64,
    quid: f64, // inverted quadrance
}

pub struct ResultDataMandelbrot {
    pub width: usize,
    pub height: usize,
    pub points: Vec<Vec<Arc<Mutex<MandelbrotResultPixel>>>>,
}

impl ResultDataMandelbrot {
    pub fn set_pixel(&self, x: usize, y: usize, value: u32, quad: f64) {
        let arc = self.points.get(x).unwrap().get(y).unwrap();
        let mut mutex_guard = arc.lock().unwrap();
        let mrp = mutex_guard.deref_mut();
        mrp.value = value;
        mrp.quad = quad;
        mrp.quid = 1.0 / quad;
    }

    pub fn values_at(&self, x: usize, y: usize) -> (u32, f64, f64) {
        let arc = self.points.get(x).unwrap().get(y).unwrap();
        let mut mutex_guard = arc.lock().unwrap();
        let mrp = mutex_guard.deref_mut();
        (mrp.value, mrp.quad, mrp.quid)
    }

    pub fn all_points(&self) -> &Vec<Vec<Arc<Mutex<MandelbrotResultPixel>>>> {
        &self.points
    }
}

pub fn init(area: &Area) -> ResultDataMandelbrot {
    let mut vx = Vec::new();
    for _ in 0..area.width_x {
        let mut vy = Vec::new();
        for _ in 0..area.height_y {
            vy.push(Arc::new(Mutex::new(MandelbrotResultPixel { value: 0, quad: 1.0, quid: 1.0 })));
        }
        vx.push(vy);
    }
    ResultDataMandelbrot {
        width: area.width_x,
        height: area.height_y,
        points: vx,
    }
}