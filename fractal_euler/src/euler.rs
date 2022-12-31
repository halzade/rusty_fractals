use rusty_fractals_core::fractal::CALCULATION_BOUNDARY;
use rusty_fractals_core::mathematician::Mathematician;
use rusty_fractals_domain::domain_element::DomainElement;

/**
 * Fractal Euler type uses three color spectra for better mathematical analysis and better coloring results.
 * Possible use as:
 * - prime path lengths     -> Red spectrum
 * - Fibonacci path lengths -> Green spectrum
 * - other path lengths     -> Blue spectrum
 */

fn add_escape_path_long(path: Vec<[i64; 2]>) {
    paths.add(path);
}

fn translate_paths_to_pixel_grid() {
    log.debug("translate_paths_to_pixel_grid");

    let added = 0;
    for path in paths {
        for i in 0..path.size() - 1 {
            let tmp = path.get(i);
            /* translate [re,im] to [px,py] */
            AreaFinebrot.pointToPixel(m, tmp[0], tmp[1]);
            if m.good {
                added += 1;
                FractalEuler.colorsFor(m, i, path.size());
                PixelsEulerFinebrot.add(m.px, m.py, m.spectra);
            }
        }
    }
    log.debug("* Added:   " + added);

    /* remove elements which moved ouf of tiny area */
    removeElementsOutside();
}

fn colors_for(m: MemEuler, int elementIndex, int pathLength) {
    if Mathematician.isPrime(elementIndex) {
        m.spectra = red;
        return;
    }
    if Mathematician.isPrime(pathLength) {
        m.spectra = green;
        return;
    }
    m.spectra = blue;
}

fn calculate_path(el: &DomainElement) {
    let mut iterator = 0;
    let length = 0;
    final MemEuler
    m = new
    MemEuler(el.originRe, el.originIm);
    while m.quadrance() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {
        /*
         * Investigate if this is a good calculation path
         * Don't create path data yet. Too many origin's don't produce good data
         * Most long expensive calculations end up inside Mandelbrot set
         */
        math(m, el.originRe, el.originIm);
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
        m.reset(el.originRe, el.originIm);
        el.goodPath();
        final ArrayList < double
        [] > path = new
        ArrayList < > (length);
        for i in 0..iterator {
            math(m, el.originRe, el.originIm);
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
}
