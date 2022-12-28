use rusty_fractals_domain::domain_element::DomainElement;
use crate::fractal::CALCULATION_BOUNDARY;
use crate::mem::Mem;

fn calculate_iterations_mandelbrot(el: &DomainElement) {
    let iterator = 0;
    final MemCollatzConjecture
    m = new
    MemCollatzConjecture(el.originRe, el.originIm);
    while m.quadrance() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {
        math(m, el.originRe, el.originIm);
        iterator += 1;
    }

    el.setFinishedState(iterator, m.quadrance());
}

fn calculate_path_collatz(el: &DomainElement) {
    let iterator = 0;
    let length = 0;
    final MemCollatzConjecture
    m = new
    MemCollatzConjecture(el.originRe, el.originIm);
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
            /* It is 1.68x faster to calculate path twice, but recording exclusively good paths */
            math(m, el.originRe, el.originIm);
            if AreaFinebrot.contains(m) {
                path.add(new double[]{ m.re, m.im });
            }
        }
        el.setFinishedState(iterator, length);
        return path;
    } else {
        el.setFinishedState(iterator, length);
        return null;
    }
}


// Phoenix fractal parameters
// c, p
protected double c;
protected double p;

fn calculate_path_phoenix(el: &DomainElement) {
    let iterator = 0;
    let length = 0;
    final MemPhoenix
    m = new
    MemPhoenix(el.originRe, el.originIm);
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
        el.setFinishedState(iterator, length);
        return path;
    } else {
        el.setFinishedState(iterator, length);
        return null;
    }
}