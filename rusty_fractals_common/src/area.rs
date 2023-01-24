use crate::constants::ZOOM;

pub struct AreaConfig {
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    pub width_x: usize,
    pub height_y: usize,
}

const VANILLA_AREA_CONFIG: AreaConfig = AreaConfig { width_re: 1.0, center_re: 0.0, center_im: 0.0, width_x: 10, height_y: 10 };

pub struct Area {
    pub width_re: f64,
    pub height_im: f64,
    pub width_half_re: f64,
    pub height_half_im: f64,
    pub width_x: usize,
    pub height_y: usize,
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


impl Area {
    pub fn contains(&self, re: f64, im: f64) -> bool {
        re > self.border_low_re
            && re < self.border_high_re
            && im > self.border_low_im
            && im < self.border_high_im
    }

    pub fn screen_to_domain_re(&self, x: usize) -> f64 {
        self.numbers_re[x]
    }

    pub fn screen_to_domain_im(&self, y: usize) -> f64 {
        self.numbers_im[y]
    }

    // check first, if can convert
    pub fn domain_point_to_result_pixel(&self, re: f64, im: f64) -> (usize, usize) {
        let px = ((self.width_x as f64 * (re - self.center_re) / self.width_re) + self.width_half_re).round() as usize;
        let py = ((self.height_y as f64 * (im - self.center_im) / self.height_im) + self.height_half_im).round() as usize;
        (px, py)
    }

    pub fn zoom_in(&mut self) {
        println!("zoom_in()");
        self.width_re = self.width_re * ZOOM;
        self.height_im = self.height_im * ZOOM;
        self.plank = self.width_re / self.width_x as f64;
        // TODO initiate();
    }

    // fn move_to_coordinates(&mut self) {
    //     self.center_re = screenToDomainCreateRe(Target.getScreenFromCornerX());
    //     self.center_im = screenToDomainCreateIm(Target.getScreenFromCornerY());
    //     println!("Move to: {}, {}", self.center_re, self.center_im);
    // }

    /**
     * move to zoom target
     */
    pub fn move_to_initial_coordinates(&mut self, init_target_re: f64, init_target_im: f64) {
        println!("move_to_initial_coordinates()");
        self.center_re = init_target_re;
        self.center_im = init_target_im;
    }
}

pub fn init(config: AreaConfig) -> Area {
    println!("init()");
    let width_re = config.width_re;
    let center_re = config.center_re;
    let center_im = config.center_im;
    let width_x = config.width_x;
    let height_y = config.height_y;

    let plank = width_re / width_x as f64;
    let height_im = width_re * (width_x as f64 / height_y as f64);
    let width_half_re = width_re / 2.0;
    let height_half_im = height_im / 2.0;
    let border_low_re = center_re - (width_half_re);
    let border_high_re = center_re + (width_half_re);
    let border_low_im = center_im - (height_half_im);
    let border_high_im = center_im + (height_half_im);

    println!("border_low_re  {}", border_low_re);
    println!("border_high_re {}", border_high_re);
    println!("border_low_im  {}", border_low_im);
    println!("border_high_im {}", border_high_im);

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width_x {
        numbers_re.push(border_low_re + (plank * x as f64));
    }
    for y in 0..height_y {
        numbers_im.push(border_low_im + (plank * y as f64));
    }

    Area {
        width_re,
        height_im,
        width_half_re,
        height_half_im,
        width_x,
        height_y,
        numbers_re,
        numbers_im,
        center_re,
        center_im,
        border_low_re,
        border_low_im,
        border_high_re,
        border_high_im,
        plank,
    }
}

#[test]
fn test_init() {
    let area = init(VANILLA_AREA_CONFIG);
    assert_eq!(area.border_low_re, -0.5);
    assert_eq!(area.border_high_re, 0.5);
    assert_eq!(area.border_low_im, -0.5);
    assert_eq!(area.border_high_im, 0.5);
}

#[test]
fn test_contains() {
    let area = init(VANILLA_AREA_CONFIG);
    let y = area.contains(0.4, 0.4);
    let n = area.contains(0.4, 1.5);
    assert_eq!(y, true);
    assert_eq!(n, false);
}

#[test]
fn test_screen_to_domain_re() {
    let area = init(VANILLA_AREA_CONFIG);
    let r = area.screen_to_domain_re(5);
    assert_eq!(r, 0.0);
}

#[test]
fn test_screen_to_domain_im() {
    let area = init(VANILLA_AREA_CONFIG);
    let i = area.screen_to_domain_im(2);
    assert_eq!(i, -0.3);
}
