use crate::infra::constants::PHOENIX_INITIALIZER;
use crate::rusty::fractal::MemType;
use crate::calc::mem::Mem;

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
    pub fn plus(&mut self, r: f64, i: f64) {
        self.m.plus(r, i);
    }

    pub fn square(&mut self) {
        self.m.square();
    }

    pub const fn new(re: f64, im: f64) -> Self {
        Self {
            m: Mem { re, im, it: 0 },
            prev_prev_re: PHOENIX_INITIALIZER,
            prev_prev_im: PHOENIX_INITIALIZER,
            prev_re: PHOENIX_INITIALIZER,
            prev_im: PHOENIX_INITIALIZER,
        }
    }
}

impl MemType<Self> for MemPhoenix {
    fn new(re: f64, im: f64) -> Self {
        Self::new(re, im)
    }

    fn quad(&self) -> f64 {
        self.m.quad()
    }

    fn re(&self) -> f64 {
        self.m.re
    }

    fn im(&self) -> f64 {
        self.m.im
    }
}

#[cfg(test)]
mod tests {
    use crate::calc::mem_phoenix::MemPhoenix;

    #[tokio::test]
    async fn test_plus() {
        let mut mp = MemPhoenix::new(0.02, 0.1);
        mp.plus(1.1, 1.2);
        assert_eq!(mp.m.re, 1.12);
        assert_eq!(mp.m.im, 1.3);
    }
}
