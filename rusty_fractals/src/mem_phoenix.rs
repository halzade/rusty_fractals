use crate::constants::PHOENIX_INITIALIZER;
use crate::fractal::MemType;
use crate::mem::Mem;

/**
 * Memory object for Phoenix fractal
 */
pub struct MemPhoenix {
    pub m: Mem,
    // Values of previous calculation results
    pub prev_prev_re: f64,
    pub prev_prev_im: f64,
    pub prev_re: f64,
    pub prev_im: f64,
}

impl MemType<MemPhoenix> for MemPhoenix {
    fn new(re: f64, im: f64) -> MemPhoenix {
        MemPhoenix {
            m: Mem { re, im },
            prev_prev_re: PHOENIX_INITIALIZER,
            prev_prev_im: PHOENIX_INITIALIZER,
            prev_re: PHOENIX_INITIALIZER,
            prev_im: PHOENIX_INITIALIZER,
        }
    }

    fn quad(&self) -> f64 {
        self.m.quad()
    }

    fn re(&self) -> f64 {
        self.re()
    }

    fn im(&self) -> f64 {
        self.im()
    }
}

impl MemPhoenix {
    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }

    pub fn re(&self) -> f64 {
        self.m.re
    }

    pub fn im(&self) -> f64 {
        self.m.im
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
