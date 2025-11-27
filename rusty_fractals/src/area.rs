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
    width_x: usize,
    width_xf64: f64,
    height_y: usize,
    height_yf64: f64,
    width_re: f64,
    height_im: f64,
    width_half_xf64: f64,
    height_half_yf64: f64,
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
    pub fn width_x(&self) -> usize {
        self.data.read().unwrap().width_x
    }

    pub fn height_y(&self) -> usize {
        self.data.read().unwrap().height_y
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

        let px = (d.width_xf64 * (re - d.center_re) / d.width_re) + d.width_half_xf64;
        let py = (d.height_yf64 * (im - d.center_im) / d.height_im) + d.height_half_yf64;

        (px as usize, py as usize)
    }

    pub fn zoom_in(&self) {
        self.zoom_in_by(ZOOM);
    }

    pub fn zoom_in_by(&self, zoom: f64) {
        println!("zoom_in()");
        let mut d = self.data.write().unwrap();

        d.width_re = d.width_re * zoom;
        d.height_im = d.width_re * ((d.height_y as f64) / (d.width_x as f64));

        d.plank = d.width_re / d.width_x as f64;

        d.border_low_re = d.center_re - (d.width_re / 2.0);
        d.border_high_re = d.center_re + (d.width_re / 2.0);
        d.border_low_im = d.center_im - d.height_im / 2.0;
        d.border_high_im = d.center_im + (d.height_im / 2.0);

        d.width_xf64 = d.width_x as f64;
        d.height_yf64 = d.height_y as f64;
        d.width_half_xf64 = d.width_xf64 / 2.0;
        d.height_half_yf64 = d.height_yf64 / 2.0;

        d.numbers_re.clear();
        d.numbers_im.clear();

        // re
        for x in 0..d.width_x + 1 {
            let v = d.border_low_re + (d.plank * x as f64);
            d.numbers_re.push(v);
        }

        // im
        for y in 0..d.height_y + 1 {
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
        match self.data.write() {
            Ok(mut d) => {
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

                d.width_xf64 = d.width_x as f64;
                d.height_yf64 = d.height_y as f64;
                d.width_half_xf64 = d.width_xf64 / 2.0;
                d.height_half_yf64 = d.height_yf64 / 2.0;

                d.numbers_re.clear();
                d.numbers_im.clear();
                // use re, im in the center of each pixel
                let ph = d.plank / 2.0;
                for x in 0..d.width_x + 1 {
                    let v = d.border_low_re + (d.plank * x as f64) + ph;
                    d.numbers_re.push(v);
                }
                for y in 0..d.height_y + 1 {
                    let v = d.border_low_im + (d.plank * y as f64) + ph;
                    d.numbers_im.push(v);
                }
                println!("recalculated");
            }
            Err(e) => {
                println!("(): {}", e);
            }
        }
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
        println!("width_xf64:       {:6}", d.width_xf64);
        println!("width_half_xf64:  {:6}", d.width_half_xf64);
        println!("height_yf64:      {:6}", d.height_yf64);
        println!("height_half_yf64: {:6}", d.height_half_yf64);
    }
}

pub fn init<'lt>(config: &FractalConfig) -> Area {
    println!("init()");
    let width_re = config.width_re;
    let center_re = config.center_re;
    let center_im = config.center_im;
    let width_x = config.width_x;
    let height_y = config.height_y;

    let plank = width_re / (width_x as f64);
    let height_im = width_re * ((height_y as f64) / (width_x as f64));
    let width_half_x = width_x / 2;
    let height_half_y = height_y / 2;
    let border_low_re = center_re - width_re / 2.0;
    let border_high_re = center_re + width_re / 2.0;
    let border_low_im = center_im - height_im / 2.0;
    let border_high_im = center_im + height_im / 2.0;

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width_x + 1 {
        numbers_re.push(border_low_re + (plank * x as f64));
    }
    for y in 0..height_y + 1 {
        numbers_im.push(border_high_im - (plank * y as f64));
    }

    let area_data = AreaData {
        width_x,
        width_xf64: width_x as f64,
        height_y,
        height_yf64: height_y as f64,
        width_re,
        height_im,
        width_half_xf64: width_half_x as f64,
        height_half_yf64: height_half_y as f64,
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
        assert_eq!(*d.numbers_re.get(2).unwrap(), 0.5);
        assert_eq!(*d.numbers_im.get(0).unwrap(), 0.5);
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
    fn test_point_to_pixel_static() {
        let conf = fractal::init_trivial_static_config();
        let area = init(&conf);

        let a = area.point_to_pixel(0.4, 0.4);
        assert_eq!(a, (18, 18));

        let b = area.point_to_pixel(-0.5, 0.499999);
        assert_eq!(b, (0, 19)); // from 0 to 19 is 20 chunks x
    }

    #[test]
    fn test_point_to_pixel_dynamic() {
        let conf = fractal::init_trivial_dynamic_config();
        let area = init(&conf);

        let a = area.point_to_pixel(-0.225, -0.225);
        assert_eq!(a, (5, 5));

        let a = area.point_to_pixel(-0.225, 0.225);
        assert_eq!(a, (5, 14));

        let a = area.point_to_pixel(0.225, -0.225);
        assert_eq!(a, (14, 5));

        let a = area.point_to_pixel(0.225, 0.225);
        assert_eq!(a, (14, 14));

        let a = area.point_to_pixel(0.175, -0.175);
        assert_eq!(a, (13, 6));

        let a = area.point_to_pixel(0.4, 0.4);
        assert_eq!(a, (18, 18));

        let a = area.point_to_pixel(-0.5, 0.499999);
        assert_eq!(a, (0, 19)); // from 0 to 19 is 20 chunks x
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
        let c = fractal::init_trivial_dynamic_config();
        let a = init(&c);

        a.print_info();
    }

    #[test]
    fn test_print_more() {
        let c = fractal::init_trivial_dynamic_config();
        let a = init(&c);

        a.print_more();
    }

    #[test]
    fn test_zoom_in_by() {
        let c = fractal::init_trivial_dynamic_config();
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
    }

    #[test]
    fn test_zoom_in() {
        let c = fractal::init_trivial_dynamic_config();
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

        assert_eq!(d.width_xf64, 2.0);
        assert_eq!(d.height_yf64, 2.0);
        assert_eq!(d.width_half_xf64, 1.0);
        assert_eq!(d.height_half_yf64, 1.0);
    }
}
