// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - color all significant pixels ordered by value

use crate::data_image::DataImage;
use crate::palette::Palette3;

// for Nebula like fractals
struct Pix {
    x: usize,
    y: usize,
    value: u32,
}

/*
fn perfectly_color_euler_values(data: &DataImage, palette3: &Palette3) {

    let width = data.width_x;
    let height = data.height_y;

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
            let (r, g ,b) = data.value_3_at(x, y);
            
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
    pixels_red.sort_by(|first, second| first.1.cmp(&second.1));
    pixels_green.sort_by(|first, second| first.1.cmp(&second.1));
    pixels_blue.sort_by(|first, second| first.1.cmp(&second.1));

    let all_pixels_total : u32 = width as u32 * height as u32;
    let all_pixels_non_zero_red : u32 = all_pixels_total - zero_value_elements_red;
    let all_pixels_non_zero_green : u32 = all_pixels_total - zero_value_elements_green;
    let all_pixels_non_zero_blue : u32 = all_pixels_total - zero_value_elements_blue;
    let palette_color_count : u32 = palette3.color_resolution();
    let single_color_use_red : u32 = all_pixels_non_zero_red / palette_color_count;
    let single_color_use_green : u32 = all_pixels_non_zero_green / palette_color_count;
    let single_color_use_blue : u32 = all_pixels_non_zero_blue / palette_color_count;
    let left_red : u32 = all_pixels_non_zero_red - (palette_color_count * single_color_use_red);
    let left_green : u32 = all_pixels_non_zero_green - (palette_color_count * single_color_use_green);
    let left_blue : u32 = all_pixels_non_zero_blue - (palette_color_count * single_color_use_blue);

    println!("------------------------------------");
    println!("All pixels to paint:         {}", all_pixels_total);
    println!("---------------------------> {}", (zero_value_elements_red + left_red + (single_color_use_red * palette_color_count)));
    println!("---------------------------> {}", (zero_value_elements_green + left_green + (single_color_use_green * palette_color_count)));
    println!("---------------------------> {}", (zero_value_elements_blue + left_blue + (single_color_use_blue * palette_color_count)));
    println!("Zero value pixels to paint:  {}", zero_value_elements_red);
    println!("Zero value pixels to paint:  {}", zero_value_elements_green);
    println!("Zero value pixels to paint:  {}", zero_value_elements_blue);
    println!("Non zero pixels to paint:    {}", all_pixels_non_zero_red);
    println!("Non zero pixels to paint:    {}", all_pixels_non_zero_green);
    println!("Non zero pixels to paint:    {}", all_pixels_non_zero_blue);
    println!("Spectrum, available colors: {}", palette_color_count);
    println!("Pixels per each color:      {}", single_color_use_red);
    println!("Pixels per each color:      {}", single_color_use_green);
    println!("Pixels per each color:      {}", single_color_use_blue);
    println!("left:                        {}", left_red);
    println!("left:                        {}", left_green);
    println!("left:                        {}", left_blue);
    println!("------------------------------------");

    // paint mismatched pixel amount with the least value color
    for pi_red in 0..(left_red + zero_value_elements_red) {
        let sp = pixels_red.get(pi_red);
        data.color(sp.x, sp.y, red, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_red {
            // color all these pixels with same color
            let sp = pixels_red.get(pi_red += 1);
            if sp.pixelValue() <= threshold {
                data.color(sp.x, sp.y, red, 0);
            } else {
                // perfect-color all significant pixels
                data.color(sp.x, sp.y, red, PaletteEuler3.getSpectrumValueRed(palette_color_index).getRed());
            }
        }
    }

    for pi_green in 0..(left_green + zero_value_elements_green) {
        sp = pixels_green.get(pi_green);
        data.color(sp.x, sp.y, green, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_green {
            // color all these pixels with same color
            sp = pixels_green.get(pi_green += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                data.color(sp.x, sp.y, green, 0);
            } else {
                // perfect-color all significant pixels
                data.color(sp.x, sp.y, green, PaletteEuler3.getSpectrumValueGreen(palette_color_index).getGreen());
            }
        }
    }

    for pi_blue in 0..(left_blue + zero_value_elements_blue) {
        sp = pixels_blue.get(pi_blue);
        data.color(sp.x, sp.y, blue, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_blue {
            // color all these pixels with same color
            sp = pixels_blue.get(pi_blue += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                data.color(sp.x, sp.y, blue, 0);
            } else {
                // perfect-color all significant pixels
                data.color(sp.x, sp.y, blue, PaletteEuler3.getSpectrumValueBlue(palette_color_index).getBlue());
            }
        }
    }

    // read 3 euler spectra colors and write image colors
    for y in 0..height {
        for x in 0..width {
            let r = data.value_at(x, y, red);
            let g = data.value_at(x, y, green);
            let b = data.value_at(x, y, blue);
            result_image.setRGB(x, y, Rgb([r, g, b]));
        }
    }
    // Behold, the coloring is perfect!
}
*/

#[cfg(test)]
mod tests {

    #[test]
    fn test_perfectly_color_euler_values() {
        // TODO
    }
}
