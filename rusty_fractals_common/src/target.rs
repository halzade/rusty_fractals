// represents mouse position
// coordinates are calculated from the top left corner
pub struct Target {
    pub width: usize,
    pub height: usize,
    pub scr_x: usize,
    pub scr_y: usize,
}

impl Target {
    pub fn update(&mut self, mouse_position_x: usize, mouse_position_y: usize) {
        let scr_corner_re = mouse_position_x;
        let scr_corner_im = mouse_position_y;
        self.scr_x = (scr_corner_re - (self.width / 2)) as usize;
        self.scr_y = (scr_corner_im - (self.height / 2)) as usize;
    }

    pub fn screen_from_center_x(&self) -> usize {
        self.scr_x
    }

    pub fn screen_from_center_y(&self) -> usize {
        self.scr_y
    }

    pub fn screen_from_corner_x(&self) -> usize {
        // TODO self.scr_corner_re
        0
    }

    pub fn screen_from_corner_y(&self) -> usize {
        // TODO self.scr_corner_im
        0
    }
}

/*
fn init(width_x: usize, height_y: usize) -> Target {
    Target {
        width: width_x,
        height: height_y,
        scr_x: (width_x as f64 / 2.0) as usize,
        scr_y: (height_y as f64 / 2.0) as usize,
    }
}
*/