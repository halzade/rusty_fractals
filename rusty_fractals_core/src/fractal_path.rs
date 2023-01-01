use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_domain::domain_element::DomainElement;
use crate::mem::Mem;


fn calculate_iterations_mandelbrot(el: &DomainElement) {
    /*
    let mut iterator = 0;
    let m = MemCollatzConjecture(el.origin_re, el.origin_im);
    while m.quadrance() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {
        math(m, el.origin_re, el.origin_im);
        iterator += 1;
    }

    el.set_finished_state(iterator, m.quadrance());
    */
}


fn calculate_path_collatz(el: &DomainElement) {
    /*
    let mut iterator = 0;
    let length = 0;
    let m = MemCollatzConjecture(el.origin_re, el.origin_im);
    while m.quadrance() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {

        math(m, el.origin_re, el.origin_im);
        if AreaFinebrot.contains(m) {
            length += 1;
        }
        iterator += 1;
    }

    if length > ITERATION_min && iterator < ITERATION_MAX {
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


// Phoenix fractal parameters
// c, p
// protected double c;
// protected double p;

fn calculate_path_phoenix(el: &DomainElement) {
    /*
    let mut iterator = 0;
    let length = 0;
    let m = new MemPhoenix(el.origin_re, el.origin_im);
    while m.quadrance() < CALCULATION_BOUNDARY && iterator < ITERATION_MAX {
        math(m, el.origin_re, el.origin_im);
        if AreaFinebrot.contains(m) {
            length += 1;
        }
        iterator += 1;
    }

    if length > ITERATION_min && iterator < ITERATION_MAX {
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
