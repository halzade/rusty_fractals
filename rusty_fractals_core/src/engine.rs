use rusty_fractals_domain::domain;
use crate::fractal;

// to calculate zoom, sequence of images
pub struct Engine {
    domain: domain::Domain,
    fractal: fractal::Fractal,
}

impl Engine {
    fn run(&self) {
        let mut first = true;
        for it in 1.. {
            println!("{}", it);

            if first {
                first = false;
                self.domain.init_domain_elements();
            } else {
                self.domain.recalculate_pixels_positions_for_this_zoom();
            }

            fractal.calculate();

            // PathsFinebrot.translatePathsToPixelGrid();
            // MaskMandelbrot.maskFullUpdate();

            fractal.perfectlyColorValues();
            Application.repaintMandelbrotWindow();

            if (SAVE_IMAGES) {
                FractalImages.saveMandelbrotImages();
            }

            fractal.update();
            // image_pixels.clear()
            Application.zoomIn();
        }
    }
}

fn calculate() {
    let domain_full_chunked_and_wrapped = full_domain_as_wrapped_parts();
    Collections.shuffle(domain_full_chunked_and_wrapped);

    for part in domainFullChunkedAndWrapped {
        /*
         * Calculate independently each domain chunk
         */
        executor.execute(new CalculationPathThread(finebrotFractal, part));
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
// TODO
// rusty_fractals_result_lib


fn full_domain_as_wrapped_parts() {
    todo!()
}
