/**
 * Calculation paths
 * Dynamic data for Finebrot fractal. These double[] data will be projected to in[][] pixels and then colored.
 * As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the tiny finebrot Area.
 * Elements outside tiny finebrot Area are removed. Very short paths are also removed.
 * [re, im] representation as double[2] is better than 2x Double.
 */
const paths = new ArrayList< > ();

/**
 * All elements on calculation path are already inside displayed area
 * Because they are filtered like that during calculation
 */


fn remove_elements_outside() {
    log.debug("Remove elements which zoomed out");
    for (ArrayList < double[]> path : paths) {
        path.removeIf(el -> AreaFinebrot.isOutside(el[0], el[1]));
    }
    paths.removeIf(path -> path.size() < TOLERATE_PATH_LENGTH_min);
}

fn add_escape_path_long(ArrayList<double[]> path) {
    Stats.pathsNewPointsAmount += path.size();
    paths.add(path);
}

fn translate_paths_to_pixel_grid() {
    log.debug("translate_paths_to_pixel_grid()");

    int
    pixelsTotal = 0;

    final Mem
    m = new
    Mem();
    double
    []
    tmp;
    for (ArrayList < double[]> path : paths) {
        if (path != null) {
            for (int i = 0; i < path.size() -1; i+ +) {
                tmp = path.get(i);
                /* translate [re,im] to [px,py] */
                AreaFinebrot.pointToPixel(m, tmp[0], tmp[1]);
                if (m.good) {
                    pixelsTotal + +;
                    PixelsFinebrot.add(m.px, m.py);
                }
            }
        } else {
            log.error("path can't be null");
        }
    }
    log.debug("pixelsTotal:   " + pixelsTotal);

    /* remove elements which moved out of tiny area */
    removeElementsOutside();

    Stats.pathsTotalAmount = paths.size();
    Stats.pixelsValueTotal = pixelsTotal;
}