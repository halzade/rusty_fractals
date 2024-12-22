use crate::constants::ZOOM;
use crate::fractal::FractalConfig;
use std::sync::Mutex;

/**
 * RxR Area on which the Fractal is calculated
 */
pub struct Area {
    pub data: Mutex<AreaData>,
}

/**
 * Mutable Area data.
 */
pub struct AreaData {
    pub width_x: usize,
    pub width_xf64: f64,
    pub height_y: usize,
    pub height_yf64: f64,
    pub width_re: f64,
    pub height_im: f64,
    pub width_half_x: usize,
    pub width_half_xf64: f64,
    pub height_half_y: usize,
    pub height_half_yf64: f64,
    pub numbers_re: Vec<f64>,
    pub numbers_im: Vec<f64>,
    pub center_re: f64,
    pub center_im: f64,
    border_low_re: f64,
    border_low_im: f64,
    border_high_re: f64,
    border_high_im: f64,
    plank: f64,
}

pub struct AreaDataCopy {
    pub center_re: f64,
    pub center_im: f64,
    pub width_re: f64,
    pub width_xf64: f64,
    pub width_half_xf64: f64,
    pub height_im: f64,
    pub height_yf64: f64,
    pub height_half_yf64: f64,
}

impl AreaDataCopy {
    pub fn point_to_pixel(&self, re: f64, im: f64) -> (usize, usize) {
        let px = (self.width_xf64 * (re - self.center_re) / self.width_re) + self.width_half_xf64;
        let py =
            (self.height_yf64 * (im - self.center_im) / self.height_im) + self.height_half_yf64;
        (px as usize, py as usize)
    }
}

impl<'lt> Area {
    // TODO faster
    pub fn contains(&self, re: f64, im: f64) -> bool {
        match self.data.lock() {
            Ok(d) => {
                re > d.border_low_re
                    && re < d.border_high_re
                    && im > d.border_low_im
                    && im < d.border_high_im
            }
            Err(e) => {
                println!("Area.contains(): {}", e);
                panic!()
            }
        }
    }

    /**
     * Maps pixels [x, y] to their center [re, im]
     */
    pub fn screen_to_domain_re_copy(&self) -> Vec<f64> {
        println!("screen_to_domain_re_copy()");
        match self.data.lock() {
            Ok(d) => d.numbers_re.clone(),
            Err(e) => {
                println!("(): {}", e);
                panic!()
            }
        }
    }

    /**
     * Maps pixels [x, y] to their center [re, im]
     */
    pub fn screen_to_domain_im_copy(&self) -> Vec<f64> {
        match self.data.lock() {
            Ok(d) => d.numbers_im.clone(),
            Err(e) => {
                println!("(): {}", e);
                panic!()
            }
        }
    }

    /**
     * Check first, if element can convert, only then call this method
     */
    pub fn point_to_pixel(&self, re: f64, im: f64) -> (usize, usize) {
        match self.data.lock() {
            Ok(d) => {
                let px = (d.width_xf64 * (re - d.center_re) / d.width_re) + d.width_half_xf64;
                let py = (d.height_yf64 * (im - d.center_im) / d.height_im) + d.height_half_yf64;
                (px as usize, py as usize)
            }
            Err(e) => {
                println!("(): {}", e);
                panic!()
            }
        }
    }

    /**
     * copy area data for point_to_pixel method
     * element's re, im coordinates can be converted to x,y because they were verified during path calculation
     */
    pub fn copy_data(&self) -> AreaDataCopy {
        let area = &self.data.lock().unwrap();
        AreaDataCopy {
            center_re: area.center_re,
            center_im: area.center_im,
            width_re: area.width_re,
            width_xf64: area.width_xf64,
            width_half_xf64: area.width_half_xf64,
            height_im: area.height_im,
            height_yf64: area.height_yf64,
            height_half_yf64: area.height_half_yf64,
        }
    }

    pub fn zoom_in(&self) {
        println!("zoom_in()");
        match self.data.lock() {
            Ok(mut d) => {
                d.width_re = d.width_re * ZOOM;
                d.height_im = d.width_re * ((d.height_y as f64) / (d.width_x as f64));

                d.plank = d.width_re / d.width_x as f64;

                d.border_low_re = d.center_re - d.width_re / 2.0;
                d.border_high_re = d.center_re + d.width_re / 2.0 - d.plank;
                d.border_low_im = d.center_im - d.height_im / 2.0;
                d.border_high_im = d.center_im + d.height_im / 2.0 - d.plank;

                d.numbers_re.clear();
                d.numbers_im.clear();

                // use re, im in the center of each pixel
                let ph = d.plank / 2.0;

                // re
                for x in 0..d.width_x {
                    let v = d.border_low_re + (d.plank * x as f64) + ph;
                    d.numbers_re.push(v);
                }

                // im
                for y in 0..d.height_y {
                    let v = d.border_low_im + (d.plank * y as f64) + ph;
                    d.numbers_im.push(v);
                }
            }
            Err(e) => {
                println!("(): {}", e);
            }
        }
    }

    pub fn move_to_initial_coordinates(&self, init_target_re: f64, init_target_im: f64) {
        println!("move_to_initial_coordinates()");
        match self.data.lock() {
            Ok(mut d) => {
                d.center_re = init_target_re;
                d.center_im = init_target_im;
            }
            Err(e) => {
                println!("Area.move_to_initial_coordinates(): {}", e);
            }
        }
    }

    pub fn plank(&self) -> f64 {
        self.data.lock().unwrap().plank
    }

    pub fn move_target(&self, x: usize, y: usize) {
        match self.data.lock() {
            Ok(mut d) => {
                println!("move_target({}, {})", x, y);
                let re = d.numbers_re[x];
                let im = d.numbers_im[y];
                println!("move_target({}, {})", re, im);
                d.center_re = re;
                d.center_im = im;
                d.border_low_re = d.center_re - d.width_re / 2.0;
                d.border_high_re = d.center_re + d.width_re / 2.0 - d.plank;
                d.border_low_im = d.center_im - d.height_im / 2.0;
                d.border_high_im = d.center_im + d.height_im / 2.0 - d.plank;
                d.numbers_re.clear();
                d.numbers_im.clear();
                // use re, im in the center of each pixel
                let ph = d.plank / 2.0;
                for x in 0..d.width_x {
                    let v = d.border_low_re + (d.plank * x as f64) + ph;
                    d.numbers_re.push(v);
                }
                for y in 0..d.height_y {
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
}

pub fn init<'lt>(config: &FractalConfig) -> Area {
    println!("init()");
    let width_re = config.width_re;
    let center_re = config.center_re;
    let center_im = config.center_im;
    let width_x = config.width_x;
    let height_y = config.height_y;

    let plank = width_re / (width_x as f64);
    // center everything at the middle of pixel really
    let plank_half = plank / 2.0;
    let height_im = width_re * ((height_y as f64) / (width_x as f64));
    let width_half_x = width_x / 2;
    let height_half_y = height_y / 2;
    let border_low_re = center_re - width_re / 2.0;
    let border_high_re = center_re + width_re / 2.0;
    let border_low_im = center_im - height_im / 2.0;
    let border_high_im = center_im + height_im / 2.0;

    println!("width_re:       {}", width_re);
    println!("height_im:      {}", height_im);
    println!("border_low_re:  {}", border_low_re);
    println!("border_high_re: {}", border_high_re);
    println!("border_low_im:  {}", border_low_im);
    println!("border_high_im: {}", border_high_im);
    println!("(plank):        {}", plank);
    println!("(plank_half):   {}", plank_half);

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width_x {
        numbers_re.push(border_low_re + (plank * x as f64) + plank_half);
    }
    for y in 0..height_y {
        numbers_im.push(border_low_im + (plank * y as f64) + plank_half);
    }

    let area_data = AreaData {
        width_x,
        width_xf64: width_x as f64,
        height_y,
        height_yf64: height_y as f64,
        width_re,
        height_im,
        width_half_x,
        width_half_xf64: width_half_x as f64,
        height_half_y,
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
        data: Mutex::new(area_data),
    }
}

#[cfg(test)]
mod tests {
    use crate::area::init;
    use crate::fractal;

    #[test]
    fn test_init() {
        let conf = fractal::init_trivial_config();
        let area = init(&conf);
        let data = area.data.lock().unwrap();

        assert_eq!(data.border_low_re, -0.5);
        assert_eq!(data.border_high_re, 0.5);
        assert_eq!(data.border_low_im, -0.5);
        assert_eq!(data.border_high_im, 0.5);
    }

    #[test]
    fn test_contains() {
        let conf = fractal::init_trivial_config();
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
    }

    #[test]
    fn test_screen_to_domain_re() {
        let conf = fractal::init_trivial_config();
        let area = init(&conf);

        let res = area.screen_to_domain_re_copy();

        assert_eq!(res.len(), 20);
        assert_eq!(res[0], -0.475);
        assert_eq!(res[19], 0.4750000000000001);
    }

    #[test]
    fn test_screen_to_domain_im() {
        let conf = fractal::init_trivial_config();
        let area = init(&conf);

        let ims = area.screen_to_domain_im_copy();

        assert_eq!(ims.len(), 20);
        assert_eq!(ims[0], -0.475);
        assert_eq!(ims[19], 0.4750000000000001);
    }
}
