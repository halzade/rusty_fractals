use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooShort, HibernatedDeepBlack};
use std::sync::{Arc, Mutex};

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


    pub fn set_finished_state(&mut self, state: DomainElementState) {
        self.state = state;
    }

    pub fn past(&mut self) {
        if self.state == FinishedSuccess {
            self.state = FinishedSuccessPast;
        }
    }

    pub fn has_worse_state_then(&self, e: DomainElement) -> bool {
        self.state.cmp(&e.state).is_gt()
    }
}

pub fn init(re: f64, im: f64) -> Arc<Mutex<DomainElement>> {
    Arc::new(Mutex::new(DomainElement {
        origin_re: re,
        origin_im: im,
        state: ActiveNew,
    }))
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
