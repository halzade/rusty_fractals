/**
* Calculation PATHS
* Dynamic data for Finebrot fractal. These double[] data will be projected to in[][] pixels and then colored.
* As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the tiny finebrot Area.
* Elements outside tiny finebrot Area are removed. Very short PATHS are also removed.
* [re, im] representation as double[2] is better than 2x Double.

* All elements on calculation path are already inside displayed area
* Because they are filtered like that during calculation
 */

pub struct ResultData {
    mut paths: Vec<Vec<[f64; 1]>>,
}

impl ResultData {
    pub fn remove_elements_outside() {
        log.debug("Remove elements which zoomed out");
        for path in PATHS {
            path.removeIf(el -> AreaFinebrot.isOutside(el[0], el[1]));
        }
        PATHS.removeIf(path -> path.size() < TOLERATE_PATH_LENGTH_min);
    }

    pub fn add_escape_path_long(&mut self, path: Vec<[f64; 1]>) {
        Stats.pathsNewPointsAmount += path.size();
        self.paths.push(path);
    }

    pub fn translate_paths_to_pixel_grid() {
        log.debug("translate_paths_to_pixel_grid()");

        let pixels_total = 0;

        final Mem
        m = new
        Mem();
        double
        []
        tmp;
        for path in PATHS {
            for i in 0..path.size() - 1 {
                tmp = path.get(i);
                /* translate [re,im] to [px,py] */
                AreaFinebrot.pointToPixel(m, tmp[0], tmp[1]);
                if m.good {
                    pixels_total += 1;
                    PixelsFinebrot.add(m.px, m.py);
                }
            }
        }
        log.debug("pixels_total:   " + pixels_total);

        /* remove elements which moved out of tiny area */
        removeElementsOutside();

        Stats.pathsTotalAmount = PATHS.size();
        Stats.pixelsValueTotal = pixels_total;
    }
}