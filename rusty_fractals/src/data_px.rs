use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{
    ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    HibernatedDeepBlack,
};
use image::Rgb;

#[derive(Clone, Copy)]
pub struct DataPx {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: u32,
    /* Element state is decided by calculation result.
     * Alternatively: If all it's neighbours finished too long, it is going to be 
     * created as HibernatedBlack and its origin won't seed any calculation path.
     */
    pub state: DomainElementState,
    pub quad: f64,
    pub color: Option<Rgb<u8>>,
}

impl DataPx {
    pub fn is_active_new(&self) -> bool {
        self.state == ActiveNew
    }

    pub fn is_finished_too_short(&self) -> bool {
        self.state == FinishedTooShort
    }

    pub fn is_finished_too_long(&self) -> bool {
        self.state == FinishedTooLong
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

    pub fn has_worse_state_then(&self, other: &DataPx) -> bool {
        self.state.cmp(&other.state).is_gt()
    }

    pub fn set_finished_state(&mut self, state: DomainElementState) {
        self.state = state;
    }

    pub fn set_average_with(&mut self, other: DataPx) {
        self.value = (((self.value + other.value) as f64) / 2.0) as u32;
    }
}

// #[derive(Clone, Copy)]
// pub struct DataPx3 {
//     pub origin_re: f64,
//     pub origin_im: f64,
//     pub value_r: u32,
//     pub value_g: u32,
//     pub value_b: u32,
//     // Element state is decided by calculation result. Alternatively: If all it's neighbours finished too long, it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
//     pub state: DomainElementState,
//     pub color_r: Option<Rgb<u8>>,
//     pub color_g: Option<Rgb<u8>>,
//     pub color_b: Option<Rgb<u8>>,
// }

pub fn init(origin_re: f64, origin_im: f64) -> DataPx {
    DataPx {
        origin_re,
        origin_im,
        value: 0,
        state: ActiveNew,
        quad: 0.0,
        color: None,
    }
}

pub fn hibernated_deep_black(re: f64, im: f64) -> DataPx {
    // TODO copy quad
    DataPx {
        origin_re: re,
        origin_im: im,
        value: 0,
        state: HibernatedDeepBlack,
        quad: 1.0,
        color: None,
    }
}

pub fn active_new(re: f64, im: f64) -> DataPx {
    // todo copy quad
    DataPx {
        origin_re: re,
        origin_im: im,
        value: 0,
        state: ActiveNew,
        quad: 1.0,
        color: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::data_px::{active_new, DataPx};
    use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccessPast};

    #[test]
    fn test_set_average_with() {
        let mut dp = DataPx {
            origin_re: 0.0,
            origin_im: 0.0,
            value: 10,
            state: FinishedSuccessPast,
            quad: 0.0,
            color: None,
        };
        let other = DataPx {
            origin_re: 0.0,
            origin_im: 0.0,
            value: 3,
            state: FinishedSuccessPast,
            quad: 0.0,
            color: None,
        };

        dp.set_average_with(other);
        assert_eq!(dp.value, 6);
    }

    #[test]
    fn test_active_new() {
        let dpx = active_new(0.0, 0.0);
        // implements PartialEq
        assert_eq!(dpx.state == ActiveNew, true);
    }
}
