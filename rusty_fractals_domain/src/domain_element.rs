use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{ActiveNew, Finished, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort, GoodPath, HibernatedDeepBlack};

pub struct DomainElement {
    pub origin_re: f64,
    pub origin_im: f64,
    // Element state is decided by calculation result. Alternatively: If all it's neighbours finished too long, it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    pub state: DomainElementState,
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


    pub fn set_finished_state(&mut self, iterator : u32, path_length : u32, last_quadrance: f64) {
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

    fn past(&mut self) {
        if self.state == FinishedSuccess {
            self.state = FinishedSuccessPast;
        }
    }

    // Returns a negative integer, zero, or a positive integer as this object is less than, equal to, or greater than the specified object
    fn compare_to(&self MaskMandelbrotElement e) {
        if self == e {
            0
        }
        self.state.compareTo(e.state);
    }

    fn has_worse_state_then(&self, MaskMandelbrotElement e) {
        self.compareTo(e) > 0;
    }

    fn good_path(&mut self) {
        self.state = GoodPath;
    }
}

pub fn init(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        state: ActiveNew,
    }
}

pub fn active_new(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        state: ActiveNew
    }
}

pub fn hibernated_deep_black(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        state: HibernatedDeepBlack
    }
}


#[test]
pub fn is_active_new() {}

#[test]
pub fn set_finished_state() {}

