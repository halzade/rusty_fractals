// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - color all significant pixels ordered by value

use crate::constants::COLORING_THRESHOLD;
use crate::data_image::DataImage;
use crate::palette::Palette;
use std::cmp::Ordering::Equal;

// for Nebula like fractals
struct Pix {
    x: usize,
    y: usize,
    value: u32,
}

// for Mandelbrot like fractals
struct Mix {
    x: usize,
    y: usize,
    value: u32,
    // the black interior of set
    quad: f64,
    quid: f64,
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
    println!("All pixels to paint:         {}", all_pixels_total);
    println!(
        "---------------------------> {}",
        zero_value_elements + left + (single_color_use * palette_color_count)
    );
    println!("Zero value pixels to paint:  {}", zero_value_elements);
    println!("Non zero pixels to paint:    {}", all_pixels_non_zero);
    println!("Spectrum, available colors: {}", palette_color_count);
    println!("Pixels per each color:      {}", single_color_use);
    println!("left:                        {}", left);
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
        result_image.put_pixel(sp.x, sp.y, red, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_red {
            // color all these pixels with same color
            let sp = pixels_red.get(pi_red += 1);
            if sp.pixelValue() <= threshold {
                result_image.put_pixel(sp.x, sp.y, red, 0);
            } else {
                // perfect-color all significant pixels
                result_image.put_pixel(sp.x, sp.y, red, PaletteEuler3.getSpectrumValueRed(palette_color_index).getRed());
            }
        }
    }

    for pi_green in 0..(leftGreen + zeroValueElementsGreen) {
        sp = pixelsGreen.get(pi_green);
        result_image.put_pixel(sp.x, sp.y, green, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_green {
            // color all these pixels with same color
            sp = pixelsGreen.get(pi_green += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                result_image.put_pixel(sp.x, sp.y, green, 0);
            } else {
                // perfect-color all significant pixels
                result_image.put_pixel(sp.x, sp.y, green, PaletteEuler3.getSpectrumValueGreen(palette_color_index).getGreen());
            }
        }
    }

    for pi_blue in 0..(leftBlue + zeroValueElementsBlue) {
        sp = pixelsBlue.get(pi_blue);
        result_image.put_pixel(sp.x, sp.y, blue, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_color_index in 0..palette_color_count {
        for _ in 0..single_color_use_blue {
            // color all these pixels with same color
            sp = pixelsBlue.get(pi_blue += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                result_image.put_pixel(sp.x, sp.y, blue, 0);
            } else {
                // perfect-color all significant pixels
                result_image.put_pixel(sp.x, sp.y, blue, PaletteEuler3.getSpectrumValueBlue(palette_color_index).getBlue());
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

const NEIGHBOR_COORDINATES: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

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
            let (value, _, quad, quid, _) = data.values5_at(x, y);
            if value == 0 {
                zero_value_elements += 1;
                pixels_zero.push(Mix {
                    x,
                    y,
                    value,
                    quad,
                    quid,
                });
            } else {
                pixels.push(Mix {
                    x,
                    y,
                    value,
                    quad,
                    quid,
                });
            }
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|first, second| {
        let ordering = first.value.cmp(&second.value);
        if ordering == Equal {
            return first.quid.total_cmp(&second.quid);
        }
        ordering
    });
    pixels_zero.sort_by(|first, second| first.quad.total_cmp(&second.quad));

    let all_pixels_total: u32 = (width * height) as u32;
    let all_pixels_non_zero: u32 = all_pixels_total - zero_value_elements;
    let palette_color_count: u32 = palette.spectrum.len() as u32;
    let single_color_use: u32 = (all_pixels_non_zero as f64 / palette_color_count as f64) as u32;

    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

    println!("------------------------------------");
    println!("All pixels to paint:         {}", all_pixels_total);
    println!(
        "---------------------------> {}",
        zero_value_elements + left + (single_color_use * palette_color_count)
    );
    println!("Zero value pixels to paint:  {}", zero_value_elements);
    println!("Non zero pixels to paint:    {}", all_pixels_non_zero);
    println!("Spectrum, available colors: {}", palette_color_count);
    println!("Pixels per each color:      {}", single_color_use);
    println!("left:                        {}", left);
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

    // Fix black dots caused by quad inverse imperfection
    // Keep incorrect quad results
    for mpp in pixels {
        let average_color_index = ac_if_black_dot(&mpp, data);
        if average_color_index != -1 {
            // let mpp.colorValue(average_color_index);
            data.color(
                mpp.x,
                mpp.y,
                palette.spectrum_value(average_color_index as usize),
            );
        }
    }

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

// Return average color of neighbour elements
fn ac_if_black_dot(mp: &Mix, data: &DataImage) -> i32 {
    let width = data.width_x;
    let height = data.height_y;
    let pv = mp.value;
    let mut sum = 0;
    let mut neighbours = 0;
    for c in NEIGHBOR_COORDINATES {
        let x = mp.x as i32 + c[0];
        let y = mp.y as i32 + c[1];
        if check_domain(x, y, width, height) {
            let neighbor_value = data.value_at(x as usize, y as usize);
            if (pv as i32 - neighbor_value as i32).abs() > 2 {
                // verify only one value difference gradient
                return -1;
            }
            sum += neighbor_value;
            neighbours += 1;
        } else {
            // don't fix elements of edges
            return -1;
        }
    }
    let cv = mp.value as i32;
    let average_value = (sum as f64 / neighbours as f64) as i32;

    if cv < average_value - 5 {
        // darker
        return average_value;
    }
    -1
}

fn check_domain(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
