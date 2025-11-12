// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - color all significant pixels ordered by value

use crate::constants::COLORING_THRESHOLD;
use crate::data_image::DataImage;
use crate::palette::Palette;

// for Nebula like fractals
struct Pix {
    x: usize,
    y: usize,
    value: u32,
}

pub fn perfectly_color_nebula_values(data: &DataImage, palette: &Palette) {
    println!("perfectly_color_nebula_values()");
    let width = data.width_x;
    let height = data.height_y;

    // Result pixels, order by value
    let mut pixels: Vec<Pix> = Vec::new();
    let mut zero_value_elements = 0;

    // read screen values
    for y in 0..height {
        for x in 0..width {
            let v = data.value_at(x, y);
            if v <= COLORING_THRESHOLD {
                zero_value_elements += 1;
            }
            pixels.push(Pix { x, y, value: v });
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|first, second| first.value.cmp(&second.value));

    let all_pixels_total: u32 = (width * height) as u32;
    let all_pixels_non_zero: u32 = all_pixels_total - zero_value_elements;
    let palette_color_count: u32 = palette.spectrum.len() as u32;
    let single_color_use: u32 = (all_pixels_non_zero as f64 / palette_color_count as f64) as u32;
    let left: u32 = all_pixels_non_zero - (palette_color_count * single_color_use);

    println!("------------------------------------");
    println!("All pixels to paint:        {:8}", all_pixels_total);
    println!(
        "---------------------------> {}",
        zero_value_elements + left + (single_color_use * palette_color_count)
    );
    println!("Zero value pixels to paint: {:8}", zero_value_elements);
    println!("Non zero pixels to paint:   {:8}", all_pixels_non_zero);
    println!("Spectrum, available colors: {:8}", palette_color_count);
    println!("Pixels per each color:      {:8}", single_color_use);
    println!("left:                       {:8}", left);
    println!("------------------------------------");

    // paint mismatched pixel amount with the least value color
    let mut pi = 0;
    for _ in 0..(left + zero_value_elements) {
        let sp = pixels.get(pi).expect("pixels error");
        pi += 1;
        data.color(sp.x, sp.y, palette.spectrum_value(0));
    }

    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use {
            // color all these pixels with same color
            let sp = pixels.get(pi).expect("pixels error");
            pi += 1;
            if sp.value <= COLORING_THRESHOLD {
                // color zero-value elements and low-value-noise with the darkest color
                data.color(sp.x, sp.y, palette.spectrum_value(0));
            } else {
                // perfect-color all significant pixels
                data.color(
                    sp.x,
                    sp.y,
                    palette.spectrum_value(palette_color_index as usize),
                );
            }
        }
    }
    assert_eq!(pixels.len(), pi);
    println!("painted:                     {}", pi);
    // Behold, the coloring is perfect!
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_perfectly_color_nebula_values() {
        // TODO
    }
}
