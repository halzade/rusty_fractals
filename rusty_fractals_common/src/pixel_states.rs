use image::Rgb;
use crate::pixel_states::DomainElementState::{ActiveNew, FinishedSuccessPast, FinishedTooShort, HibernatedDeepBlack};

pub const ACTIVE_NEW: Rgb<u8> = Rgb([40, 180, 150]);
pub const FINISHED_TOO_LONG: Rgb<u8> = Rgb([0, 0, 0]);
pub const HIBERNATED_DEEP_BLACK: Rgb<u8> = Rgb([90, 90, 90]);
pub const FINISHED_TOO_SHORT: Rgb<u8> = Rgb([220, 220, 240]);
pub const FINISHED_SUCCESS: Rgb<u8> = Rgb([255, 0, 0]);
pub const FINISHED_SUCCESS_PAST: Rgb<u8> = Rgb([130, 100, 130]);
pub const FINISHED: Rgb<u8> = Rgb([130, 130, 100]);


#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum DomainElementState {
    /**
     * 1.
     * Calculation path Finished with success in previous calculation iteration (zoom).
     * This is updated state from previous state {@link #FinishedSuccess}.
     * If there was a conflict when moving pixels to new location after zoomIn(), use this state.
     * There won't be any difference in result data, only in mandelbrot pixel state and colour.
     */
    FinishedSuccessPast,

    /**
     * 2.
     * Path length more than ITERATION_MIN, this element produced good data.
     */
    FinishedSuccess,

    /**
     * 3.
     * Calculation completed
     */
    Finished,

    /**
     * 4.
     * New element just added to Mandelbrot Pixels
     */
    ActiveNew,

    /**
     * 5.
     * Path length was less than ITERATION_MIN.
     */
    FinishedTooShort,

    /**
     * 6.
     * Path length reached ITERATION_MAX.
     * It is considered as inside of Mandelbrot set.
     */
    FinishedTooLong,

    /**
     * 7.
     * Created as already hibernated, and won't be calculated.
     * It didn't have any good data producing NEIGHBOURS {@link #FinishedSuccess} near enough.
     * It had only {@link #FinishedTooLong} NEIGHBOURS.
     */
    HibernatedDeepBlack,
}

pub fn is_active_new(state: DomainElementState) -> bool {
    state == ActiveNew
}

pub fn is_finished_success_past(state: DomainElementState) -> bool {
    state == FinishedSuccessPast
}

pub fn is_hibernated(state: DomainElementState) -> bool {
    state == FinishedTooShort || state == HibernatedDeepBlack
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::Less;
    use crate::pixel_states::DomainElementState::{Finished, FinishedSuccessPast};

    #[test]
    fn test_pixel_state() {
        assert_eq!(FinishedSuccessPast.cmp(&Finished), Less);
    }
}
