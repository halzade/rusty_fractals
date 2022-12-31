use rgb::RGB;

const ACTIVE_NEW: RGB<u8> = RGB::new(40, 180, 150);
const FINISHED_TOO_LONG: RGB<u8> = RGB::new(0, 0, 0);
const HIBERNATED_DEEP_BLACK: RGB<u8> = RGB::new(90, 90, 90);
const FINISHED_TOO_SHORT: RGB<u8> = RGB::new(220, 220, 240);
const FINISHED_SUCCESS: RGB<u8> = RGB::new(255, 0, 0);
const FINISHED_SUCCESS_PAST: RGB<u8> = RGB::new(130, 100, 130);
const GOOD_PATH: RGB<u8> = RGB::new(200, 108, 10);


#[derive(PartialOrd, Ord, PartialEq, Eq)]
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
    GoodPath
}

#[test]
fn test_mandelbrot_pixel_state() {
    assert_eq!(ActiveNew.cmp(&Finished), -1);
}
