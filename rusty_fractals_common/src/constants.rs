
// How many elements around specific domain element will be investigated for optimization.
// If there is nothing interesting going on around the specific element, the (new) element will be ignored.
pub const NEIGHBOURS: u8 = 3;

// save generated images here
pub const PATH: &str = "/Fractals/";

// to remove noise, tread pixels with this value or less as 0
pub const COLORING_THRESHOLD: u32 = 3;

// Delete shorter paths then this
pub const MINIMUM_PATH_LENGTH: u32 = 4;

// 4 is quadrance from (0, 0)
// If intermediate calculation result [re,im] spirals beyond this boundary. Calculation stops as divergent.
pub const CALCULATION_BOUNDARY: u32 = 4;

// 0.98 is a good choice for 25fps and moderate speed
pub const ZOOM: f64 = 0.98;
