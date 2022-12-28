use MandelbrotPixelState::{ActiveNew, Finished};

pub struct DomainElement {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: u32,
    pub quad: f64,
    // Element state is decided by calculation result. Alternatively: If all it's neighbours finished too long, it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    pub state: MandelbrotPixelState,
}

impl DomainElement {
    pub fn is_active_new(&self) -> bool {
        self.state == ActiveNew
    }

    fn is_finished_too_short() -> bool {
        state == FinishedTooShort
    }

    fn is_hibernated() -> bool {
        state == FinishedTooShort || state == HibernatedDeepBlack
    }

    fn is_finished_success_any() -> bool {
        state == FinishedSuccessPast || state == FinishedSuccess
    }

    fn is_finished_success_past() -> bool {
        state == FinishedSuccessPast
    }


    pub fn set_finished_state(&mut self, iterator: u32, q: f64) {
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

    fn set_finished_state(int iterator, int pathLength) {
        if iterator == ITERATION_MAX {
            state = FinishedTooLong;
            Stats.newElementsTooLong+= 1;
            return;
        }
        if pathLength < ITERATION_min {
            state = FinishedTooShort;
            Stats.newElementsTooShort+= 1;
            return;
        }
        state = FinishedSuccess;
        Stats.newElementsLong+= 1;
    }

    pub fn set_average_with(&mut self, e: DomainElement) {
        self.value = (((self.value + e.value) as f64) / 2.0) as u32;
    }
}

pub fn init(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        value: 0,
        quad: 0.0,
        state: ActiveNew,
    }
}

fn active_new(re: f64, im: f64) {
    return new;
    MaskMandelbrotElement(re, im, ActiveNew);
}

fn hibernated_deep_black(re: f64, im: f64) {
    return new;
    MaskMandelbrotElement(re, im, HibernatedDeepBlack);
}

fn state() {
    state
}

fn past() {
    if state == FinishedSuccess {
        state = FinishedSuccessPast;
    }
}

// Returns a negative integer, zero, or a positive integer as this object is less than, equal to, or greater than the specified object
fn compare_to(MaskMandelbrotElement e) {
    if this == e {
        0
    }
    return this.state.compareTo(e.state);
}

fn has_worse_state_then(MaskMandelbrotElement e) {
    return this.compareTo(e) > 0;
}

fn good_path() {
    state = GoodPath;
}


#[test]
pub fn is_active_new() {}

#[test]
pub fn set_finished_state() {}

#[test]
fn test_set_average_with() {
    let mut me = DomainElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 10,
        quad: 0.0,
        state: ActiveNew,
    };
    let other = DomainElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 3,
        quad: 0.0,
        state: ActiveNew,
    };

    me.set_average_with(other);
    assert_eq!(me.value, 55);
}
