use crate::fractal;
use crate::pixel_states::MandelbrotPixelState;
use MandelbrotPixelState::{ActiveNew, Finished};

pub struct MandelbrotElement {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: i32,
    pub quad: f64,
    pub state: MandelbrotPixelState,
}

impl MandelbrotElement {
    pub fn is_active_new(&self) -> bool {
        self.state == ActiveNew
    }

    pub fn set_finished_state(&mut self, iterator: i32, q: f64) {
        self.state = Finished;
        self.quad = q;
        if iterator < 1 {
            self.value = 1;
            return;
        }
        if iterator == fractal::ITERATION_MAX {
            self.value = 0;
            return;
        }
        self.value = iterator;
    }

    pub fn set_average_with(&mut self, e: MandelbrotElement) {
        self.value = (((self.value + e.value) as f64) / 2.0) as i32;
    }
}

pub fn init(re: f64, im: f64) -> MandelbrotElement {
    MandelbrotElement {
        origin_re: re,
        origin_im: im,
        value: 0,
        quad: 0.0,
        state: ActiveNew,
    }
}

#[test]
pub fn is_active_new() {}

#[test]
pub fn set_finished_state() {}

#[test]
fn test_set_average_with() {
    let mut me = MandelbrotElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 10,
        quad: 0.0,
        state: ActiveNew,
    };
    let other = MandelbrotElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 3,
        quad: 0.0,
        state: ActiveNew,
    };

    me.set_average_with(other);
    assert_eq!(me.value, 55);
}
