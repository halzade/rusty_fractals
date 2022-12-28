pub struct DomainArea {
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

    fn is_outside(&self, re: f64, im: f64) {
        re < self.border_low_re
            || re > self.border_high_re
            || im < self.border_low_im
            || im > self.border_high_im;
    }

    pub fn screen_to_domain_re(&self, x: usize) -> f64 {
        self.numbers_re[x]
    }

    pub fn screen_to_domain_im(&self, y: usize) -> f64 {
        self.numbers_im[y]
    }

    fn point_to_pixel(m: mem::Mem, re: f64, im: f64) {
        m.good = true;
        m.px = (int)
        Math.round((RESOLUTION_WIDTH * (re - this.centerRe) / this.sizeRe) + resolutionHalfRe);
        if (m.px >= RESOLUTION_WIDTH || m.px < 0) {
            m.good = false;
            return;
        }
        m.py = (int)
        Math.round(((RESOLUTION_HEIGHT * (im - this.centerIm)) / this.sizeIm) + resolutionHalfIm);
        if (m.py >= RESOLUTION_HEIGHT || m.py < 0) {
            m.good = false;
        }
    }
}

pub fn init(width_re: f64, center_re: f64, center_im: f64, width: u32, height: u32) -> DomainArea {
    let scr_ratio_x = width as f64 / height as f64;
    let width_im = width_re * scr_ratio_x;
    let plank = width_re / width as f64;

    let border_low_re = center_re - (width_re / 2.0);
    let border_high_re = center_re + (width_re / 2.0);
    let border_low_im = center_im - (width_im / 2.0);
    let border_high_im = center_im + (width_im / 2.0);

    println!("border_low_re  {}", border_low_re);
    println!("border_high_re {}", border_high_re);
    println!("border_low_im  {}", border_low_im);
    println!("border_high_im {}", border_high_im);

    /* Generate domain elements */
    let mut numbers_re: Vec<f64> = Vec::new();
    let mut numbers_im: Vec<f64> = Vec::new();
    for x in 0..width {
        numbers_re.push(border_low_re + (plank * x as f64));
    }
    for y in 0..height {
        numbers_im.push(border_low_im + (plank * y as f64));
    }

    DomainArea {
        numbers_re,
        numbers_im,
        border_low_re,
        border_low_im,
        border_high_re,
        border_high_im,
        plank,
    }

    public
    void
    zoomIn()
    {
        sizeRe = sizeRe * ZOOM;
        sizeIm = sizeIm * ZOOM;
        this.plank = sizeRe / RESOLUTION_WIDTH;
        initiate();
    }

    public
    void
    moveToCoordinates()
    {
        this.centerRe = screenToDomainCreateRe(Target.getScreenFromCornerX());
        this.centerIm = screenToDomainCreateIm(Target.getScreenFromCornerY());
        log.debug("Move to: " + this.centerRe + "," + this.centerIm);
    }

    /**
     * move to zoom target
     */
    public
    void
    moveToInitialCoordinates()
    {
        this.centerRe = INIT_FINEBROT_TARGET_re;
        this.centerIm = INIT_FINEBROT_TARGET_im;
    }
}

#[test]
fn test_init() {
    let area = init(1.0, 0.0, 0.0, 10, 10);
    assert_eq!(area.border_low_re, -0.5);
    assert_eq!(area.border_high_re, 0.5);
    assert_eq!(area.border_low_im, -0.5);
    assert_eq!(area.border_high_im, 0.5);
}

#[test]
fn test_contains() {
    let area = init(1.0, 0.0, 0.0, 10, 10);
    let y = area.contains(0.4, 0.4);
    let n = area.contains(0.4, 1.5);
    assert_eq!(y, true);
    assert_eq!(n, false);
}

#[test]
fn test_screen_to_domain_re() {
    let area = init(1.0, 0.0, 0.0, 10, 10);
    let r = area.screen_to_domain_re(500);
    assert_eq!(r, 0.125);
}

#[test]
fn test_screen_to_domain_im() {
    let area = init(1.0, 0.0, 0.0, 10, 10);
    let i = area.screen_to_domain_im(20);
    assert_eq!(i, -0.475);
}
