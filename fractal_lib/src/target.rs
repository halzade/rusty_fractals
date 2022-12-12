// coordinates are calculated from the top left corner

use crate::fractal::{HEIGHT_Y, WIDTH_X};

struct Target {
    scr_re: u32,
    scr_im: u32,
    scr_corner_re: u32,
    scr_corner_im: u32,
}

impl Target {
    pub fn update(&mut self, mouse_position_x: u32, mouse_position_y: u32) {
        self.scr_corner_re = mouse_position_x;
        self.scr_corner_im = mouse_position_y;
        self.scr_re = self.scr_corner_re - (WIDTH_X as f64 / 2.0) as u32;
        self.scr_im = self.scr_corner_im - (HEIGHT_Y as f64 / 2.0) as u32;
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