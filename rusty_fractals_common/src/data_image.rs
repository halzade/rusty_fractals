use std::sync::{Arc, Mutex, MutexGuard};
use std::time::SystemTime;
use image::{Rgb, RgbImage};
use data_px::{active_new, hibernated_deep_black};
use crate::area::Area;
use crate::constants::{GRAY, MINIMUM_PATH_LENGTH, NEIGHBOURS, REFRESH_MS};
use crate::data_px;
use crate::data_px::DataPx;
use crate::pixel_states::{ACTIVE_NEW, DomainElementState, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK, is_finished_any, is_finished_success_past};
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort, HibernatedDeepBlack};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Square2;

pub struct DataImage {
    pub width: usize,
    pub height: usize,
    pub dynamic: bool,
    // static data for image
    pub pixels: Vec<Vec<Arc<Mutex<DataPx>>>>,
    // dynamic data for zoom video
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the the tiny area.
    // Elements outside of tiny result_area are removed. Very short (calculation) paths are also removed.
    // All elements on paths are already inside result_area because they are filtered like that during the calculation.
    pub paths: Arc<Mutex<Vec<Vec<[f64; 2]>>>>,
    // show one patch during calculation with pixel wrap
    pub show_path: Mutex<Vec<[f64; 2]>>,
    pub path_locker: Option<Arc<Mutex<SystemTime>>>,
}

static MAX_VALUE: Mutex<u32> = Mutex::new(0);

impl DataImage {
    pub fn colour(&self, x: usize, y: usize, palette_colour: Rgb<u8>) {
        let mut p = self.mpx_at(x, y);
        p.colour = Some(palette_colour);
    }

    pub fn image(&self, final_image: bool, area_o: Option<&Area>) -> Vec<u8> {
        return if final_image {
            self.image_result()
        } else {
            self.image_temp(area_o)
        };
    }

    pub fn image_temp(&self, area_o: Option<&Area>) -> Vec<u8> {
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
        match area_o {
            Some(area) => {
                let path = self.show_path.lock().unwrap();
                for p in path.as_slice() {
                    let (x, y) = area.point_to_pixel(p[0], p[1]);
                    image.put_pixel(x as u32, y as u32, GRAY);
                }
            }
            None => {}
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

    pub fn image_init(&self) -> Vec<u8> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                image.put_pixel(x as u32, y as u32, colour_for_state(ActiveNew));
            }
        }
        image.as_raw().clone()
    }

    fn set_show_path_maybe(&self, path: &Vec<[f64; 2]>, time_lock_o: &Option<Arc<Mutex<SystemTime>>>, max: u32) {
        match time_lock_o {
            Some(time_lock) => {
                let lo = time_lock.lock();
                match lo {
                    Ok(_) => {
                        let ms = SystemTime::now().duration_since(*lo.unwrap()).unwrap().as_millis();
                        if ms > REFRESH_MS - 2 {
                            // save path to show during recalculation with pixel wrap
                            let l = path.len();
                            // show only longer paths
                            if l > (max as f64 / 42.0) as usize {
                                *self.show_path.lock().unwrap() = path.clone();
                                *time_lock.lock().unwrap() = SystemTime::now();
                            }
                        }
                    }
                    Err(e) => {
                        println!("translate_path_to_point_grid() error: {}", e);
                    }
                }
            }
            None => {}
        }
    }

    pub fn translate_path_to_point_grid(&self, path: Vec<[f64; 2]>, area: &Area, time_lock_o: &Option<Arc<Mutex<SystemTime>>>, max: u32) {
        self.set_show_path_maybe(&path, time_lock_o, max);
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

    pub fn save_path(&self, path: Vec<[f64; 2]>, time_lock_o: &Option<Arc<Mutex<SystemTime>>>, max: u32) {
        self.set_show_path_maybe(&path, time_lock_o, max);
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
        let mut p = self.mpx_at(x, y);
        p.value += 1;
    }

    fn mpx_at(&self, x: usize, y: usize) -> MutexGuard<DataPx> {
        let arc_mutex_dpx = self.pixels.get(x).unwrap().get(y).unwrap();
        arc_mutex_dpx.lock().unwrap()
    }

    fn mpx_o_at(&self, x: usize, y: usize) -> Option<&Arc<Mutex<DataPx>>> {
        // let arc_mutex_dpx = self.pixels.get(x).unwrap().get(y).unwrap();
        let ovx = self.pixels.get(x);
        match ovx {
            None => { None }
            Some(vx) => {
                let ovy = vx.get(y);
                match ovy {
                    None => { None }
                    Some(vy) => {
                        return Some(vy);
                    }
                }
            }
        }
    }

    fn replace_px(&self, x: usize, y: usize, dp: DataPx) {
        let arc_mut_px = self.pixels.get(x).unwrap().get(y).unwrap();
        let mut _old = arc_mut_px.lock().unwrap();
        *_old = dp;
    }

    pub fn values_at(&self, x: usize, y: usize) -> (u32, DomainElementState, f64, f64, Option<Rgb<u8>>) {
        let p = self.mpx_at(x, y);
        (p.value, p.state, p.quad, p.quid, p.colour)
    }

    pub fn value_state_at(&self, x: usize, y: usize) -> (u32, DomainElementState) {
        let p = self.mpx_at(x, y);
        (p.value, p.state)
    }

    pub fn value_at(&self, x: usize, y: usize) -> u32 {
        let p = self.mpx_at(x, y);
        p.value
    }

    pub fn state_origin_at(&self, x: usize, y: usize) -> (DomainElementState, f64, f64) {
        let p = self.mpx_at(x, y);
        (p.state, p.origin_re, p.origin_im)
    }

    pub fn origin_at(&self, x: usize, y: usize) -> (f64, f64) {
        let p = self.mpx_at(x, y);
        (p.origin_re, p.origin_im)
    }

    pub fn set_pixel_mandelbrot(&self, x: usize, y: usize, iterator: u32, quad: f64, state: DomainElementState, max: u32) {
        let mut p = self.mpx_at(x, y);
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
        let mut p = self.mpx_at(x, y);
        p.quad = 1.0;
        p.quid = 1.0;
        p.state = state;
    }

    pub fn recalculate_pixels_states(&self) {
        println!("recalculate_pixels_states()");
        for y in 0..self.height {
            for x in 0..self.width {
                self.mpx_at(x, y).past();
            }
        }
    }

    pub fn clear_screen_pixel_values(&self) {
        println!("clear_screen_pixel_values()");
        for y in 0..self.height {
            for x in 0..self.width {
                self.mpx_at(x, y).clear_screen_values();
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
                sum += self.mpx_at(x, y).value;
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
    pub fn recalculate_pixels_positions_for_next_calculation(&mut self, area: &Area) {
        println!("recalculate_pixels_positions_for_next_calculation()");
        // Scan all elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, subsequent calculations will be skipped.
        // 1. save references for elements moving to now positions
        let mut c_to_move = 0;
        let mut c_to_drop = 0;
        let mut px_for_move: Vec<DataPx> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let px = self.mpx_at(x as usize, y as usize);
                // There was already zoom in, the new area is smaller
                if area.contains(px.origin_re, px.origin_im) {
                    // Element did not move out of the zoomed in area
                    c_to_move += 1;
                    px_for_move.push(px.clone());
                } else {
                    c_to_drop += 1;
                }
            }
        }
        // If there is a conflict, two or more points moved to same pixel, don't drop conflicts around, because that would create really bad mess,
        // Simply calculate new elements in the next calculation iteration, it gives better results without complications.
        let mut all_some_none: Vec<Vec<Mutex<Option<DataPx>>>> = Vec::new();
        for _ in 0..self.width {
            let mut vy = Vec::new();
            for _ in 0..self.height {
                vy.push(Mutex::new(None));
            }
            all_some_none.push(vy);
        }
        // 2. calculate position for moving elements and resolve conflicts
        let mut c_conflict = 0;
        for px in px_for_move {
            // translate [px,py] to [re,im]
            let (x, y) = area.point_to_pixel(px.origin_re, px.origin_im);
            let filled_already_o = tpx_at(&all_some_none, x, y).lock().unwrap().clone();
            match filled_already_o {
                // conflict
                Some(conf) => {
                    c_conflict += 1;
                    if conf.has_worse_state_then(&px) {
                        // Replace by element with better state
                        // Better to delete the other one, then to drop it to other empty pixel.
                        // That would cause problem with optimization, better calculate new and shiny pixel
                        set_tpx_at(&all_some_none, x, y, px.clone());
                    }
                }
                // Excellent, there is no conflict
                None => {
                    set_tpx_at(&all_some_none, x, y, px.clone());
                }
            }
        }
        // 3. drop moved elements to new positions and create new elements on positions where nothing was moved to
        let mut c_moved = 0;
        let mut c_created = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let mg_el_co = tpx_at(&all_some_none, x, y).lock().unwrap();
                match &*mg_el_co {
                    Some(cop) => {
                        self.replace_px(x, y, cop.clone());
                        c_moved += 1;
                    }
                    None => {
                        c_created += 1;
                        let re = area.screen_to_domain_re(x);
                        let im = area.screen_to_domain_im(y);
                        if self.all_neighbors_finished_too_long(x, y) {
                            // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                            self.replace_px(x, y, hibernated_deep_black(re, im));
                        } else {
                            self.replace_px(x, y, active_new(re, im));
                        }
                    }
                }
            }
        }
        println!("to move:   {}", c_to_move);
        println!("to drop:   {}", c_to_drop);
        println!("conflicts: {}", c_conflict);
        println!("moved:     {}", c_moved);
        println!("created:   {}", c_created);
        assert!(c_to_move > 0);
        assert!(c_to_drop > 0);
        assert!(c_moved > 0);
        assert!(c_created > 0);
    }

    // Verify if any neighbor px,py finished well, long or at least too short.
    // This method identifies deep black convergent elements of Mandelbrot set interior.
    // Don't do any calculation for those.
    fn all_neighbors_finished_too_long(&mut self, x: usize, y: usize) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..(neigh + 1) {
            for b in -neigh..(neigh + 1) {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if check_domain(xx, yy, self.width, self.height) {
                    let oel = self.mpx_o_at(xx as usize, yy as usize);
                    match oel {
                        None => {
                            // no element moved to this position
                        }
                        Some(ael) => {
                            let el = ael.lock().unwrap();
                            if el.is_finished_success_any() || el.is_finished_too_short() {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

fn tpx_at(vec: &Vec<Vec<Mutex<Option<DataPx>>>>, x: usize, y: usize) -> &Mutex<Option<DataPx>> {
    vec.get(x).unwrap().get(y).unwrap()
}

fn set_tpx_at(vec: &Vec<Vec<Mutex<Option<DataPx>>>>, x: usize, y: usize, px_ref: DataPx) {
    let vy = vec.get(x).unwrap().to_owned();
    vy[y].lock().unwrap().replace(px_ref.clone());
}

pub fn init_data_image(area: &Area, lock: Option<Arc<Mutex<SystemTime>>>) -> DataImage {
    init(area, lock, false)
}

pub fn init_data_video(area: &Area, lock: Option<Arc<Mutex<SystemTime>>>) -> DataImage {
    init(area, lock, true)
}

pub fn init(area: &Area, lock: Option<Arc<Mutex<SystemTime>>>, dynamic: bool) -> DataImage {
    DataImage {
        width: area.width_x,
        height: area.height_y,
        dynamic,
        pixels: init_domain(area),
        paths: Arc::new(Mutex::new(Vec::new())),
        show_path: Mutex::new(Vec::new()),
        path_locker: lock,
    }
}

fn init_domain(area: &Area) -> Vec<Vec<Arc<Mutex<DataPx>>>> {
    let mut vx = Vec::new();
    for x in 0..area.width_x {
        let mut vy = Vec::new();
        for y in 0..area.height_y {
            let origin_re = area.screen_to_domain_re(x);
            let origin_im = area.screen_to_domain_im(y);
            vy.push(Arc::new(Mutex::new(data_px::init(origin_re, origin_im))));
        }
        vx.push(vy);
    }
    vx
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
    use crate::data_image::{DataImage, init_data_image};
    use crate::resolution_multiplier::ResolutionMultiplier::{Square101, Square11, Square3, Square5, Square51, Square9};

    fn init() -> (Area, DataImage) {
        let area_config = AreaConfig { width_re: 1.0, center_re: 0.0, center_im: 0.0, width_x: 10, height_y: 10 };
        let area = area::init(&area_config);
        let data = init_data_image(&area, None);
        (area, data)
    }


    fn at(w: &Vec<[f64; 2]>, index: usize) -> (f64, f64) {
        let a = w.get(index).unwrap();
        (a[0], a[1])
    }

    #[test]
    fn test_wrap_3() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square3, &area);
        assert_eq!(w.len(), 8);
        let (re, im) = at(&w, 0);
        assert_eq!(re, -0.3333333333333333);
        assert_eq!(im, -0.23333333333333328);
        assert_eq!(o_re - re, 0.033333333333333326);
        assert_eq!(o_im - im, 0.033333333333333326);
    }

    #[test]
    fn test_wrap_5() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square5, &area);
        assert_eq!(w.len(), 24);
    }

    #[test]
    fn test_wrap_9() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square9, &area);
        assert_eq!(w.len(), 80);
    }


    #[test]
    fn test_wrap_11() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(7, 8);
        let w = data.wrap(o_re, o_im, Square11, &area);
        assert_eq!(w.len(), 120);
    }

    #[test]
    fn test_wrap_51() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square51, &area);
        assert_eq!(w.len(), 2600);
    }

    #[test]
    fn test_wrap_101() {
        let (area, data) = init();
        let (o_re, o_im) = data.origin_at(2, 3);
        let w = data.wrap(o_re, o_im, Square101, &area);
        assert_eq!(w.len(), 10_200);
    }
}