

// to calculate zoom, sequence of images
pub struct Engine {
    domain : domain::Domain,
    fractal : fractal::Fractal
}

impl Engine {
    fn run(&self) {
        let mut first= true;
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

    for (ArrayList < MaskMandelbrotElement> part : domainFullChunkedAndWrapped) {
        /*
         * Calculate independently each domain chunk
         */
        executor.execute(new CalculationPathThread(finebrotFractal, part));
    }
}

// TODO
// rusty_fractals_result_lib


fn full_domain_as_wrapped_parts() {
    todo!()
}
