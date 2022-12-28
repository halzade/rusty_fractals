use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rusty_fractals_domain::domain;
use crate::{fractal, machine};
use crate::machine::Machine;

// to calculate zoom, sequence of images
pub struct Engine {
    domain: domain::Domain,
    fractal: fractal::Fractal,
}

impl Engine {
    pub fn calculate(&self) {
        let mut first = true;

        for it in 1.. {
            println!("{}", it);

            if first {
                first = false;
                self.domain.init_domain_elements();
            } else {
                self.domain.recalculate_pixels_positions_for_this_zoom();
            }

            let coordinates_xy = self.domain.shuffled_calculation_coordinates();

            // Calculate independently and in parallel each domain chunks
            coordinates_xy.into_par_iter().for_each(
                |xy| machine::chunk_calculation(&self.domain, xy)
            );


            PathsFinebrot.translatePathsToPixelGrid();
            MaskMandelbrot.maskFullUpdate();

            fractal.perfectly_color_values();
            Application.repaint_mandelbrot_window();

            if SAVE_IMAGES {
                FractalImages.saveMandelbrotImages();
            }

            fractal.update();
            // image_pixels.clear()
            Application.zoomIn();
        }
    }
}

fn run() {
    for el in maskMandelbrotElementsPart {
        /*
         * Investigate calculation path for each mandelbrot pixel
         */
        final ArrayList < double
        [] > path = finebrotFractal.calculatePath(el);
        if path != null {
            /*
             * Removed lastIteration, lastVisitedRe, lastVisitedIm
             * There isn't continuation of unfinished iteration from previous calculation (ITERATION_MAX increased)
             * The element and its path is going to migrate out of screen soon.
             */
            PathsFinebrot.addEscapePathLong(path);
        }
    }
    if lastMandelbrotRefresh + 97 < currentTimeMillis() {
        /*
         * Handle refresh with calculation progress for all the threads
         */
        lastMandelbrotRefresh = currentTimeMillis();

        MaskMandelbrot.maskFullUpdate();
        Application.repaintMaskMandelbrotWindow();
    }
}
