use image::Rgb;
use crate::pixel_states::{ACTIVE_NEW, DomainElementState, FINISHED, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, HIBERNATED_DEEP_BLACK};
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooShort, HibernatedDeepBlack};

pub struct DataPx {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: u32,
    // Element state is decided by calculation result. Alternatively: If all it's neighbours finished too long, it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    pub state: DomainElementState,
    pub quad: f64,
    // inverted quadrance
    pub quid: f64,
    pub color_value: u32,
}

impl DataPx {
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

    pub fn past(&mut self) {
        if self.state == FinishedSuccess {
            self.state = FinishedSuccessPast;
        }
    }

    pub fn has_worse_state_then(&self, other: DataPx) -> bool {
        self.state.cmp(&other.state).is_gt()
    }

    pub fn set_finished_state(&mut self, state: DomainElementState) {
        self.state = state;
    }

    fn color_value(&self) -> u32 {
        self.color_value
    }

    fn set_color_value(&mut self, palette_colour_index: u32) {
        self.color_value = palette_colour_index;
    }

    pub fn set_average_with(&mut self, other: DataPx) {
        self.value = (((self.value + other.value) as f64) / 2.0) as u32;
    }
}

pub fn init(origin_re: f64, origin_im: f64) -> DataPx {
    DataPx {
        origin_re,
        origin_im,
        value: 0,
        state: ActiveNew,
        quad: 1.0,
        quid: 1.0,
        color_value: 0,
    }
}

pub fn color_for_state(state: DomainElementState) -> Rgb<u8> {
    match state {
        // most of the elements are going to be FinishedSuccessPast
        FinishedSuccessPast => FINISHED_SUCCESS_PAST,
        HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
        ActiveNew => ACTIVE_NEW,
        FinishedSuccess => FINISHED_SUCCESS,
        FinishedTooShort => FINISHED_TOO_SHORT,
        FinishedTooLong => FINISHED_TOO_LONG,
        Finished => FINISHED
    }
}


#[test]
fn test_set_average_with() {
    let mut dp = DataPx { origin_re: 0.0, origin_im: 0.0, value: 10, state: FinishedSuccessPast, quad: 0.0, quid: 0.0, color_value: 0 };
    let other = DataPx { origin_re: 0.0, origin_im: 0.0, value: 3, state: FinishedSuccessPast, quad: 0.0, quid: 0.0, color_value: 0 };

    dp.set_average_with(other);
    assert_eq!(dp.value, 55);
}

