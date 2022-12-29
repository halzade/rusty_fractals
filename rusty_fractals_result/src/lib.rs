pub mod palette;
pub mod palettes;
mod palette_utils;
pub mod result_pixels;
mod perfect_color_distribution;
pub mod result_data;


fn lib() {
    let cores: usize = num_cpus::get();
}