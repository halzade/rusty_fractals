use rusty_fractals::mathematician;
use crate::mem_euler::MemEuler;

// Fractal Euler type uses three color spectra for better mathematical analysis and better coloring results.
// Possible use as:
// - prime path length / el.order      -> Red spectrum
// - Fibonacci path lengths / el.order -> Green spectrum
// - other path lengths / el.order     -> Blue spectrum

fn add_calculation_path(path: Vec<[i64; 2]>) {
    // paths.add(path);
}

fn translate_paths_to_pixel_grid() {
    /*
    debug!("translate_paths_to_pixel_grid");

    let mut added = 0;
    for path in paths {
        for i in 0..path.size() - 1 {
            let tmp = path.get(i);
            // translate [re,im] to [px,py]
            AreaFinebrot.pointToPixel(m, tmp[0], tmp[1]);
            if m.good {
                added += 1;
                FractalEuler.colorsFor(m, i, path.size());
                PixelsEulerFinebrot.add(m.px, m.py, m.spectra);
            }
        }
    }
    debug!("* Added:   {}", added);

    /* remove elements which moved ouf of tiny area */
    removeElementsOutside();
    */
}

fn colors_for(m: MemEuler, element_index: u32, path_length: u32) {
    /*
    if mathematician::is_prime(element_index) {
        m.spectra = red;
        return;
    }
    if mathematician::is_prime(path_length) {
        m.spectra = green;
        return;
    }
    m.spectra = blue;
    */
}

fn calculate_path() {
    /*
    let mut iterator = 0;
    let length = 0;
    final MemEuler
    m = new
    MemEuler(el.origin_re, el.origin_im);
    while m.quad() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {
        /*
         * Investigate if this is a good calculation path
         * Don't create path data yet. Too many origin's don't produce good data
         * Most long expensive calculations end up inside Mandelbrot set
         */
        math(m, el.origin_re, el.origin_im);
        if AreaFinebrot.contains(m) {
            length += 1;
        }
        iterator += 1;
    }

    /* Verify divergent path length */
    if length > ITERATION_min && iterator < ITERATION_MAX {
        /*
         * This origin produced good data, record calculation path
         */
        m.reset(el.origin_re, el.origin_im);
        el.goodPath();
        final ArrayList < double
        [] > path = new
        ArrayList < > (length);
        for i in 0..iterator {
            math(m, el.origin_re, el.origin_im);
            if AreaFinebrot.contains(m) {
                path.add(new double[]{ m.re, m.im });
            }
        }
        el.set_finished_state(iterator, length);
        return path;
    } else {
        el.set_finished_state(iterator, length);
        return null;
    }
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}

