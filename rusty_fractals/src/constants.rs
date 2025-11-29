use image::Rgb;

// How many elements around specific domain element will be investigated for optimization.
// If there is nothing interesting going on around the specific element, the (new) element will be ignored.
pub const NEIGHBOURS: u8 = 3;

// save generated images here
pub const PATH: &str = "/Fractals/";

// to remove noise, ignore pixels with this value or less as 0
pub const COLORING_THRESHOLD: u64 = 3;

// Delete paths shorter than this
// Remembered paths got shorter as some of their elements moved out of displayed Area
pub const MINIMUM_PATH_LENGTH: u64 = 4;

// 4 is quadrance from (0, 0)
// If intermediate calculation result [re,im] spirals beyond this boundary. Calculation stops as divergent.
pub const CALCULATION_BOUNDARY: u64 = 4;

// 0.98 is a good choice for 25fps and moderate speed
pub const ZOOM: f64 = 0.98;

// take result data snapshot for comparison at well colored frame
pub const TAKE_MEASURES_AT_FRAME: u64 = 20;

// phoenix fractal constants
pub const PHOENIX_INIT_C: f64 = 0.35;
pub const PHOENIX_INIT_P: f64 = -0.25;
pub const PHOENIX_INITIALIZER: f64 = 1.0;

// color for last path display
pub const GRAY: Rgb<u8> = Rgb([254, 254, 254]);
