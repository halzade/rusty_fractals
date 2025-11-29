use crate::area::Area;
use crate::constants::{MINIMUM_PATH_LENGTH, NEIGHBOURS};
use crate::data_px;
use crate::data_px::DataPx;
use crate::data_px3::DataPx3;
use crate::fractal::{FractalConfig, Optimizer};
use crate::pixel::Spectra::{Blue, Green, Red};
use crate::pixel_states::DomainElementState::{
    ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    HibernatedDeepBlack,
};
use crate::pixel_states::{
    is_finished_success_past, DomainElementState, ACTIVE_NEW, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST,
    FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK,
};
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Square2;
use image::Rgb;
use std::sync::{Arc, RwLock};
use ResolutionMultiplier::{Single, Square101, Square11, Square3, Square5, Square51, Square9};

pub struct DataImage {
    pub width_xl: usize,
    pub width_xp: usize,
    pub height_yl: usize,
    pub height_yp: usize,
    pub is_dynamic: bool,
    pub is_mandelbrot: bool,
    /*
     * static data for image
     */
    pub pixels: Vec<DataPx>,
    pub pixels3: Vec<DataPx3>,
    /*
     * dynamic data for zoom video
     * As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the tiny area.
     * Elements outside the tiny result_area are removed. Very short (calculation) paths are also removed.
     * All elements on paths are already inside result_area because they are filtered like that during the calculation.
     */
    pub paths: Arc<RwLock<Vec<Vec<[f64; 2]>>>>,
}

impl DataImage {
    pub fn color(&self, x: usize, y: usize, palette_color: Rgb<u8>) {
        self.px_at(x, y).set_c(palette_color);
    }

    pub fn color_r(&self, x: usize, y: usize, cr: u8) {
        self.px_at3(x, y).set_c(Red, cr);
    }
    pub fn color_g(&self, x: usize, y: usize, c: u8) {
        self.px_at3(x, y).set_c(Green, c);
    }
    pub fn color_b(&self, x: usize, y: usize, c: u8) {
        self.px_at3(x, y).set_c(Blue, c);
    }

    pub fn translate_one_path_to_point_grid_now(&self, path: Vec<[f64; 2]>, area: &Area) {
        for [re, im] in path {
            let (x, y) = area.point_to_pixel(re, im);
            self.add(x, y);
        }
    }

    pub fn translate_all_paths_to_point_grid(&self, area: &Area) {
        println!("translate_all_paths_to_point_grid()");
        let all = self.paths.read().unwrap().to_owned();
        for path in all {
            for [re, im] in path {
                let (x, y) = area.point_to_pixel(re, im);
                self.add(x, y);
            }
        }
    }

    /**
     * save any path
     * verify path length before saving
     */
    pub fn save_path(&self, path: Vec<[f64; 2]>) {
        self.paths.write().unwrap().push(path);
    }

    pub fn remove_elements_outside(&self, area: &Area) {
        println!("remove_elements_outside()");
        // all paths
        let all = self.paths.read().unwrap().to_owned();

        // remove elements outside Area
        for i in 0..all.len() {
            self.paths
                .write()
                .unwrap()
                .get_mut(i)
                .unwrap()
                .retain(|el| area.contains(el[0], el[1]))
        }

        // remove short paths
        self.paths
            .write()
            .unwrap()
            .retain(|path| path.len() as u32 > MINIMUM_PATH_LENGTH);
    }

    pub fn clear_all_px_data(&self) {
        for y in 0..self.height_yp {
            for x in 0..self.width_xp {
                self.px_at(x, y).set_v(0);
            }
        }
    }

    fn add(&self, x: usize, y: usize) {
        self.px_at(x, y).add_v1();
    }

    /**
     * [0,0] is at the top left
     */
    pub(crate) fn px_at(&self, x: usize, y: usize) -> &DataPx {
        self.pixels
            .get(x + y * self.width_xp)
            .expect(&format!("[{}, {}] out of bounds", x, y))
    }

    fn px_at3(&self, x: usize, y: usize) -> &DataPx3 {
        self.pixels3
            .get(x + y * self.width_xp)
            .expect(&format!("[{}, {}] out of bounds", x, y))
    }

    fn move_px_to_new_position(&self, x: usize, y: usize, px: &DataPx) {
        self.px_at(x, y).override_by(px);
    }

    fn kill_at(&self, x: usize, y: usize) {
        self.px_at(x, y).kill();
    }

    pub fn values_state_quad_color_at(
        &self,
        x: usize,
        y: usize,
    ) -> (u32, DomainElementState, f64, Option<Rgb<u8>>) {
        self.px_at(x, y).get_vsqc()
    }

    pub fn values_state_color_at(
        &self,
        x: usize,
        y: usize,
    ) -> (u32, DomainElementState, Option<Rgb<u8>>) {
        self.px_at(x, y).get_vsc()
    }

    pub fn state_at(&self, x: usize, y: usize) -> DomainElementState {
        self.px_at(x, y).get_s()
    }

    pub fn color_at(&self, x: usize, y: usize) -> Option<Rgb<u8>> {
        self.px_at(x, y).get_c()
    }

    pub fn value_state_at(&self, x: usize, y: usize) -> (u32, DomainElementState) {
        self.px_at(x, y).get_vs()
    }

    pub fn value_at(&self, x: usize, y: usize) -> u32 {
        self.px_at(x, y).get_v()
    }

    pub fn value_at3(&self, x: usize, y: usize) -> (u32, u32, u32) {
        self.px_at3(x, y).get_v3()
    }

    pub fn define_color_at3(&self, x: usize, y: usize) {
        self.px_at3(x, y).define_color3()
    }

    pub fn state_origin_at(&self, x: usize, y: usize) -> (DomainElementState, f64, f64) {
        self.px_at(x, y).get_sri()
    }

    pub fn origin_at(&self, x: usize, y: usize) -> (f64, f64) {
        self.px_at(x, y).get_ri()
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
        let value = {
            if iterator < 1 {
                1
            } else if iterator == max {
                0
            } else {
                iterator
            }
        };
        self.px_at(x, y).set_qsv(quad, state, value);
    }

    // for Nebula like fractals
    pub fn set_pixel_state(&self, x: usize, y: usize, state: DomainElementState) {
        self.px_at(x, y).set_qs(1.0, state);
    }

    pub fn recalculate_pixels_states(&self) {
        println!("recalculate_pixels_states()");
        for y in 0..self.height_yp {
            for x in 0..self.width_xp {
                self.px_at(x, y).past();
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
                if check_domain(xx, yy, self.width_xp, self.height_yp) {
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
        let chunk_size_x = self.width_xl / 20;
        let chunk_size_y = self.height_yl / 20;
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
                sum += self.px_at(x, y).get_v();
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
        let px = self.px_at(x, y);
        let (ore, oim) = px.get_ri();

        // There was already zoom in, the new area is smaller
        if area.contains(ore, oim) {
            // Element didn't move out of the smaller area
            // new pixel position
            let (nx, ny) = area.point_to_pixel(ore, oim);
            if (x == nx) && (y == ny) {
                // insignificant move within the same pixel
            } else {
                // move px to new position
                self.move_px_to_new_position(nx, ny, px);
            }
        } else {
            // clean position of elements which moved beyond area edges
            self.kill_at(x, y)
        }
    }

    pub fn fill_the_gaps(&self, area: &Area) -> (u32, u32) {
        let res = area.screen_to_domain_re_copy();
        let ims = area.screen_to_domain_im_copy();

        let mut c_moved = 0;
        let mut c_created = 0;

        for y in 0..self.height_yp {
            for x in 0..self.width_xp {
                let mo_px = self.px_at(x, y);
                if !mo_px.is_alive() {
                    c_created += 1;

                    let re = res[x];
                    let im = ims[y];

                    if self.all_neighbors_finished_bad(x, y) {
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        mo_px.reset(re, im, HibernatedDeepBlack);
                    } else {
                        mo_px.reset(re, im, ActiveNew);
                    }
                } else {
                    c_moved += 1;
                }
            }
        }
        (c_moved, c_created)
    }

    // Verify if any neighbor px,py finished well, long or at least too short.
    // This method identifies deep black convergent elements of Mandelbrot set interior.
    // Don't do any calculation for those.
    fn all_neighbors_finished_bad(&self, x: usize, y: usize) -> bool {
        let neigh = NEIGHBOURS as i32;

        for a in -neigh..(neigh + 1) {
            for b in -neigh..(neigh + 1) {
                let xx = x as i32 + a;
                let yy = y as i32 + b;

                if (a != 0 || b != 0) && check_domain(xx, yy, self.width_xp, self.height_yp) {
                    let px = self.px_at(xx as usize, yy as usize);

                    if self.is_mandelbrot {
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
        true
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }

    /**
     * expect 99 most
     */
    pub fn print_data_values(&self) {
        for y in 0..self.height_yp {
            for x in 0..self.width_xp {
                let v = self.px_at(x, y).get_v();
                print!("{:3}", v);
            }
            println!();
        }
    }

    pub fn set(&self, x: usize, y: usize, value: u32) {
        self.px_at(x, y).set_v(value);
    }
}

pub fn init(conf: &FractalConfig, area: &Area) -> DataImage {
    init_o(conf, area, None)
}

pub fn init_o(conf: &FractalConfig, area: &Area, oo: Option<Optimizer>) -> DataImage {
    DataImage {
        width_xl: area.width_xl(),
        width_xp: area.width_xp(),
        height_yl: area.height_yl(),
        height_yp: area.height_yp(),
        is_dynamic: conf.is_dynamic(),
        is_mandelbrot: conf.is_mandelbrot(),
        pixels: init_domain(area, oo),
        pixels3: Vec::new(),
        paths: Arc::new(RwLock::new(Vec::new())),
    }
}

/**
 * [0,0] is at the top left
 */
fn init_domain(area: &Area, oo: Option<Optimizer>) -> Vec<DataPx> {
    let mut ret = Vec::new();

    let res = area.screen_to_domain_re_copy();
    let ims = area.screen_to_domain_im_copy();

    let optimizer = oo.unwrap_or_else(Optimizer::trivial);

    for y in 0..area.height_yp() {
        for x in 0..area.width_xp() {
            let origin_re = res[x];
            let origin_im = ims[y];
            let state = (optimizer.initial_state_for)(origin_re, origin_im);

            ret.push(data_px::init(origin_re, origin_im, state));
        }
    }
    ret
}

pub fn resolve_multiplier(rm: ResolutionMultiplier) -> f64 {
    match rm {
        Single => 1.0,
        Square2 => 1.0, // special case
        Square3 => 3.0,
        Square5 => 5.0,
        Square9 => 9.0,
        Square11 => 11.0,
        Square51 => 51.0,
        Square101 => 101.0,
    }
}

pub fn color_for_state(state: DomainElementState) -> Rgb<u8> {
    match state {
        // most of the elements are going to be FinishedSuccessPast
        ActiveNew => ACTIVE_NEW,
        FinishedSuccess => FINISHED_SUCCESS,
        FinishedTooLong => FINISHED_TOO_LONG,
        FinishedTooShort => FINISHED_TOO_SHORT,
        FinishedSuccessPast => FINISHED_SUCCESS_PAST,
        HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
    }
}

fn check_domain(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

#[cfg(test)]
mod tests {
    use crate::area;
    use crate::data_image::{check_domain, color_for_state, init};
    use crate::fractal::{init_trivial_dynamic_config, FractalConfig};
    use crate::pixel_states::DomainElementState::ActiveNew;
    use crate::resolution_multiplier::ResolutionMultiplier::{
        Square101, Square11, Square3, Square5, Square51, Square9,
    };

    use crate::area::Area;
    use image::Pixel;
    use std::sync::LazyLock;

    static CONF: FractalConfig = init_trivial_dynamic_config(3);
    static AREA: LazyLock<Area> = LazyLock::new(|| area::init(&CONF));

    #[test]
    fn test_px_at() {
        let di = init(&CONF, &AREA);

        assert_eq!(di.px_at(0, 0).get_ri(), (-0.5, 0.5));
        assert_eq!(di.px_at(1, 0).get_ri(), (0.0, 0.5));
        assert_eq!(di.px_at(2, 0).get_ri(), (0.5, 0.5));

        assert_eq!(di.px_at(0, 1).get_ri(), (-0.5, 0.0));
        assert_eq!(di.px_at(1, 1).get_ri(), (0.0, 0.0));
        assert_eq!(di.px_at(2, 1).get_ri(), (0.5, 0.0));

        assert_eq!(di.px_at(0, 2).get_ri(), (-0.5, -0.5));
        assert_eq!(di.px_at(1, 2).get_ri(), (0.0, -0.5));
        assert_eq!(di.px_at(2, 2).get_ri(), (0.5, -0.5));
    }

    #[test]
    fn test_add() {
        let dynamic = init(&CONF, &AREA);

        dynamic.add(0, 0);
        let v = dynamic.px_at(0, 0);

        assert_eq!(v.get_v(), 1);
    }

    #[test]
    fn test_remove_elements_outside() {
        let dynamic = init(&CONF, &AREA);

        // test data
        // the last 2 pairs to should be removed
        let path = vec![
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [10.0, 0.0],
            [0.0, 10.0],
            [10.0, 10.0],
        ];
        // full path  should be removed
        let short = vec![[0.0, 0.0], [0.0, 0.0]];

        dynamic.save_path(path);
        dynamic.save_path(short);

        // execute test
        dynamic.remove_elements_outside(&AREA);

        // get test data
        let result_all = dynamic.paths.read().unwrap();
        assert_eq!(result_all.len(), 1);

        let remaining_path = result_all.get(0).unwrap();
        assert_eq!(remaining_path.len(), 8);
    }

    #[test]
    fn test_mo_px_at() {
        let data = init(&CONF, &AREA);

        assert_eq!(data.px_at(0, 0).get_s(), ActiveNew);
        assert_eq!(data.px_at(2, 2).get_s(), ActiveNew);
    }

    #[test]
    fn test_wrap_3() {
        // prepare test
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        // execute test
        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square3, area_plank);
        assert_eq!(w.len(), 8);
    }

    #[test]
    fn test_wrap_5() {
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square5, area_plank);
        assert_eq!(w.len(), 24);
    }

    #[test]
    fn test_wrap_9() {
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square9, area_plank);
        assert_eq!(w.len(), 80);
    }

    #[test]
    fn test_wrap_11() {
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square11, area_plank);
        assert_eq!(w.len(), 120);
    }

    #[test]
    fn test_wrap_51() {
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square51, area_plank);
        assert_eq!(w.len(), 2600);
    }

    #[test]
    fn test_wrap_101() {
        let data = init(&CONF, &AREA);
        let area_plank = AREA.plank();

        let (o_re, o_im) = data.origin_at(0, 0);
        let w = data.wrap(o_re, o_im, Square101, area_plank);
        assert_eq!(w.len(), 10_200);
    }

    #[test]
    fn test_color_for_state() {
        let red = color_for_state(ActiveNew).channels()[0];
        assert_eq!(red, 40);
    }

    #[test]
    fn test_check_domain() {
        assert_eq!(check_domain(0, 0, 0, 0), false);
        assert_eq!(check_domain(0, 0, 1, 1), true);
        assert_eq!(check_domain(-1, 0, 1, 1), false);
        assert_eq!(check_domain(0, -1, 1, 1), false);
        assert_eq!(check_domain(2, 0, 1, 1), false);
        assert_eq!(check_domain(0, 2, 1, 1), false);
    }

    #[test]
    fn test_print_data_values() {
        let di = init(&CONF, &AREA);
        di.print_data_values();
    }

    /**
     * :)
     */
    #[test]
    fn test_move_to_new_position() {
        let dc = init_trivial_dynamic_config(7);
        let ar = area::init(&dc);
        let di = init(&dc, &ar);

        di.set(2, 2, 13);
        di.set(4, 2, 14);
        di.set(2, 4, 15);
        di.set(4, 4, 16);

        ar.zoom_in_by(0.5);

        // move data from these positions to new coordinates
        di.move_to_new_position(2, 2, &ar);
        di.move_to_new_position(4, 2, &ar);
        di.move_to_new_position(2, 4, &ar);
        di.move_to_new_position(4, 4, &ar);

        // original position
        assert_eq!(di.px_at(2, 2).is_alive(), false);
        assert_eq!(di.px_at(4, 2).is_alive(), false);
        assert_eq!(di.px_at(2, 4).is_alive(), false);
        assert_eq!(di.px_at(4, 4).is_alive(), false);

        // moved after zoom in
        assert_eq!(di.px_at(1, 1).is_alive(), true);
        assert_eq!(di.px_at(5, 1).is_alive(), true);
        assert_eq!(di.px_at(1, 5).is_alive(), true);
        assert_eq!(di.px_at(5, 5).is_alive(), true);

        assert_eq!(di.px_at(1, 1).get_v(), 13);
        assert_eq!(di.px_at(5, 1).get_v(), 14);
        assert_eq!(di.px_at(1, 5).get_v(), 15);
        assert_eq!(di.px_at(5, 5).get_v(), 16);
    }

    #[test]
    fn test_move_px_to_new_position() {
        let di = init(&CONF, &AREA);
        di.set(2, 2, 11);

        di.move_px_to_new_position(1, 1, di.px_at(2, 2));

        assert_eq!(di.px_at(2, 2).is_alive(), false);
        assert_eq!(di.px_at(1, 1).is_alive(), true);
        assert_eq!(di.px_at(1, 1).get_v(), 11);
    }

    #[test]
    fn test_init_domain() {
        let di = init(&CONF, &AREA);
        assert_eq!(di.origin_at(0, 0), (-0.5, 0.5));
        assert_eq!(di.origin_at(1, 0), (0.0, 0.5));
        assert_eq!(di.origin_at(2, 0), (0.5, 0.5));

        assert_eq!(di.origin_at(0, 1), (-0.5, 0.0));
        assert_eq!(di.origin_at(1, 1), (0.0, 0.0));
        assert_eq!(di.origin_at(2, 1), (0.5, 0.0));

        assert_eq!(di.origin_at(0, 2), (-0.5, -0.5));
        assert_eq!(di.origin_at(1, 2), (0.0, -0.5));
        assert_eq!(di.origin_at(2, 2), (0.5, -0.5));

        assert_eq!(di.pixels.len(), 9);
    }
}
