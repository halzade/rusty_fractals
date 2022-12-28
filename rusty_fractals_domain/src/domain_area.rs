pub struct DomainArea {
    pub width_re: f64,
    pub height_im: f64,
    pub width_x: u32,
    pub height_y: u32,
    pub numbers_re: Vec<f64>,
    pub numbers_im: Vec<f64>,
    border_low_re: f64,
    border_low_im: f64,
    border_high_re: f64,
    border_high_im: f64,
    plank: f64,
}

impl DomainArea {
    fn contains(&self, re: f64, im: f64) -> bool {
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

    fn point_to_pixel(&self, m: rusty_fractals_core::mem::Mem, re: f64, im: f64) {
        m.good = true;
        m.px = Math.round((self.width_x * (re - self.centerRe) / self.width_re) + resolutionHalfRe);
        if m.px >= self.width_x || m.px < 0 {
            m.good = false;
            return;
        }
        m.py = Math.round(((RESOLUTION_HEIGHT * (im - this.centerIm)) / this.sizeIm) + resolutionHalfIm);
        if m.py >= RESOLUTION_HEIGHT || m.py < 0 {
            m.good = false;
        }
    }

    fn zoom_in(&mut self) {
        self.width_re = self.width_re * ZOOM;
        self.height_im = self.height_im * ZOOM;
        self.plank = self.width_re / RESOLUTION_WIDTH;
        initiate();
    }

    fn move_to_coordinates(&self) {
        self.centerRe = screenToDomainCreateRe(Target.getScreenFromCornerX());
        self.centerIm = screenToDomainCreateIm(Target.getScreenFromCornerY());
        log.debug("Move to: " + self.centerRe + "," + self.centerIm);
    }

    /**
     * move to zoom target
     */
    fn move_to_initial_coordinates(&self, init_target_re: f64, init_target_im: f64) {
        self.centerRe = init_target_re;
        self.centerIm = init_target_im;
    }
}

pub fn init(width_re: f64, center_re: f64, center_im: f64, width_x: u32, height_y: u32) -> DomainArea {
    let plank = width_re / width_x as f64;
    let height_im = width_re * (width_x as f64 / height_y as f64);
    let border_low_re = center_re - (width_re / 2.0);
    let border_high_re = center_re + (width_re / 2.0);
    let border_low_im = center_im - (height_im / 2.0);
    let border_high_im = center_im + (height_im / 2.0);

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

    DomainArea {
        width_re,
        height_im,
        width_x,
        height_y,
        numbers_re,
        numbers_im,
        border_low_re,
        border_low_im,
        border_high_re,
        border_high_im,
        plank,
    }
}

#[test]
fn test_init() {
    let area = init(1.0, 0.5, 0.5, 10, 10);
    assert_eq!(area.border_low_re, -0.5);
    assert_eq!(area.border_high_re, 0.5);
    assert_eq!(area.border_low_im, -0.5);
    assert_eq!(area.border_high_im, 0.5);
}

#[test]
fn test_contains() {
    let area = init(1.0, 0.5, 0.5, 10, 10);
    let y = area.contains(0.4, 0.4);
    let n = area.contains(0.4, 1.5);
    assert_eq!(y, true);
    assert_eq!(n, false);
}

#[test]
fn test_screen_to_domain_re() {
    let area = init(1.0, 0.5, 0.5, 10, 10);
    let r = area.screen_to_domain_re(500);
    assert_eq!(r, 0.125);
}

#[test]
fn test_screen_to_domain_im() {
    let area = init(1.0, 0.5, 0.5, 10, 10);
    let i = area.screen_to_domain_im(20);
    assert_eq!(i, -0.475);
}
