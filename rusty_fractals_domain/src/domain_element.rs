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

    pub fn is_finished_too_short(&self) -> bool {
        self.state == FinishedTooShort
    }

    pub fn is_hibernated(&self) -> bool {
        self.state == FinishedTooShort || self.state == HibernatedDeepBlack
    }

    pub fn is_finished_success_any(&self) -> bool {
        self.state == FinishedSuccessPast || self.state == FinishedSuccess
    }

    pub fn is_finished_success_past(&self) -> bool {
        self.state == FinishedSuccessPast
    }


    pub fn set_finished_state(&mut self, iterator: u32) {
        if iterator == ITERATION_MAX {
            self.state = FinishedTooLong;
            Stats.newElementsTooLong += 1;
            return;
        }
        if pathLength < ITERATION_min {
            self.state = FinishedTooShort;
            Stats.newElementsTooShort += 1;
            return;
        }
        self.state = FinishedSuccess;
        Stats.newElementsLong += 1;
    }

    pub fn past(&mut self) {
        if self.state == FinishedSuccess {
            self.state = FinishedSuccessPast;
        }
    }

    // Returns a negative integer, zero, or a positive integer as this object is less than, equal to, or greater than the specified object
    pub fn compare_to(&self, e: DomainElement) {
        if self == e {
            0
        }
        self.state.compareTo(e.state);
    }

    pub fn has_worse_state_then(&self, e: DomainElement) {
        self.compareTo(e) > 0;
    }

    pub fn good_path(&mut self) {
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
        state: ActiveNew,
    }
}

pub fn hibernated_deep_black(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        state: HibernatedDeepBlack,
    }
}


#[test]
pub fn is_active_new() {}

#[test]
pub fn set_finished_state() {}

