use rusty_fractals_domain::domain_element::DomainElement;

fn calculate_path(el: &DomainElement) {
    let mut iterator = 0;
    let length = 0;
    final Mem
    m = new
    Mem(el.originRe, el.originIm);
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

    /*
     * Verify NON-divergent path length
     */
    if length > ITERATION_min && iterator == ITERATION_MAX {
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
