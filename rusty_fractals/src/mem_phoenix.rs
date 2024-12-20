use crate::constants::PHOENIX_INITIALIZER;
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

impl MemPhoenix {
    pub fn new(re: f64, im: f64) -> MemPhoenix {
        MemPhoenix {
            m: Mem { re, im },
            prev_prev_re: PHOENIX_INITIALIZER,
            prev_prev_im: PHOENIX_INITIALIZER,
            prev_re: PHOENIX_INITIALIZER,
            prev_im: PHOENIX_INITIALIZER,
        }
    }

    pub fn re(&self) -> f64 {
        self.re()
    }

    pub fn im(&self) -> f64 {
        self.im()
    }

    pub fn quad(&self) -> f64 {
        self.m.quad()
    }

    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
