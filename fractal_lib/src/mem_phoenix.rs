use crate::mem::Mem;

/**
 * Memory object for Phoenix fractal
 */
pub struct MemPhoenix {
    pub m: Mem,
    // Values of previous calculation results
    prev_prev_re: f64,
    prev_prev_im: f64,
    prev_re: f64,
    prev_im: f64,
}

impl MemPhoenix {
    pub fn re(&self) -> f64 {
        self.m.re
    }

    pub fn im(&self) -> f64 {
        self.m.im
    }

    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }
}
