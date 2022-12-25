use crate::pixel_states::MandelbrotPixelState::{ActiveNew, Finished};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum MandelbrotPixelState {
    /**
     * 1.
     * New element just added to Mandelbrot Pixels
     */
    ActiveNew,

    /**
     * 2.
     * Calculation completed
     */
    Finished
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum MaskMandelbrotPixelState {
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
     * New element just added to Mandelbrot Pixels
     * color = {@link MaskMandelbrotMaskColors#ACTIVE_NEW}
     */
    ActiveNew,

    /**
     * 4.
     * Path length was less than ITERATION_MIN.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_TOO_SHORT}
     */
    FinishedTooShort,

    /**
     * 5.
     * Path length reached ITERATION_MAX.
     * It is considered as inside of Mandelbrot set.
     * color = {@link MaskMandelbrotMaskColors#FINISHED_TOO_LONG}
     */
    FinishedTooLong,

    /**
     * 6.
     * Created as already hibernated, and won't be calculated.
     * It didn't have any good data producing NEIGHBOURS {@link #FinishedSuccess} near enough.
     * It had only {@link #FinishedTooLong} NEIGHBOURS.
     * color = {@link MaskMandelbrotMaskColors#HIBERNATED_DEEP_BLACK}
     */
    HibernatedDeepBlack,

    /**
     * 7.
     * Temporarily state, recalculation of divergent PATH in progress
     * color = {@link MaskMandelbrotMaskColors#GOOD_PATH}
     */
    GoodPath
}

#[test]
fn test_mandelbrot_pixel_state() {
    assert_eq!(ActiveNew.cmp(&Finished), -1);
}
