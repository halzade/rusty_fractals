mod domain_pixel;
pub mod domain;
pub mod domain_area;
pub mod domain_element;
pub mod resolution_multiplier;
mod domain_pixel_colors;
mod pixel_states;

// How many elements around specific domain element will be investigated for optimization.
// If there is nothing interesting going on around the specific element, the (new) element will be ignored.
const NEIGHBOURS: u8 = 3;