use crate::constants::ZOOM;
use crate::fractal::FractalConfig;
use std::sync::RwLock;

/**
 * RxR Area on which the Fractal is calculated
 */
pub struct Area {
    data: RwLock<AreaData>,
}

/**
 * Mutable Area data.
 */
struct AreaData {
    width_xl: usize, // length [-0.5, 0.5] = 2 intervals for width_re = 1
    width_xp: usize, // points [-0.5, 0, 0.5] = 3 elements for width_re = 1
    width_xlf64: f64,
    height_yl: usize,
    height_yp: usize,
    height_ylf64: f64,
    width_re: f64,
    height_im: f64,
    width_half_xlf64: f64,
    height_half_ylf64: f64,
    numbers_re: Vec<f64>,
    numbers_im: Vec<f64>,
    center_re: f64,
    center_im: f64,
    border_low_re: f64,
    border_low_im: f64,
    border_high_re: f64,
    border_high_im: f64,
    plank: f64,
}

impl<'lt> Area {
    pub fn width_xl(&self) -> usize {
        self.data.read().unwrap().width_xl
    }

    pub fn width_xp(&self) -> usize {
        self.data.read().unwrap().width_xp
    }

    pub fn height_yl(&self) -> usize {
        self.data.read().unwrap().height_yl
    }
    pub fn height_yp(&self) -> usize {
        self.data.read().unwrap().height_yp
    }

    pub fn center_re(&self) -> f64 {
        self.data.read().unwrap().center_re
    }

    pub fn center_im(&self) -> f64 {
        self.data.read().unwrap().center_im
    }

    pub fn contains(&self, re: f64, im: f64) -> bool {
        let d = self.data.read().unwrap();
        re > d.border_low_re
            && re < d.border_high_re
            && im > d.border_low_im
            && im < d.border_high_im
    }

    /**
     * Maps pixels [x, y] to their center [re, im]
     */
    pub fn screen_to_domain_re_copy(&self) -> Vec<f64> {
        self.data.read().unwrap().numbers_re.clone()
    }

    /**
     * Maps pixels [x, y] to their center [re, im]
     */
    pub fn screen_to_domain_im_copy(&self) -> Vec<f64> {
        self.data.read().unwrap().numbers_im.clone()
    }

    /**
     * Check first, if element can convert, only then call this method
     */
    pub fn point_to_pixel(&self, re: f64, im: f64) -> (usize, usize) {
        let d = self.data.read().unwrap();

        let px = (d.width_xlf64 * (re - d.center_re) / d.width_re) + d.width_half_xlf64;
        let py = d.height_half_ylf64 - (d.height_ylf64 * (im - d.center_im) / d.height_im);

        (px as usize, py as usize)
    }

    pub fn zoom_in(&self) {
        self.zoom_in_by(ZOOM);
    }

    pub fn zoom_in_by(&self, zoom: f64) {
        println!("zoom_in()");
        let mut d = self.data.write().unwrap();

        d.width_re = d.width_re * zoom;
        d.height_im = d.width_re * ((d.height_yl as f64) / (d.width_xl as f64));

        d.plank = d.width_re / d.width_xl as f64;

        d.border_low_re = d.center_re - (d.width_re / 2.0);
        d.border_high_re = d.center_re + (d.width_re / 2.0);
        d.border_low_im = d.center_im - d.height_im / 2.0;
        d.border_high_im = d.center_im + (d.height_im / 2.0);

        d.width_xlf64 = d.width_xl as f64;
        d.height_ylf64 = d.height_yl as f64;
        d.width_half_xlf64 = d.width_xlf64 / 2.0;
        d.height_half_ylf64 = d.height_ylf64 / 2.0;

        d.numbers_re.clear();
        d.numbers_im.clear();

        // re
        for x in 0..d.width_xp {
            let v = d.border_low_re + (d.plank * x as f64);
            d.numbers_re.push(v);
        }

        // im
        for y in 0..d.height_yp {
            let v = d.border_high_im - (d.plank * y as f64);
            d.numbers_im.push(v);
        }
    }

    // TODO
    pub fn move_to_initial_coordinates(&self, init_target_re: f64, init_target_im: f64) {
        println!("move_to_initial_coordinates()");
        let mut d = self.data.write().unwrap();
        d.center_re = init_target_re;
        d.center_im = init_target_im;
    }

    pub fn plank(&self) -> f64 {
        self.data.read().unwrap().plank
    }

    // TODO
    pub fn move_target(&self, x: usize, y: usize) {
        let mut d = self.data.write().unwrap();
        println!("move_target({}, {})", x, y);
        let re = d.numbers_re[x];
        let im = d.numbers_im[y];
        println!("move_target({}, {})", re, im);
        d.center_re = re;
        d.center_im = im;

        d.border_low_re = d.center_re - d.width_re / 2.0;
        d.border_high_re = d.center_re + d.width_re / 2.0;
        d.border_low_im = d.center_im - d.height_im / 2.0;
        d.border_high_im = d.center_im + d.height_im / 2.0;

        d.width_xlf64 = d.width_xl as f64;
        d.height_ylf64 = d.height_yl as f64;
        d.width_half_xlf64 = d.width_xlf64 / 2.0;
        d.height_half_ylf64 = d.height_ylf64 / 2.0;

        d.numbers_re.clear();
        d.numbers_im.clear();

        for x in 0..d.width_xp {
            let v = d.border_low_re + (d.plank * x as f64);
            d.numbers_re.push(v);
        }
        for y in 0..d.height_yp {
            let v = d.border_high_im - (d.plank * y as f64);
            d.numbers_im.push(v);
        }
        println!("recalculated");
    }

    pub fn print_info(&self) {
        println!("print_info()");
        let d = self.data.read().unwrap();
        println!("width_re:       {:6}", d.width_re);
        println!("height_im:      {:6}", d.height_im);
        println!("border_low_re:  {:6}", d.border_low_re);
        println!("border_high_re: {:6}", d.border_high_re);
        println!("border_low_im:  {:6}", d.border_low_im);
        println!("border_high_im: {:6}", d.border_high_im);
        println!("(plank):        {:6}", d.plank);
    }

    pub fn print_more(&self) {
        println!("print_more()");
        let d = self.data.read().unwrap();
        println!("width_xlf64:       {:6}", d.width_xlf64);
        println!("width_half_xlf64:  {:6}", d.width_half_xlf64);
        println!("height_ylf64:      {:6}", d.height_ylf64);
        println!("height_half_ylf64: {:6}", d.height_half_ylf64);
    }
}

/**
 * coordinates [0, 0] are at the top left
 * width_x sets up length x
 * that is x + 1 points, considering both sides, left and right, with zero at the center
 */
pub fn init<'lt>(config: &FractalConfig) -> Area {
    println!("init()");
    let width_re = config.width_re;
    let center_re = config.center_re;
    let center_im = config.center_im;

    // e.g:. for length 2, three points [left, 0, right]
    let width_xl = config.width_xl;
    let width_xp = config.width_xp;
    let height_yl = config.height_yl;
    let height_yp = config.height_yp;

    let plank = width_re / (width_xl as f64);
    let height_im = width_re * ((height_yl as f64) / (width_xl as f64));
    let width_half_xl = width_xl / 2;
    let height_half_yl = height_yl / 2;
    let border_low_re = center_re - width_re / 2.0;
    let border_high_re = center_re + width_re / 2.0;
    let border_low_im = center_im - height_im / 2.0;
    let border_high_im = center_im + height_im / 2.0;

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width_xp {
        numbers_re.push(border_low_re + (plank * x as f64));
    }
    for y in 0..height_yp {
        numbers_im.push(border_high_im - (plank * y as f64));
    }

    let area_data = AreaData {
        width_xl,
        width_xp,
        width_xlf64: width_xl as f64,
        height_yl,
        height_yp,
        height_ylf64: height_yl as f64,
        width_re,
        height_im,
        width_half_xlf64: width_half_xl as f64,
        height_half_ylf64: height_half_yl as f64,
        numbers_re,
        numbers_im,
        center_re,
        center_im,
        border_low_re,
        border_low_im,
        border_high_re,
        border_high_im,
        plank,
    };
    Area {
        data: RwLock::new(area_data),
    }
}

#[cfg(test)]
mod tests {
    use crate::area::init;
    use crate::fractal;

    #[test]
    fn test_init() {
        let c = fractal::init_trivial_static_config();
        let a = init(&c);
        let d = a.data.read().unwrap();

        assert_eq!(d.border_low_re, -0.5);
        assert_eq!(d.border_high_re, 0.5);
        assert_eq!(d.border_low_im, -0.5);
        assert_eq!(d.border_high_im, 0.5);

        // coordinates [0, 0] are at the top left
        assert_eq!(*d.numbers_re.get(0).unwrap(), -0.5);
        assert_eq!(*d.numbers_re.get(1).unwrap(), 0.0);
        assert_eq!(*d.numbers_re.get(2).unwrap(), 0.5);

        assert_eq!(*d.numbers_im.get(0).unwrap(), 0.5);
        assert_eq!(*d.numbers_im.get(1).unwrap(), 0.0);
        assert_eq!(*d.numbers_im.get(2).unwrap(), -0.5);
    }

    #[test]
    fn test_contains() {
        let conf = fractal::init_trivial_static_config();
        let area = init(&conf);

        // top right
        assert_eq!(area.contains(0.4, 0.4), true);
        assert_eq!(area.contains(0.4, 0.6), false);
        assert_eq!(area.contains(0.6, 0.4), false);

        // bottom left
        assert_eq!(area.contains(-0.4, -0.4), true);
        assert_eq!(area.contains(-0.4, -0.6), false);
        assert_eq!(area.contains(-0.6, -0.4), false);

        // top left
        assert_eq!(area.contains(-0.4, 0.4), true);
        assert_eq!(area.contains(-0.6, 0.4), false);
        assert_eq!(area.contains(-0.4, 0.6), false);

        // bottom right
        assert_eq!(area.contains(0.4, -0.4), true);
        assert_eq!(area.contains(0.4, -0.6), false);
        assert_eq!(area.contains(0.6, -0.4), false);

        // precision
        assert_eq!(area.contains(0.4999999999999, -0.4), true);
        assert_eq!(area.contains(0.5000000000001, -0.4), false);
        assert_eq!(area.contains(0.5, -0.4), false);
    }

    #[test]
    fn test_point_to_pixel() {
        let conf = fractal::init_trivial_static_config();
        let area = init(&conf);

        let a = area.point_to_pixel(-0.5, 0.5);
        assert_eq!(a, (0, 0));
        let a = area.point_to_pixel(0.0, 0.5);
        assert_eq!(a, (1, 0));
        let a = area.point_to_pixel(0.5, 0.5);
        assert_eq!(a, (2, 0));

        let a = area.point_to_pixel(-0.5, 0.0);
        assert_eq!(a, (0, 1));
        let a = area.point_to_pixel(0.0, 0.0);
        assert_eq!(a, (1, 1));
        let a = area.point_to_pixel(0.5, 0.0);
        assert_eq!(a, (2, 1));

        let a = area.point_to_pixel(-0.5, -0.5);
        assert_eq!(a, (0, 2));
        let a = area.point_to_pixel(0.0, -0.5);
        assert_eq!(a, (1, 2));
        let a = area.point_to_pixel(0.5, -0.5);
        assert_eq!(a, (2, 2));
    }

    #[test]
    fn test_screen_to_domain_re_copy() {
        let conf = fractal::init_trivial_static_config();
        let area = init(&conf);

        let res = area.screen_to_domain_re_copy();

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], -0.5);
        assert_eq!(res[2], 0.5);
    }

    #[test]
    fn test_screen_to_domain_im_copy() {
        let conf = fractal::init_trivial_static_config();
        let area = init(&conf);

        let ims = area.screen_to_domain_im_copy();

        assert_eq!(ims.len(), 3);
        assert_eq!(ims[0], 0.5);
        assert_eq!(ims[2], -0.5);
    }

    #[test]
    fn test_print_info() {
        let c = fractal::init_trivial_dynamic_config(3);
        let a = init(&c);

        a.print_info();
    }

    #[test]
    fn test_print_more() {
        let c = fractal::init_trivial_dynamic_config(3);
        let a = init(&c);

        a.print_more();
    }

    #[test]
    fn test_zoom_in_by() {
        let c = fractal::init_trivial_dynamic_config(3);
        let a = init(&c);

        a.zoom_in_by(0.5);

        let d = a.data.read().unwrap();
        assert_eq!(d.center_re, 0.0);
        assert_eq!(d.center_im, 0.0);
        assert_eq!(d.width_re, 0.5);
        assert_eq!(d.height_im, 0.5);
        assert_eq!(d.border_low_re, -0.25);
        assert_eq!(d.border_high_re, 0.25);
        assert_eq!(d.border_low_im, -0.25);
        assert_eq!(d.border_high_im, 0.25);

        // TODO test numbers re & im
    }

    #[test]
    fn test_zoom_in() {
        let c = fractal::init_trivial_dynamic_config(3);
        let a = init(&c);

        a.zoom_in();

        let d = a.data.read().unwrap();
        assert_eq!(d.center_re, 0.0);
        assert_eq!(d.center_im, 0.0);
        assert_eq!(d.width_re, 0.98);
        assert_eq!(d.height_im, 0.98);

        assert_eq!(d.border_low_re, -0.49);
        assert_eq!(d.border_high_re, 0.49);
        assert_eq!(d.border_low_im, -0.49);
        assert_eq!(d.border_high_im, 0.49);

        assert_eq!(d.width_xlf64, 2.0);
        assert_eq!(d.height_ylf64, 2.0);
        assert_eq!(d.width_half_xlf64, 1.0);
        assert_eq!(d.height_half_ylf64, 1.0);

        // TODO test numbers re & im
    }
}
