use crate::constants::ZOOM;
// TODO remove all phantom data?
use std::marker::PhantomData;
use std::sync::Mutex;

/**
 * RxR Area on which the Fractal is calculated
 */
pub struct Area<'lt> {
    pub data: Mutex<AreaData<'lt>>,
}

/**
 * Initial Area configuration
 */
pub struct AreaConfig {
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
}

/**
 * Mutable Area data.
 */
pub struct AreaData<'lt> {
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
    phantom: PhantomData<&'lt bool>,
}

impl<'lt> Area<'_> {
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
     * Check first, if can convert
     * Only then call this method
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

pub fn init<'lt>(config: AreaConfig) -> Area<'lt> {
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

    println!("width_re:       {}", width_re);
    println!("height_im:      {}", height_im);
    println!("border_low_re:  {}", border_low_re);
    println!("border_high_re: {}", border_high_re);
    println!("border_low_im:  {}", border_low_im);
    println!("border_high_im: {}", border_high_im);
    println!("(plank):        {}", plank);

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width_x {
        numbers_re.push(border_low_re + (plank * x as f64));
    }
    for y in 0..height_y {
        numbers_im.push(border_low_im + (plank * y as f64));
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

pub fn init_trivial<'lt>() -> Area<'lt> {
    let area_data = AreaData {
        width_x: 1,
        width_xf64: 1.0,
        height_y: 1,
        height_yf64: 1.0,
        width_re: 1.0,
        height_im: 1.0,
        width_half_x: 0.5 as usize,
        width_half_xf64: 0.5,
        height_half_y: 0.5 as usize,
        height_half_yf64: 0.5,
        numbers_re: Vec::new(),
        numbers_im: Vec::new(),
        center_re: 0.5,
        center_im: 0.5,
        border_low_re: 0.0,
        border_low_im: 0.0,
        border_high_re: 1.0,
        border_high_im: 1.0,
        plank: 0.1,
    };
    Area {
        data: Mutex::new(area_data),
    }
}

#[cfg(test)]
mod tests {
    use crate::area::{init, AreaConfig};

    const VANILLA_AREA_CONFIG: AreaConfig = AreaConfig {
        width_re: 1.0,
        center_re: 0.0,
        center_im: 0.0,
        width_x: 10,
        height_y: 5,
    };

    #[test]
    fn test_init() {
        let area = init(VANILLA_AREA_CONFIG);
        assert_eq!(area.data.lock().unwrap().border_low_re, -0.5);
        assert_eq!(area.data.lock().unwrap().border_high_re, 0.5);
        assert_eq!(area.data.lock().unwrap().border_low_im, -0.25);
        assert_eq!(area.data.lock().unwrap().border_high_im, 0.25);
    }

    #[test]
    fn test_contains() {
        let area = init(VANILLA_AREA_CONFIG);
        assert_eq!(area.contains(0.4, 0.2), true);
        assert_eq!(area.contains(0.4, 0.3), false);
        assert_eq!(area.contains(0.6, 0.2), false);

        assert_eq!(area.contains(-0.4, -0.2), true);
        assert_eq!(area.contains(-0.4, -0.3), false);
        assert_eq!(area.contains(-0.6, -0.2), false);

        assert_eq!(area.contains(-0.4, 0.2), true);
        assert_eq!(area.contains(-0.4, 0.3), false);
        assert_eq!(area.contains(-0.6, 0.2), false);

        assert_eq!(area.contains(0.4, -0.2), true);
        assert_eq!(area.contains(0.4, -0.3), false);
        assert_eq!(area.contains(0.6, -0.2), false);
    }

    #[test]
    fn test_screen_to_domain_re() {
        let area = init(VANILLA_AREA_CONFIG);
        let res = area.screen_to_domain_re_copy();
        assert_eq!(res[0], -0.5);
        assert_eq!(res[5], 0.0);
        assert_eq!(res[9], 0.4);
    }

    #[test]
    fn test_screen_to_domain_im() {
        let area = init(VANILLA_AREA_CONFIG);
        let ims = area.screen_to_domain_im_copy();
        assert_eq!(ims[0], -0.25);
        assert_eq!(ims[1], -0.15);
        assert_eq!(ims[2], -0.04999999999999999);
        assert_eq!(ims[3], 0.050000000000000044);
        assert_eq!(ims[4], 0.15000000000000002);
    }
}
