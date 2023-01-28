use std::cmp::Ordering::Less;
use image::Rgb;
use crate::pixel_states::DomainElementState::{ActiveNew, Finished, FinishedSuccessPast, FinishedTooShort, HibernatedDeepBlack};

pub const ACTIVE_NEW: Rgb<u8> = Rgb([40, 180, 150]);
pub const FINISHED_TOO_LONG: Rgb<u8> = Rgb([0, 0, 0]);
pub const HIBERNATED_DEEP_BLACK: Rgb<u8> = Rgb([90, 90, 90]);
pub const FINISHED_TOO_SHORT: Rgb<u8> = Rgb([220, 220, 240]);
pub const FINISHED_SUCCESS: Rgb<u8> = Rgb([255, 0, 0]);
pub const FINISHED_SUCCESS_PAST: Rgb<u8> = Rgb([130, 100, 130]);
pub const FINISHED: Rgb<u8> = Rgb([130, 130, 100]);
pub const GOOD_PATH: Rgb<u8> = Rgb([200, 108, 10]);


#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum DomainElementState {
    /**
     * 1.
     * Calculation PATH Finished with success in previous calculation iteration (zoom).
     * This is updated state from previous state {@link #FinishedSuccess}.
     * If there was a conflict when moving pixels to new location after zoomIn(), use this state.
     * There won't be any difference in Finebrot data, only in mandelbrot pixel state and color.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_SUCCESS_PAST}
     */
    FinishedSuccessPast,

    /**
     * 2.
     * Path length more than ITERATION_MIN, this element produced good data.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_SUCCESS}
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
     * color = {@link MaskMandelbrotMaskColors#ACTIVE_NEW}
     */
    ActiveNew,

    /**
     * 5.
     * Path length was less than ITERATION_MIN.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_TOO_SHORT}
     */
    FinishedTooShort,

    /**
     * 6.
     * Path length reached ITERATION_MAX.
     * It is considered as inside of Mandelbrot set.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_TOO_LONG}
     */
    FinishedTooLong,

    /**
     * 7.
     * Created as already hibernated, and won't be calculated.
     * It didn't have any good data producing NEIGHBOURS {@link #FinishedSuccess} near enough.
     * It had only {@link #FinishedTooLong} NEIGHBOURS.
     * color = {@link MaskMandelbrotMaskColors#HIBERNATED_DEEP_BLACK}
     */
    HibernatedDeepBlack,

    /**
     * 8.
     * Temporarily state, recalculation of divergent PATH in progress
     * color = {@link MaskMandelbrotMaskColors#GOOD_PATH}
     */
    GoodPath,
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

#[test]
fn test_pixel_state() {
    assert_eq!(ActiveNew.cmp(&Finished), Less);
}
