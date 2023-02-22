use std::sync::{Arc, Mutex, MutexGuard};
use image::{ImageBuffer, Rgb, RgbImage};
use crate::area::Area;
use crate::constants::NEIGHBOURS;
use crate::data_px;
use crate::data_px::DataPx;
use crate::pixel_states::{ACTIVE_NEW, DomainElementState, FINISHED, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK, is_finished_success_past};
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort, HibernatedDeepBlack};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Square2;

pub struct DataImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Arc<Mutex<DataPx>>>>,
}

static MAX_VALUE: Mutex<u32> = Mutex::new(0);

impl DataImage {
    pub fn colour(&self, x: usize, y: usize, palette_colour: Rgb<u8>) {
        let mut p = self.px_at(x, y);
        p.colour = Some(palette_colour);
    }

    pub fn image(&self, final_image: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        return if final_image {
            self.image_result()
        } else {
            self.image_temp()
        };
    }

    pub fn image_temp(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let (value, state, _, _, colour_index_o) = self.values_at(x, y);
                let colour: Rgb<u8>;
                match colour_index_o {
                    Some(pixel_colour) => {
                        colour = pixel_colour;
                    }
                    None => {
                        if state == ActiveNew {
                            colour = colour_for_state(state);
                        } else {
                            let mut mv = MAX_VALUE.lock().unwrap();
                            if value > *mv {
                                *mv = value;
                            }
                            let c: u8 = (value as f64 / *mv as f64 * 255.0) as u8;
                            colour = Rgb([c, c, c]);
                        }
                    }
                }
                image.put_pixel(x as u32, y as u32, colour);
            }
        }
        image
    }

    pub fn image_result(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let (_, _, _, _, colour_index_o) = self.values_at(x, y);
                image.put_pixel(x as u32, y as u32, colour_index_o.unwrap());
            }
        }
        image
    }

    pub fn image_init(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                image.put_pixel(x as u32, y as u32, colour_for_state(ActiveNew));
            }
        }
        image
    }

    pub fn translate_path_to_point_grid(&self, path: Vec<[f64; 2]>, area: &Area) {
        for [re, im] in path {
            let (x, y) = area.domain_point_to_result_pixel(re, im);
            self.add(x, y);
        }
    }

    fn add(&self, x: usize, y: usize) {
        let mut p = self.px_at(x, y);
        p.value += 1;
    }

    fn px_at(&self, x: usize, y: usize) -> MutexGuard<DataPx> {
        let arc_mutex_dpx = self.pixels.get(x).unwrap().get(y).unwrap();
        arc_mutex_dpx.lock().unwrap()
    }

    pub fn values_at(&self, x: usize, y: usize) -> (u32, DomainElementState, f64, f64, Option<Rgb<u8>>) {
        let p = self.px_at(x, y);
        (p.value, p.state, p.quad, p.quid, p.colour)
    }

    pub fn value_state_at(&self, x: usize, y: usize) -> (u32, DomainElementState) {
        let p = self.px_at(x, y);
        (p.value, p.state)
    }

    pub fn value_at(&self, x: usize, y: usize) -> u32 {
        let p = self.px_at(x, y);
        p.value
    }

    pub fn state_origin_at(&self, x: usize, y: usize) -> (DomainElementState, f64, f64) {
        let p = self.px_at(x, y);
        (p.state, p.origin_re, p.origin_im)
    }

    pub fn origin_at(&self, x: usize, y: usize) -> (f64, f64) {
        let p = self.px_at(x, y);
        (p.origin_re, p.origin_im)
    }

    pub fn set_pixel_mandelbrot(&self, x: usize, y: usize, iterator: u32, quad: f64, state: DomainElementState, max: u32) {
        let mut p = self.px_at(x, y);
        p.quad = quad;
        p.quid = 1.0 / quad;
        p.state = state;
        if iterator < 1 {
            p.value = 1;
        } else if iterator == max {
            p.value = 0;
        } else {
            p.value = iterator
        }
    }

    // for Nebula like fractals
    pub fn set_pixel_state(&self, x: usize, y: usize, state: DomainElementState) {
        let mut p = self.px_at(x, y);
        p.quad = 1.0;
        p.quid = 1.0;
        p.state = state;
    }

    fn past(&self, x: usize, y: usize) {
        self.px_at(x, y).past();
    }

    pub fn recalculate_pixels_states(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.past(x, y);
            }
        }
    }

    // all new elements are Active New
    // for wrapping, search only elements, which have some past well finished neighbors
    // previous calculation must be completed
    pub fn is_on_mandelbrot_horizon(&self, x: usize, y: usize) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..neigh {
            for b in -neigh..neigh {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if check_domain(xx, yy, self.width, self.height) {
                    let (_, state) = self.value_state_at(xx as usize, yy as usize);
                    if is_finished_success_past(state) {
                        return true;
                    }
                }
            }
        }
        false
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
                sum += self.px_at(x, y).value;
            }
        }
        sum
    }

    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    pub fn wrap(&self, origin_re: f64, origin_im: f64, rm: ResolutionMultiplier, area: &Area) -> Vec<[f64; 2]> {
        let mut ret = Vec::new();
        if rm == Square2 {
            let d = area.plank() / 3.0;
            ret.push([origin_re + d, origin_im + d]);
            ret.push([origin_re - d, origin_im - d]);
            ret.push([origin_re - d, origin_im + d]);
            ret.push([origin_re + d, origin_im - d]);
        } else {
            let multiplier: f64 = resolve_multiplier(rm);
            let d: f64 = area.plank() / multiplier;
            // multiplier was odd
            let half = ((multiplier - 1.0) / 2.0) as i32;
            // This fills the pixel with multiple points
            for x in -half..half {
                for y in -half..half {
                    if x != 0 || y != 0 {
                        ret.push([origin_re + (x as f64 * d), origin_im + (y as f64 * d)]);
                    }
                }
            }
        }
        ret
    }
}

pub fn init_data_image(area: &Area) -> DataImage {
    let width = area.width_x;
    let height = area.height_y;
    let mut vx = Vec::new();
    for x in 0..width {
        let mut vy = Vec::new();
        for y in 0..height {
            let origin_re = area.screen_to_domain_re(x);
            let origin_im = area.screen_to_domain_im(y);
            vy.push(Arc::new(Mutex::new(data_px::init(origin_re, origin_im))));
        }
        vx.push(vy);
    }
    DataImage {
        width,
        height,
        pixels: vx,
    }
}

pub fn state_from_path_length(iterator: u32, path_length: u32, min: u32, max: u32) -> DomainElementState {
    if path_length < min {
        return FinishedTooShort;
    }
    if iterator == max {
        return FinishedTooLong;
    }
    FinishedSuccess
}

pub fn resolve_multiplier(rm: ResolutionMultiplier) -> f64 {
    match rm {
        ResolutionMultiplier::Single => 1.0,
        Square3 => 3.0,
        Square5 => 5.0,
        Square9 => 9.0,
        Square11 => 11.0,
        Square51 => 51.0,
        Square101 => 101.0,
        Square2 => 1.0
    }
}

pub fn colour_for_state(state: DomainElementState) -> Rgb<u8> {
    match state {
        // most of the elements are going to be FinishedSuccessPast
        FinishedSuccessPast => FINISHED_SUCCESS_PAST,
        HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
        ActiveNew => ACTIVE_NEW,
        FinishedSuccess => FINISHED_SUCCESS,
        FinishedTooShort => FINISHED_TOO_SHORT,
        FinishedTooLong => FINISHED_TOO_LONG,
        Finished => FINISHED
    }
}

fn check_domain(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}
