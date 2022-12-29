add use crate::fractal::{HEIGHT_Y, WIDTH_X};

// represents mouse position
// coordinates are calculated from the top left corner
struct Target {
    pub scr_re: u32,
    pub scr_im: u32,
}

impl Target {
    pub fn update(&mut self, mouse_position_x: u32, mouse_position_y: u32) {
        let scr_corner_re = mouse_position_x;
        let scr_corner_im = mouse_position_y;
        self.scr_re = scr_corner_re - (WIDTH_X as f64 / 2.0) as u32;
        self.scr_im = scr_corner_im - (HEIGHT_Y as f64 / 2.0) as u32;
    }

    pub fn screen_from_center_x(&self) -> u32 {
        self.scr_re
    }

    pub fn screen_from_center_y(&self) -> u32 {
        self.scr_im
    }

    pub fn screen_from_corner_x(&self) -> u32 {
        self.scr_corner_re
    }

    pub fn screen_from_corner_y(&self) -> u32 {
        self.scr_corner_im
    }
}

fn init(width_x: u32, height_y: u32) -> Target {
    Target {
        scr_re: width_x as f64 / 2.0 as u32,
        scr_im: height_y as f64 / 2.0 as u32,
    }
}