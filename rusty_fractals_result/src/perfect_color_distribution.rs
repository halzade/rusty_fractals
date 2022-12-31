// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - Color all significant pixels ordered by value

use constants::COLORING_THRESHOLD;
use rusty_fractals_common::constants;
use crate::result_pixels::ResultPixels;

fn comparator_mandelbrot(a, b) {
    let c = compare(a.pixelValue, b.pixelValue);
    if c == 0 {
        compare(a.qiad, b.qiad);
    }
    c
}

fn comparator_mandelbrot_zero() {
    // Comparator.comparingDouble(MandelbrotPixel::quad);
}

fn perfectly_color_values(mut result_pixels: &ResultPixels) {

    let width = result_pixels.width;
    let height = result_pixels.height;

    // Result pixels, order by value
    let mut pixels: Vec<[u32; 3]> = Vec::new();

    let mut zero_value_elements = 0;

    // read screen values
    for y in 0..height {
        for x in 0..width {
            let v = result_pixels.value_at(x, y);
            if v <= COLORING_THRESHOLD {
                zero_value_elements += 1;
            }
            pixels.add([v, x, y]);
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|a, b| a.1.cmp(&b.1));

    let all_pixels_total = width * height;
    let all_pixels_non_zero = all_pixels_total - zero_value_elements;
    let palette_color_count = Palette.colorResolution();
    let single_color_use = all_pixels_non_zero as f64 / palette_color_count as f64;
    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements + left + (single_color_use * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero);
    log.debug("Spectrum, available colors: " + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use);
    log.debug("left:                       " + left);
    log.debug("------------------------------------");

    // paint mismatched pixel amount with the least value colour
    for pi in 0..(left + zeroValueElements) {
        sp = pixels.get(pi);
        FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(0).getRGB());
    }

    /* color all remaining pixels, these are order by value */
    for paletteColourIndex in 0..paletteColorCount {
        for ci in 0..singleColorUse {
            /* color all these pixels with same color */
            sp = pixels.get(pi + +);
            if sp.pixelValue() <= coloringThreshold {
                /* color zero-value elements and low-value-noise with the darkest color */
                FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(0).getRGB());
            } else {
                /* perfect-color all significant pixels */
                FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(paletteColourIndex).getRGB());
            }
        }
    }
    log.debug("painted:                   " + pi);

    /*
     * Behold, the coloring is perfect
     */

    log.debug("clear pixels");
    pixels.clear();
}




/**
 * Finebrot pixels, order by value
 */
static final List<FinebrotPixel> pixelsRed = new ArrayList< > ();
static final List<FinebrotPixel> pixelsGreen = new ArrayList< > ();
static final List<FinebrotPixel> pixelsBlue = new ArrayList< > ();

fn perfectly_color_values_euler() {
    let zero_value_elements_red = 0;
    let zero_value_elements_green = 0;
    let zero_value_elements_blue = 0;

    /* identify zero and low-value elements as zero or noise */
    let threshold = 1;

    /* read screen values */
    for y in 0..RESOLUTION_HEIGHT {
        for x in 0..RESOLUTION_WIDTH {
            let r = PixelsEulerFinebrot.valueAt(x, y, red);
            let g = PixelsEulerFinebrot.valueAt(x, y, green);
            let b = PixelsEulerFinebrot.valueAt(x, y, blue);
            if r <= threshold {
                zero_value_elements_red += 1;
            }
            if g <= threshold {
                zero_value_elements_green += 1;
            }
            if b <= threshold {
                zero_value_elements_blue += 1;
            }
            pixelsRed.add(new FinebrotPixel(r, x, y));
            pixelsGreen.add(new FinebrotPixel(g, x, y));
            pixelsBlue.add(new FinebrotPixel(b, x, y));
        }
    }

    /*
     *  order pixels from the smallest to the highest value
     */
    pixelsRed.sort(comparator);
    pixelsGreen.sort(comparator);
    pixelsBlue.sort(comparator);

    let all_pixels_total = RESOLUTION_WIDTH * RESOLUTION_HEIGHT;
    let all_pixels_non_zero_red = all_pixels_total - zero_value_elements_red;
    let all_pixels_non_zero_green = all_pixels_total - zero_value_elements_green;
    let all_pixels_non_zero_blue = all_pixels_total - zero_value_elements_blue;
    let palette_color_count = PaletteEuler3.colorResolution(); // same
    let single_color_use_red = ((int)((double) all_pixels_non_zero_red / (double) palette_color_count));
    let single_color_use_green = ((int)((double) all_pixels_non_zero_green / (double) palette_color_count));
    let single_color_use_blue = ((int)((double) all_pixels_non_zero_blue / (double) palette_color_count));
    let left_red = all_pixels_non_zero_red - (palette_color_count * single_color_use_red);
    let left_green = all_pixels_non_zero_green - (palette_color_count * single_color_use_green);
    let left_blue = all_pixels_non_zero_blue - (palette_color_count * single_color_use_blue);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements_red + left_red + (single_color_use_red * palette_color_count)));
    log.debug("--------------------------->" + (zero_value_elements_green + left_green + (single_color_use_green * palette_color_count)));
    log.debug("--------------------------->" + (zero_value_elements_blue + left_blue + (single_color_use_blue * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements_red);
    log.debug("Zero value pixels to paint: " + zero_value_elements_green);
    log.debug("Zero value pixels to paint: " + zero_value_elements_blue);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_red);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_green);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_blue);
    log.debug("Spectrum, available colors: " + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use_red);
    log.debug("Pixels per each color:      " + single_color_use_green);
    log.debug("Pixels per each color:      " + single_color_use_blue);
    log.debug("left:                       " + left_red);
    log.debug("left:                       " + left_green);
    log.debug("left:                       " + left_blue);
    log.debug("------------------------------------");

    /* pixel index */
    let pi_red;
    FinebrotPixel
    sp;
    /* paint mismatched pixel amount with the least value colour */
    for pi_red in 0..(leftRed + zeroValueElementsRed) {
        sp = pixelsRed.get(pi_red);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), red, 0);
    }
    /* color all remaining pixels, these are order by value */
    for paletteColourIndex in 0..paletteColorCount {
        for ci in 0..singleColorUseRed {
            /* color all these pixels with same color */
            sp = pixelsRed.get(pi_red + +);
            if sp.pixelValue() <= threshold {
                PixelsEulerFinebrot.set(sp.px(), sp.py(), red, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), red, PaletteEuler3.getSpectrumValueRed(paletteColourIndex).getRed());
            }
        }
    }

    let pi_green;
    for pi_green in 0..(leftGreen + zeroValueElementsGreen) {
        sp = pixelsGreen.get(pi_green);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), green, 0);
    }
    /* color all remaining pixels, these are order by value */
    for paletteColourIndex in 0..paletteColorCount {
        for ci in 0..singleColorUseGreen {
            /* color all these pixels with same color */
            sp = pixelsGreen.get(pi_green + +);
            if sp.pixelValue() <= threshold {
                /* color zero-value elements and low-value-noise with the darkest color */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), green, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), green, PaletteEuler3.getSpectrumValueGreen(paletteColourIndex).getGreen());
            }
        }
    }

    let pi_blue;
    for piBlue in 0..(leftBlue + zeroValueElementsBlue) {
        sp = pixelsBlue.get(pi_blue);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, 0);
    }
    /* color all remaining pixels, these are order by value */
    for paletteColourIndex in 0..paletteColorCount {
        for ci in 0..singleColorUseBlue {
            /* color all these pixels with same color */
            sp = pixelsBlue.get(pi_blue + +);
            if sp.pixelValue() <= threshold {
                /* color zero-value elements and low-value-noise with the darkest color */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, PaletteEuler3.getSpectrumValueBlue(paletteColourIndex).getBlue());
            }
        }
    }

    log.debug("painted:                   " + pi_red);
    log.debug("painted:                   " + pi_green);
    log.debug("painted:                   " + pi_blue);

    /*
     * read 3 screen colors
     * write image colors
     */
    for y in 0..RESOLUTION_HEIGHT {
        for x in 0..RESOLUTION_WIDTH {
            let r = PixelsEulerFinebrot.valueAt(x, y, red);
            let g = PixelsEulerFinebrot.valueAt(x, y, green);
            let b = PixelsEulerFinebrot.valueAt(x, y, blue);
            FinebrotImage.setRGB(x, y, new Color(r, g, b).getRGB());
        }
    }

    /*
     * Behold, the coloring is perfect
     */

    log.debug("clear pixels");
    pixelsRed.clear();
    pixelsGreen.clear();
    pixelsBlue.clear();
    PixelsEulerFinebrot.clear();
}


/**
 * Mandelbrot pixels, order by value
 */
static final List<MandelbrotPixel> pixels = new ArrayList< > ();
static final List<MandelbrotPixel> pixelsZero = new ArrayList< > ();

const NEIGHBOR_COORDINATES: [[i8; 2]; 8] = [[-1, -1], [0, -1], [1, -1], [-1, 0], [1, 0], [-1, 1], [0, 1], [1, 1]];

private final MandelbrotPixel[][] field = new MandelbrotPixel[RESOLUTION_WIDTH][RESOLUTION_HEIGHT];


fn perfectly_color_values_mandelbrot() {
    log.debug("perfectly_color_values()");

    let zero_value_elements = 0;

    /* read screen values */

    for y in 0..RESOLUTION_HEIGHT {
        for x in 0..RESOLUTION_WIDTH {
            final MandelbrotElement
            el = PixelsMandelbrot.elAt(x, y);
            if el.value == 0 {
                zero_value_elements += 1;
                pixelsZero.add(MandelbrotPixelFactory.make(el, x, y));
            } else {
                MandelbrotPixel
                mp = MandelbrotPixelFactory.make(el, x, y);
                pixels.add(mp);
                field[x][y] = mp;
            }
        }
    }

    /*
     *  order pixels from the smallest to the highest value
     */
    pixels.sort(comparatorMandelbrot);
    pixelsZero.sort(comparator_mandelbrot_zero);

    let all_pixels_total = RESOLUTION_WIDTH * RESOLUTION_HEIGHT;
    let all_pixels_non_zero = all_pixels_total - zero_value_elements;
    let palette_color_count = Palette.colorResolution();
    let single_color_use = ((int)((double) all_pixels_non_zero / (double) palette_color_count));

    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements + left + (single_color_use * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero);
    log.debug("Spectrum, available colors:>" + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use);
    log.debug("left:                       " + left);
    log.debug("------------------------------------");


    MandelbrotPixel
    mp;
    let pi = 0;

    /* paint mismatched pixel amount with the least but not the lowest value colour */
    while pi < left {
        mp = pixels.get(pi + +);
        MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(0).getRGB());
    }

    let palette_colour_index = 0;
    while palette_colour_index < palette_color_count {
        for ci in 0..singleColorUse {
            mp = pixels.get(pi + +);
            mp.colorValue(palette_colour_index);
            MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(palette_colour_index).getRGB());
        }
        palette_colour_index += 1;
    }

    Assert.assertEquals(pixels.size(), pi);

    /*
     * Fix black dots caused by quad inverse imperfection
     * Keep incorrect qud results ~
     */

    for mpp in pixels {
        let average_colour_index = ac_if_black_dot(mpp);
        if average_colour_index != -1 {
            mpp.colorValue(average_colour_index);
            MandelbrotImage.setRGB(mpp.px, mpp.py, Palette.getSpectrumValue(average_colour_index).getRGB());
        }
    }

    /*
     * PAINT INSIDES OF MANDELBROT SET
     */

    let zero_palette_color_count = PaletteZero.colorResolution();
    let zero_single_color_use = ((int)((double) zero_value_elements / (double) zero_palette_color_count));
    let zero_left = zero_value_elements - (zero_palette_color_count * zero_single_color_use);

    log.info("zero_palette_color_count:    > " + zero_palette_color_count);
    log.info("zero_single_color_use:       > " + zero_single_color_use);
    log.info("zero_left:                 > " + zero_left);

    let piz;
    for piz in 0..zeroLeft {
        mp = pixelsZero.get(piz);
        MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(0).getRGB());
    }
    for zeroPaletteColourIndex in 0..zeroPaletteColorCount {
        for ci in 0..zeroSingleColorUse {
            /* color all these pixels with same color */
            mp = pixelsZero.get(piz + +);
            MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(zeroPaletteColourIndex).getRGB());
        }
    }

    log.debug("painted:                   " + pi);

    /*
     * Behold, the coloring is perfect
     */

    log.debug("clear pixels");
    pixels.clear();
    pixelsZero.clear();
}

/**
 * Return average color of neighbour elements
 */
fn ac_if_black_dot(MandelbrotPixel mp) -> i32 {
    let pv = mp.pixelValue;
    let sum = 0;
    let neighbours = 0;
    for c in NEIGHBOR_COORDINATES {
        let a = mp.px + c[0];
        let b = mp.py + c[1];
        let n = check_domain(a, b);
        if n != null {
            if Math.abs(pv - n.pixelValue) > 2 {
                /* verify only one value difference gradient */
                return -1;
            }
            sum += n.colorValue;
            neighbours += 1;
        } else {
            /* don't fix elements of edges */
            -1
        }
    }

    let cv = mp.colorValue;
    let average_value = (int)(sum / neighbours);

    if cv < average_value - 5 {
        /* darker */
        average_value
    }
    -1
}