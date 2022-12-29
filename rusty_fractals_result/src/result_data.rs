use rusty_fractals_common::area::Area;
use crate::result_pixels::ResultPixels;

pub struct ResultData {
    // Dynamic Vec[re,im] calculation result data.
    // As zoom progress, points [re,im] are projected to new pixels [px,py] until they migrate out of the the tiny result_rea.
    // Elements outside of tiny result_rea are removed. Very short PATHS are also removed.
    // All elements on (calculation) path are already inside displayed result_area because they are filtered like that during the calculation.
    pub paths: Vec<Vec<[f64; 2]>>,
    pub area_result: Area,
}

impl ResultData {
    pub fn remove_elements_outside(&mut self) {
        log.debug("remove_elements_outside");
        for mut path in self.paths {
            path.retain(|&el| self.area_result.contains(el.0, el.1));
        }
        self.paths.retain(path | path.size() < fractal::MINIMUM_PATH_LENGTH);
    }

    pub fn add_escape_path_long(&mut self, path: Vec<[f64; 2]>) {
        self.paths.push(path);
    }

    pub fn translate_paths_to_pixel_grid(&mut self, result_pixels : &ResultPixels) {
        log.debug("translate_paths_to_pixel_grid()");

        let mut pixels_total = 0;

        for path in self.paths {
            for re_im in path {
                // translate [re,im] to [px,py]
                let re = re_im.0;
                let im = re_im.1;
                if self.area_result.contains(re, im) {
                    (px, py) = self.area_result.domain_point_to_result_pixel(re, im);
                    result_pixels.add(px, py);
                    pixels_total += 1;
                }
            }
        }
        log.debug("pixels_total:   " + pixels_total);

        /* remove elements which moved out of tiny area */
        self.remove_elements_outside();

        // Stats.pathsTotalAmount = PATHS.size();
        // Stats.pixelsValueTotal = pixels_total;
    }
}