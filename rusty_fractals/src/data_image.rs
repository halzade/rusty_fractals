use crate::area::Area;
use crate::constants::{MINIMUM_PATH_LENGTH, NEIGHBOURS};
use crate::data_image::DataType::{Dynamic, Static};
use crate::data_px;
use crate::data_px::DataPx;
use crate::pixel_states::DomainElementState::{
    ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    HibernatedDeepBlack,
};
use crate::pixel_states::{
    is_finished_success_past, DomainElementState, ACTIVE_NEW, FINISHED_SUCCESS,
    FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK,
};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Square2;
use image::Rgb;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct DataImage {
    pub width_x: usize,
    pub height_y: usize,
    pub data_type: DataType,
    // static data for image
    pub pixels: Vec<Vec<Mutex<Option<DataPx>>>>,
    // dynamic data for zoom video
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the tiny area.
    // Elements outside the tiny result_area are removed. Very short (calculation) paths are also removed.
    // All elements on paths are already inside result_area because they are filtered like that during the calculation.
    pub paths: Arc<Mutex<Vec<Vec<[f64; 2]>>>>,
    // show one patch during calculation with pixel wrap
    pub show_path: Mutex<Vec<[f64; 2]>>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    Dynamic,
    Static,
}

impl DataImage {
    pub fn colour(&self, x: usize, y: usize, palette_colour: Rgb<u8>) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.colour = Some(palette_colour);
    }

    // save path to show during recalculation with pixel wrap
    fn set_show_path(&self, path: &Vec<[f64; 2]>) {
        let saved_len = self.show_path.lock().unwrap().len();
        let new_len = path.len();
        if new_len > (saved_len * 2) {
            println!("set_show_path() : {}", new_len);
            *self.show_path.lock().unwrap() = path.clone();
        }
    }

    pub fn translate_path_to_point_grid(&self, path: Vec<[f64; 2]>, area: &Area, is_wrap: bool) {
        if is_wrap {
            self.set_show_path(&path);
        }
        for [re, im] in path {
            let (x, y) = area.point_to_pixel(re, im);
            self.add(x, y);
        }
    }

    pub fn translate_all_paths_to_point_grid(&self, area: &Area) {
        let all = self.paths.lock().unwrap().to_owned();
        for path in all {
            for [re, im] in path {
                let (x, y) = area.point_to_pixel(re, im);
                self.add(x, y);
            }
        }
    }

    pub fn save_path(&self, path: Vec<[f64; 2]>, is_wrap: bool) {
        if is_wrap {
            self.set_show_path(&path);
        }
        self.paths.lock().unwrap().push(path);
    }

    pub fn remove_elements_outside(&self, area: &Area) {
        println!("remove_elements_outside()");
        let all = self.paths.lock().unwrap().to_owned();
        for mut path in all {
            path.retain(|&el| area.contains(el[0], el[1]));
        }
        self.paths
            .lock()
            .unwrap()
            .retain(|path| path.len() as u32 > MINIMUM_PATH_LENGTH);
    }

    fn add(&self, x: usize, y: usize) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.value += 1;
    }

    pub fn mo_px_at(&self, x: usize, y: usize) -> MutexGuard<Option<DataPx>> {
        self.pixels.get(x).unwrap().get(y).unwrap().lock().unwrap()
    }

    fn move_px_to_new_position(&self, x: usize, y: usize, px: DataPx) {
        let mu = self.pixels.get(x).unwrap().get(y).unwrap();
        let lo = mu.lock();
        match lo {
            Ok(mut op) => {
                op.replace(px);
            }
            Err(e) => {
                println!("move_px_to_new_position(): {}", e);
            }
        }
    }

    pub fn values5_at(
        &self,
        x: usize,
        y: usize,
    ) -> (u32, DomainElementState, f64, f64, Option<Rgb<u8>>) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.value, p.state, p.quad, p.quid, p.colour)
    }

    pub fn values3_at(&self, x: usize, y: usize) -> (u32, DomainElementState, Option<Rgb<u8>>) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.value, p.state, p.colour)
    }

    pub fn colour_at(&self, x: usize, y: usize) -> Option<Rgb<u8>> {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.colour
    }

    pub fn value_state_at(&self, x: usize, y: usize) -> (u32, DomainElementState) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.value, p.state)
    }

    pub fn value_at(&self, x: usize, y: usize) -> u32 {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.value
    }

    pub fn state_origin_at(&self, x: usize, y: usize) -> (DomainElementState, f64, f64) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.state, p.origin_re, p.origin_im)
    }

    pub fn origin_at(&self, x: usize, y: usize) -> (f64, f64) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.origin_re, p.origin_im)
    }

    pub fn set_pixel_mandelbrot(
        &self,
        x: usize,
        y: usize,
        iterator: u32,
        quad: f64,
        state: DomainElementState,
        max: u32,
    ) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
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
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.quad = 1.0;
        p.quid = 1.0;
        p.state = state;
    }

    pub fn recalculate_pixels_states(&self) {
        println!("recalculate_pixels_states()");
        for y in 0..self.height_y {
            for x in 0..self.width_x {
                let mut mo_px = self.mo_px_at(x, y);
                let p = mo_px.as_mut().unwrap();
                p.past();
            }
        }
    }

    // all new elements are Active New
    // for wrapping, search only elements, which have some past well finished neighbors
    // previous calculation must be completed
    pub fn is_on_mandelbrot_horizon(&self, x: usize, y: usize) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..(neigh + 1) {
            for b in -neigh..(neigh + 1) {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if check_domain(xx, yy, self.width_x, self.height_y) {
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
        let chunk_size_x = self.width_x / 20;
        let chunk_size_y = self.height_y / 20;
        let mut values: Vec<u32> = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                values.push(self.chunk_value(
                    x * chunk_size_x,
                    (x + 1) * chunk_size_x,
                    y * chunk_size_y,
                    (y + 1) * chunk_size_y,
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
                sum += self.mo_px_at(x, y).as_ref().unwrap().value;
            }
        }
        sum
    }

    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    pub fn wrap(
        &self,
        origin_re: f64,
        origin_im: f64,
        rm: ResolutionMultiplier,
        plank: f64,
    ) -> Vec<[f64; 2]> {
        let mut ret = Vec::new();
        if rm == Square2 {
            let d = plank / 3.0;
            ret.push([origin_re + d, origin_im + d]);
            ret.push([origin_re - d, origin_im - d]);
            ret.push([origin_re - d, origin_im + d]);
            ret.push([origin_re + d, origin_im - d]);
        } else {
            let multiplier: f64 = resolve_multiplier(rm);
            let d: f64 = plank / multiplier;
            // multiplier was odd
            let half = ((multiplier - 1.0) / 2.0) as i32;
            // This fills the pixel with multiple points
            for x in -half..(half + 1) {
                for y in -half..(half + 1) {
                    if x != 0 || y != 0 {
                        ret.push([origin_re + (x as f64 * d), origin_im + (y as f64 * d)]);
                    }
                }
            }
        }
        ret
    }

    pub fn move_to_new_position(&self, x: usize, y: usize, area: &Area) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        // There was already zoom in, the new area is smaller
        if area.contains(p.origin_re, p.origin_im) {
            // Element didn't move out of the smaller area
            // new pixel position
            let (nx, ny) = area.point_to_pixel(p.origin_re, p.origin_im);
            if (x == nx) && (y == ny) {
                // insignificant move within the same pixel
            } else {
                // move px to new position
                self.move_px_to_new_position(nx, ny, p.to_owned());
                // clean old position
                *mo_px = None;
            }
        } else {
            // clean position of elements which moved beyond area edges
            *mo_px = None;
        }
    }

    // Verify if any neighbor px,py finished well, long or at least too short.
    // This method identifies deep black convergent elements of Mandelbrot set interior.
    // Don't do any calculation for those.
    pub fn all_neighbors_finished_bad(&self, x: usize, y: usize, is_mandelbrot: bool) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..(neigh + 1) {
            for b in -neigh..(neigh + 1) {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if (a != 0 || b != 0) && check_domain(xx, yy, self.width_x, self.height_y) {
                    let mo_px = self.mo_px_at(xx as usize, yy as usize);
                    if mo_px.is_some() {
                        let px = mo_px.as_ref().unwrap();
                        if is_mandelbrot {
                            if px.is_finished_too_long() || px.is_hibernated() {
                                return false;
                            }
                        } else {
                            if px.is_finished_success_any() || px.is_finished_too_short() {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn is_dynamic(&self) -> bool {
        self.data_type == Dynamic
    }
}

pub fn init(data_type: DataType, area: &Area) -> DataImage {
    let wx = area.data.lock().unwrap().width_x;
    let hy = area.data.lock().unwrap().height_y;
    DataImage {
        data_type,
        width_x: wx,
        height_y: hy,
        pixels: init_domain(area),
        paths: Arc::new(Mutex::new(Vec::new())),
        show_path: Mutex::new(Vec::new()),
    }
}

pub fn init_trivial() -> DataImage {
    DataImage {
        width_x: 1,
        height_y: 1,
        data_type: Static,
        pixels: init_pixels_trivial(),
        paths: Arc::new(Mutex::new(Vec::new())),
        show_path: Mutex::new(Vec::new()),
    }
}

fn init_domain(area: &Area) -> Vec<Vec<Mutex<Option<DataPx>>>> {
    println!("init_domain()");
    let mut vx = Vec::new();

    let wx = area.data.lock().unwrap().width_x;
    let hy = area.data.lock().unwrap().height_y;

    let res = area.screen_to_domain_re_copy();
    let ims = area.screen_to_domain_im_copy();

    for x in 0..wx {
        let mut vy = Vec::new();
        for y in 0..hy {
            let origin_re = res[x];
            let origin_im = ims[y];
            vy.push(Mutex::new(Some(data_px::init(origin_re, origin_im))));
        }
        vx.push(vy);
    }
    vx
}

fn init_pixels_trivial() -> Vec<Vec<Mutex<Option<DataPx>>>> {
    let mut vx = Vec::new();
    let mut vy = Vec::new();

    vy.push(Mutex::new(Some(data_px::init(0f64, 0f64))));
    vx.push(vy);

    vx
}

pub fn resolve_multiplier(rm: ResolutionMultiplier) -> f64 {
    match rm {
        ResolutionMultiplier::Single => 1.0,
        ResolutionMultiplier::Square3 => 3.0,
        ResolutionMultiplier::Square5 => 5.0,
        ResolutionMultiplier::Square9 => 9.0,
        ResolutionMultiplier::Square11 => 11.0,
        ResolutionMultiplier::Square51 => 51.0,
        ResolutionMultiplier::Square101 => 101.0,
        _ => 1.0,
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
    }
}

fn check_domain(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

#[cfg(test)]
mod tests {
    use crate::data_image::init_trivial;
    use crate::resolution_multiplier::ResolutionMultiplier::{
        Square101, Square11, Square3, Square5, Square51, Square9,
    };

    fn element_at(w: &Vec<[f64; 2]>, index: usize) -> (f64, f64) {
        let a = w.get(index).unwrap();
        (a[0], a[1])
    }

    #[test]
    fn test_wrap_3() {
        // prepare test
        let data = init_trivial();
        let area_plank = 1.0;

        // execute test
        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square3, area_plank);
        assert_eq!(w.len(), 8);
        let (re, im) = element_at(&w, 0);
        assert_eq!(re, -0.3333333333333333);
        assert_eq!(im, -0.3333333333333333);
    }

    #[test]
    fn test_wrap_5() {
        let data = init_trivial();
        let area_plank = 0.1;

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square5, area_plank);
        assert_eq!(w.len(), 24);
    }

    #[test]
    fn test_wrap_9() {
        let data = init_trivial();
        let area_plank = 0.1;

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square9, area_plank);
        assert_eq!(w.len(), 80);
    }

    #[test]
    fn test_wrap_11() {
        let data = init_trivial();
        let area_plank = 0.1;

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square11, area_plank);
        assert_eq!(w.len(), 120);
    }

    #[test]
    fn test_wrap_51() {
        let data = init_trivial();
        let area_plank = 0.1;

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square51, area_plank);
        assert_eq!(w.len(), 2600);
    }

    #[test]
    fn test_wrap_101() {
        let data = init_trivial();
        let area_plank = 0.1;

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square101, area_plank);
        assert_eq!(w.len(), 10_200);
    }
}
