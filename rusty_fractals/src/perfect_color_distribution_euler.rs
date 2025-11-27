// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - color all significant pixels ordered by value

use crate::data_image::DataImage;
use crate::palette::Palette3;
use crate::palettes::palette_3_rgb;

// for Nebula like fractals
struct Pix {
    x: usize,
    y: usize,
    value: u32,
}

impl Pix {
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[rustfmt::skip]
pub fn perfectly_color_euler_values(data: &DataImage) {
    // todo
    
    let width = data.width_x;
    let height = data.height_y;

    let palette3: Palette3 = palette_3_rgb();

    // Result pixels, order by value
    let mut pixels_red: Vec<Pix> = Vec::new();
    let mut pixels_green: Vec<Pix> = Vec::new();
    let mut pixels_blue: Vec<Pix> = Vec::new();

    let mut zero_value_elements_red = 0;
    let mut zero_value_elements_green = 0;
    let mut zero_value_elements_blue = 0;

    // identify zero and low-value elements as zero or noise
    let threshold = 1;

    // read screen values
    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = data.value_at3(x, y);

            if r <= threshold {
                zero_value_elements_red += 1;
            }
            if g <= threshold {
                zero_value_elements_green += 1;
            }
            if b <= threshold {
                zero_value_elements_blue += 1;
            }
            pixels_red.push(Pix { x, y, value: r });
            pixels_green.push(Pix { x, y, value: g });
            pixels_blue.push(Pix { x, y, value: b });
        }
    }

    // order pixels from the smallest to the highest value
    pixels_red.sort_by(|first, second| first.value.cmp(&second.value));
    pixels_green.sort_by(|first, second| first.value.cmp(&second.value));
    pixels_blue.sort_by(|first, second| first.value.cmp(&second.value));

    let all_pixels_total: u32 = width as u32 * height as u32;
    let all_pixels_non_zero_red: u32 = all_pixels_total - zero_value_elements_red;
    let all_pixels_non_zero_green: u32 = all_pixels_total - zero_value_elements_green;
    let all_pixels_non_zero_blue: u32 = all_pixels_total - zero_value_elements_blue;

    let palette_color_count: u32 = palette3.spectrum_red.len() as u32;
    assert_eq!(palette3.spectrum_red.len(), palette3.spectrum_blue.len());
    assert_eq!(palette3.spectrum_red.len(), palette3.spectrum_green.len());

    let single_color_use_red: u32 = all_pixels_non_zero_red / palette_color_count;
    let single_color_use_green: u32 = all_pixels_non_zero_green / palette_color_count;
    let single_color_use_blue: u32 = all_pixels_non_zero_blue / palette_color_count;
    let left_red: u32 = all_pixels_non_zero_red - (palette_color_count * single_color_use_red);
    let left_green: u32 = all_pixels_non_zero_green - (palette_color_count * single_color_use_green);
    let left_blue: u32 = all_pixels_non_zero_blue - (palette_color_count * single_color_use_blue);

    println!("------------------------------------");
    println!("All pixels to paint:        {:8}", all_pixels_total);
    println!("--------------------------> {:8}", zero_value_elements_red + left_red + (single_color_use_red * palette_color_count));
    println!("--------------------------> {:8}", zero_value_elements_green + left_green + (single_color_use_green * palette_color_count));
    println!("--------------------------> {:8}", zero_value_elements_blue + left_blue + (single_color_use_blue * palette_color_count));
    println!("Zero value pixels to paint: {:8}", zero_value_elements_red);
    println!("Zero value pixels to paint: {:8}", zero_value_elements_green);
    println!("Zero value pixels to paint: {:8}", zero_value_elements_blue);
    println!("Non zero pixels to paint:   {:8}", all_pixels_non_zero_red);
    println!("Non zero pixels to paint:   {:8}", all_pixels_non_zero_green);
    println!("Non zero pixels to paint:   {:8}", all_pixels_non_zero_blue);
    println!("Spectrum, available colors: {:8}", palette_color_count);
    println!("Pixels per each color:      {:8}", single_color_use_red);
    println!("Pixels per each color:      {:8}", single_color_use_green);
    println!("Pixels per each color:      {:8}", single_color_use_blue);
    println!("left:                       {:8}", left_red);
    println!("left:                       {:8}", left_green);
    println!("left:                       {:8}", left_blue);
    println!("------------------------------------");

    // paint mismatched pixel amount with the least value color
    let mut pi_red = 0;
    while pi_red < (left_red + zero_value_elements_red) as usize {
        let sp = pixels_red.get(pi_red).unwrap();
        data.color_r(sp.x, sp.y, 0);
        pi_red += 1;
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count as usize {
        for _ in 0..single_color_use_red {
            // color all these pixels with same color
            pi_red += 1;
            let sp = pixels_red.get(pi_red).unwrap();
            if sp.value() <= threshold {
                data.color_r(sp.x, sp.y, 0);
            } else {
                // perfect-color all significant pixels
                data.color_r(sp.x, sp.y, palette_color_index);
            }
        }
    }

    let mut pi_green = 0;
    while pi_green < (left_green + zero_value_elements_green) as usize {
        let _sp = pixels_green.get(pi_green).unwrap();
        // data.color_g(sp.x, sp.y, palette3.spectrum_value_green(0));

        pi_green += 1;
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count as usize {
        for _ in 0..single_color_use_green {
            // color all these pixels with same color
            pi_green += 1;
            let sp = pixels_green.get(pi_green).unwrap();
            if sp.value() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                data.color_g(sp.x, sp.y, 0);
            } else {
                // perfect-color all significant pixels
                data.color_g(sp.x, sp.y, palette_color_index);
            }
        }
    }

    let mut pi_blue = 0;
    while pi_blue < (left_blue + zero_value_elements_blue) as usize {
        let sp = pixels_blue.get(pi_blue).unwrap();
        data.color_b(sp.x, sp.y, 0);

        pi_blue += 1;
    }
    // color all remaining pixels, these are order by value
    for _palette_color_index in 0..palette_color_count as usize {
        for _ in 0..single_color_use_blue {
            // color all these pixels with same color
            pi_blue += 1;
            let sp = pixels_blue.get(pi_blue).unwrap();
            if sp.value() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                // todo
                // data.color_b(sp.x, sp.y, palette3.spectrum_value_blue(0));
            } else {
                // perfect-color all significant pixels
                // data.color_b(sp.x, sp.y, palette3.spectrum_value_blue(palette_color_index));
            }
        }
    }

    // read 3 euler spectra colors and write image colors
    for y in 0..height {
        for x in 0..width {
            data.color_at3(x, y);
        }
    }
    // Behold, the coloring is perfect!
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_perfectly_color_euler_values() {
        // TODO
    }
}
