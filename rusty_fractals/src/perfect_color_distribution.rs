// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - color all significant pixels ordered by value

use crate::data_image::DataImage;
use crate::palette::Palette;
use std::cmp::Ordering::Equal;

// for Mandelbrot like fractals
struct Mix {
    x: usize,
    y: usize,
    value: u32,
    // the black interior of set
    quad: f64,
}

pub fn perfectly_color_mandelbrot_values(
    data: &DataImage,
    palette: &Palette,
    palette_zero: &Palette,
) {
    println!("perfectly_color_mandelbrot_values()");

    let width = data.width_x;
    let height = data.height_y;

    // Result pixels, order by value
    let mut pixels: Vec<Mix> = Vec::new();
    let mut pixels_zero: Vec<Mix> = Vec::new();

    let mut zero_value_elements = 0;

    for y in 0..height {
        for x in 0..width {
            let (value, _, quad, _) = data.values_state_quad_color_at(x, y);
            if value == 0 {
                zero_value_elements += 1;
                pixels_zero.push(Mix { x, y, value, quad });
            } else {
                pixels.push(Mix { x, y, value, quad });
            }
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|first, second| {
        let ordering = first.value.cmp(&second.value);
        if ordering == Equal {
            return second.quad.total_cmp(&first.quad);
        }
        ordering
    });

    // inverted order
    pixels_zero.sort_by(|first, second| second.quad.total_cmp(&first.quad));

    let all_pixels_total: u32 = (width * height) as u32;
    let all_pixels_non_zero: u32 = all_pixels_total - zero_value_elements;
    let palette_color_count: u32 = palette.spectrum.len() as u32;
    let single_color_use: u32 = (all_pixels_non_zero as f64 / palette_color_count as f64) as u32;

    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

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
    for _ in 0..left {
        let mp = pixels.get(pi).expect("pixels error");
        pi += 1;
        data.color(mp.x, mp.y, palette.spectrum_value(0));
    }

    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use {
            // color all these pixels with same color
            let mp = pixels.get(pi).expect("pixels error");
            pi += 1;
            // perfect-color all significant pixels
            data.color(
                mp.x,
                mp.y,
                palette.spectrum_value(palette_color_index as usize),
            );
        }
    }
    let pixels_length = pixels.len();
    assert_eq!(pixels.len(), pi);

    // Paint insides of Mandelbrot set
    let zero_palette_color_count = palette_zero.spectrum.len() as u32;
    let zero_single_color_use =
        (zero_value_elements as f64 / zero_palette_color_count as f64) as u32;
    let zero_left = zero_value_elements - (zero_palette_color_count * zero_single_color_use);

    println!("zero_palette_color_count:   {}", zero_palette_color_count);
    println!("zero_single_color_use:      {}", zero_single_color_use);
    println!("zero_left:                   {}", zero_left);
    let mut piz = 0;
    for _ in 0..zero_left {
        let mp = pixels_zero.get(piz).expect("pixel error");
        piz += 1;
        data.color(mp.x, mp.y, palette_zero.spectrum_value(0usize));
    }
    for zero_palette_color_index in 0..zero_palette_color_count {
        for _ in 0..zero_single_color_use {
            // color all these pixels with same color
            let mp = pixels_zero.get(piz).expect("pixel error");
            piz += 1;
            data.color(
                mp.x,
                mp.y,
                palette_zero.spectrum_value(zero_palette_color_index as usize),
            );
        }
    }
    assert_eq!(pixels_zero.len(), piz);
    assert_eq!(pixels_zero.len() + pixels_length, all_pixels_total as usize);
    assert_eq!(all_pixels_total as usize, pi + piz);
    println!("painted:                     {}", pi + piz);
    // Behold, the coloring is perfect!
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_perfectly_color_mandelbrot_values() {
        // TODO
    }
}
