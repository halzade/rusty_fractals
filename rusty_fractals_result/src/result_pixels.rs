use std::ops::Deref;
use std::sync::{Arc, Mutex};
use rusty_fractals_common::area::Area;

pub struct ResultPixels {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<u32>>,
}

impl ResultPixels {

    pub fn translate_all_paths_to_pixel_grid(&mut self, paths: Vec<Vec<[f64; 2]>>, area: &Area) {
        println!("translate_all_paths_to_pixel_grid()");

        let mut pixels_total = 0;

        for path in paths {
            for [re, im] in path {
                // translate [re,im] to [px,py]
                // if area.contains(re, im) {
                let (px, py) = area.domain_point_to_result_pixel(re, im);
                self.add(px, py);
                pixels_total += 1;
            }
        }
        println!("pixels_total:   {}", pixels_total);

        // remove elements which moved out of tiny area
        // TODO self.remove_elements_outside();

        // Stats.pathsTotalAmount = PATHS.size();
        // Stats.pixelsValueTotal = pixels_total;
    }

    pub fn translate_all_points_to_pixel_grid(&mut self, points: &Vec<Vec<Arc<Mutex<u32>>>>, area: &Area) {
        println!("translate_all_points_to_pixel_grid()");

        let mut pixels_total = 0;

        for x in 0..area.width_x {
            for y in 0..area.height_y {
                let arc = points.get(x).unwrap().get(y).unwrap();
                let mutex_guard = arc.lock().unwrap();
                let value = mutex_guard.deref();
                self.set(x, y, value);
                pixels_total += 1;
            }
        }
        println!("pixels_total:   {}", pixels_total);

        // remove elements which moved out of tiny area
        // TODO self.remove_elements_outside();

        // Stats.pathsTotalAmount = PATHS.size();
        // Stats.pixelsValueTotal = pixels_total;
    }

    pub fn add(&mut self, x: usize, y: usize) {
        self.pixels[x][y] += 1;
    }

    pub fn set(&mut self, x: usize, y: usize, value: &u32) {
        self.pixels[x][y] = *value;
    }

    pub fn clear(&mut self) {
        println!("clear");
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[x][y] = 0;
            }
        }
    }

    pub fn value_at(&self, x: usize, y: usize) -> u32 {
        self.pixels[x][y]
    }

    pub fn best_four_chunks_value(&self) -> u32 {
        println!("best_four_chunks_value()");
        let chunk_size_x = self.width / 20;
        let chunk_size_y = self.height / 20;
        let mut values: Vec<u32> = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                values.push(self.chunk_value(
                    x * chunk_size_x, (x + 1) * chunk_size_x,
                    y * chunk_size_y, (y + 1) * chunk_size_y,
                ));
            }
        }
        values.sort_by(|first, second| second.cmp(first));

        let mut sum = 0;
        for i in 0..4 {
            let v = values.get(i);
            match v {
                Some(v) => sum += v,
                None => panic!(),
            }
        }
        println!("best_four_chunks_value() sum: {}", sum);
        sum
    }

    fn chunk_value(&self, x_from: usize, x_to: usize, y_from: usize, y_to: usize) -> u32 {
        let mut sum = 0;
        for x in x_from..x_to {
            for y in y_from..y_to {
                sum += self.pixels[x][y];
            }
        }
        sum
    }
}

pub fn init(area: &Area) -> ResultPixels {
    let mut vx = Vec::new();
    for _ in 0..area.width_x {
        let mut vy = Vec::new();
        for _ in 0..area.height_y {
            vy.push(0);
        }
        vx.push(vy);
    }
    ResultPixels {
        width: area.width_x,
        height: area.height_y,
        pixels: vx,
    }
}