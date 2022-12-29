
// How many elements around specific domain element will be investigated for optimization.
// If there is nothing interesting going on around the specific element, the (new) element will be ignored.
pub const NEIGHBOURS: u8 = 3;

pub const PATH: &str = "/Fractals/";

pub const COLORING_THRESHOLD: i32 = 3;

// Delete shorter paths then this
pub const MINIMUM_PATH_LENGTH: i32 = 4;

// 4 is quadrance from (0, 0)
// If intermediate calculation result [re,im] spirals beyond this boundary. Calculation stops as divergent.
pub const CALCULATION_BOUNDARY: i32 = 4;

// 0.98 is a good choice for 25fps and moderate speed
pub const ZOOM: f64 = 0.98;
