use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};
use image::{Rgb, RgbImage};
use data_px::{active_new, hibernated_deep_black};
use crate::area::Area;
use crate::constants::{GRAY, MINIMUM_PATH_LENGTH, NEIGHBOURS};
use crate::data_image::DataType::{Dynamic, Static};
use crate::data_px;
use crate::data_px::DataPx;
use crate::fractal_log::now;
use crate::pixel_states::{ACTIVE_NEW, DomainElementState, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK, is_finished_any, is_finished_success_past};
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort, HibernatedDeepBlack};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Square2;

pub struct DataImage<'lt> {
    pub width: usize,
    pub height: usize,
    pub data_type: DataType,
    // static data for image
    pub pixels: Vec<Vec<Mutex<Option<DataPx>>>>,
    // dynamic data for zoom video
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the the tiny area.
    // Elements outside of tiny result_area are removed. Very short (calculation) paths are also removed.
    // All elements on paths are already inside result_area because they are filtered like that during the calculation.
    pub paths: Arc<Mutex<Vec<Vec<[f64; 2]>>>>,
    // show one patch during calculation with pixel wrap
    pub show_path: Mutex<Vec<[f64; 2]>>,
    pub show_path_update: Mutex<bool>,
    phantom: PhantomData<&'lt bool>,
}

static MAX_VALUE: Mutex<u32> = Mutex::new(0);

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    Dynamic,
    Static,
}

impl DataImage<'_> {
    pub fn colour(&self, x: usize, y: usize, palette_colour: Rgb<u8>) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.colour = Some(palette_colour);
    }

    pub fn image_temp(&self, with_path: bool, area_o: Option<&Area>) -> Vec<u8> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let (value, state, _, _, colour_index_o) = self.values_at(x, y);
                let colour: Rgb<u8>;
                if !is_finished_any(state) {
                    colour = colour_for_state(state);
                } else {
                    match colour_index_o {
                        Some(pixel_colour) => {
                            colour = pixel_colour;
                        }
                        None => {
                            let mut mv = MAX_VALUE.lock().unwrap();
                            if value > *mv {
                                *mv = value;
                            }
                            // make color (3x) brighter
                            let mut cv = ((value * 3) as f64 / *mv as f64) as f64 * 255.0;
                            if cv > 255.0 {
                                cv = 255.0;
                            }
                            let c = cv as u8;
                            colour = Rgb([c, c, c]);
                        }
                    }
                }
                image.put_pixel(x as u32, y as u32, colour);
            }
        }
        if with_path {
            let path = self.show_path.lock().unwrap();
            let area = area_o.unwrap();
            for p in path.as_slice() {
                let (x, y) = area.point_to_pixel(p[0], p[1]);
                image.put_pixel(x as u32, y as u32, GRAY);
            }
        }
        image.as_raw().clone()
    }

    pub fn image_result(&self) -> Vec<u8> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let (_, _, _, _, colour_index_o) = self.values_at(x, y);
                match colour_index_o {
                    None => {
                        panic!();
                    }
                    Some(ci) => {
                        image.put_pixel(x as u32, y as u32, ci);
                    }
                }
            }
        }
        image.as_raw().clone()
    }

    // save path to show during recalculation with pixel wrap
    fn set_show_path(&self, path: &Vec<[f64; 2]>) {
        println!("set_show_path()");
        *self.show_path.lock().unwrap() = path.clone();
    }

    pub fn translate_path_to_point_grid(&self, path: Vec<[f64; 2]>, area: &Area, is_wrap: bool) {
        if is_wrap {
            if std::mem::replace(&mut self.show_path_update.lock().unwrap(), false) {
                self.set_show_path(&path);
            }
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
            if std::mem::replace(&mut self.show_path_update.lock().unwrap(), false) {
                self.set_show_path(&path);
            }
        }
        self.paths.lock().unwrap().push(path);
    }

    pub fn remove_elements_outside(&self, area: &Area) {
        println!("remove_elements_outside()");
        let all = self.paths.lock().unwrap().to_owned();
        for mut path in all {
            path.retain(|&el| area.contains(el[0], el[1]));
        }
        self.paths.lock().unwrap().retain(|path| path.len() as u32 > MINIMUM_PATH_LENGTH);
    }

    fn add(&self, x: usize, y: usize) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        p.value += 1;
    }

    fn mo_px_at(&self, x: usize, y: usize) -> MutexGuard<Option<DataPx>> {
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

    pub fn values_at(&self, x: usize, y: usize) -> (u32, DomainElementState, f64, f64, Option<Rgb<u8>>) {
        let mut mo_px = self.mo_px_at(x, y);
        let p = mo_px.as_mut().unwrap();
        (p.value, p.state, p.quad, p.quid, p.colour)
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

    pub fn set_pixel_mandelbrot(&self, x: usize, y: usize, iterator: u32, quad: f64, state: DomainElementState, max: u32) {
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
        for y in 0..self.height {
            for x in 0..self.width {
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
                sum += self.mo_px_at(x, y).as_ref().unwrap().value;
            }
        }
        sum
    }

    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    pub fn wrap(&self, origin_re: f64, origin_im: f64, rm: ResolutionMultiplier, plank: f64) -> Vec<[f64; 2]> {
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

    // This is called after calculation finished, zoom was called and new area measures recalculated
    pub fn recalculate_pixels_positions_for_next_calculation(&mut self, area: &Area, is_mandelbrot: bool) {
        println!("recalculate_pixels_positions_for_next_calculation()");
        // Scan all elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, subsequent calculations will be skipped.
        let (cx, cy) = area.point_to_pixel(area.center_re, area.center_im);
        now("1. move top left to center");
        for y in 0..cy {
            for x in 0..cx {
                self.move_to_new_position(x, y, area);
            }
        }
        now("2. move top right to center");
        for y in 0..cy {
            for x in (cx..self.width).rev() {
                self.move_to_new_position(x, y, area);
            }
        }
        now("3. move bottom left to center");
        for y in (cy..self.height).rev() {
            for x in 0..cx {
                self.move_to_new_position(x, y, area);
            }
        }
        now("4. move bottom right to center");
        for y in (cy..self.height).rev() {
            for x in (cx..self.width).rev() {
                self.move_to_new_position(x, y, area);
            }
        }
        // Create new elements on positions where no px moved to
        now("fill empty places");
        let mut c_moved = 0;
        let mut c_created = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut mo_px = self.mo_px_at(x as usize, y as usize);
                if mo_px.is_none() {
                    c_created += 1;
                    let re = area.screen_to_domain_re(x);
                    let im = area.screen_to_domain_im(y);
                    if self.all_neighbors_finished_bad(x, y, is_mandelbrot) {
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        mo_px.replace(hibernated_deep_black(re, im));
                    } else {
                        mo_px.replace(active_new(re, im));
                    }
                } else {
                    c_moved += 1;
                }
            }
        }
        println!("moved:     {}", c_moved);
        println!("created:   {}", c_created);
        assert!(c_moved > 0);
        assert!(c_created > 0);
    }

    fn move_to_new_position(&self, x: usize, y: usize, area: &Area) {
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
    fn all_neighbors_finished_bad(&self, x: usize, y: usize, is_mandelbrot: bool) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..(neigh + 1) {
            for b in -neigh..(neigh + 1) {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if (a != 0 || b != 0) && check_domain(xx, yy, self.width, self.height) {
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

pub fn init<'lt>(data_type: DataType, area: &Area) -> DataImage<'lt> {
    DataImage {
        width: area.width_x,
        height: area.height_y,
        data_type,
        pixels: init_domain(area),
        paths: Arc::new(Mutex::new(Vec::new())),
        show_path: Mutex::new(Vec::new()),
        show_path_update: Mutex::new(false),
        phantom: PhantomData::default(),
    }
}

pub fn init_none<'lt>() -> DataImage<'lt> {
    DataImage {
        width: 1,
        height: 1,
        data_type: Static,
        pixels: Vec::new(),
        paths: Arc::new(Mutex::new(Vec::new())),
        show_path: Mutex::new(Vec::new()),
        show_path_update: Mutex::new(false),
        phantom: PhantomData::default(),
    }
}

fn init_domain(area: &Area) -> Vec<Vec<Mutex<Option<DataPx>>>> {
    let mut vx = Vec::new();
    for x in 0..area.width_x {
        let mut vy = Vec::new();
        for y in 0..area.height_y {
            let origin_re = area.screen_to_domain_re(x);
            let origin_im = area.screen_to_domain_im(y);
            vy.push(Mutex::new(Some(data_px::init(origin_re, origin_im))));
        }
        vx.push(vy);
    }
    vx
}

pub fn image_init(width: usize, height: usize) -> Vec<u8> {
    let mut image = RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            image.put_pixel(x as u32, y as u32, colour_for_state(ActiveNew));
        }
    }
    image.as_raw().clone()
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
    return match rm {
        ResolutionMultiplier::Single => 1.0,
        ResolutionMultiplier::Square3 => 3.0,
        ResolutionMultiplier::Square5 => 5.0,
        ResolutionMultiplier::Square9 => 9.0,
        ResolutionMultiplier::Square11 => 11.0,
        ResolutionMultiplier::Square51 => 51.0,
        ResolutionMultiplier::Square101 => 101.0,
        _ => 1.0
    };
}

pub fn colour_for_state(state: DomainElementState) -> Rgb<u8> {
    return match state {
        // most of the elements are going to be FinishedSuccessPast
        FinishedSuccessPast => FINISHED_SUCCESS_PAST,
        HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
        ActiveNew => ACTIVE_NEW,
        FinishedSuccess => FINISHED_SUCCESS,
        FinishedTooShort => FINISHED_TOO_SHORT,
        FinishedTooLong => FINISHED_TOO_LONG,
    };
}

fn check_domain(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

#[cfg(test)]
mod tests {
    use crate::area;
    use crate::area::{Area, AreaConfig};
    use crate::data_image::{DataImage, init};
    use crate::data_image::DataType::Static;
    use crate::resolution_multiplier::ResolutionMultiplier::{Square101, Square11, Square3, Square5, Square51, Square9};

    fn init_test<'lt>() -> (Area<'lt>, DataImage<'lt>) {
        let area_config = AreaConfig { width_re: 1.0, center_re: 0.0, center_im: 0.0, width_x: 10, height_y: 10 };
        let area = area::init(area_config);
        let data = init(Static, &area);
        (area, data)
    }


    fn at(w: &Vec<[f64; 2]>, index: usize) -> (f64, f64) {
        let a = w.get(index).unwrap();
        (a[0], a[1])
    }

    #[test]
    fn test_wrap_3() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square3, area.plank());
        assert_eq!(w.len(), 8);
        let (re, im) = at(&w, 0);
        assert_eq!(re, -0.3333333333333333);
        assert_eq!(im, -0.23333333333333328);
        assert_eq!(o_re - re, 0.033333333333333326);
        assert_eq!(o_im - im, 0.033333333333333326);
    }

    #[test]
    fn test_wrap_5() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square5, area.plank());
        assert_eq!(w.len(), 24);
    }

    #[test]
    fn test_wrap_9() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square9, area.plank());
        assert_eq!(w.len(), 80);
    }


    #[test]
    fn test_wrap_11() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(7, 8);
        let w = data.wrap(o_re, o_im, Square11, area.plank());
        assert_eq!(w.len(), 120);
    }

    #[test]
    fn test_wrap_51() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square51, area.plank());
        assert_eq!(w.len(), 2600);
    }

    #[test]
    fn test_wrap_101() {
        let (area, data) = init_test();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square101, area.plank());
        assert_eq!(w.len(), 10_200);
    }
}
