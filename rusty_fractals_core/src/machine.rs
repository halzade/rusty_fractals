use rusty_fractals_domain::domain;
use crate::fractal;

// to calculate single image
pub struct Machine {
    pub domain: domain::Domain
}

impl Machine {
    pub fn calculate(&self) {

        let domain_full_chunked_and_wrapped = full_domain_as_wrapped_parts();
        Collections.shuffle(domain_full_chunked_and_wrapped);

        domain_full_chunked_and_wrapped.par_iter().all(
            // Calculate independently each domain chunk
            CalculationPathThread
        );

        // PathsFinebrot.translatePathsToPixelGrid();
        // MaskMandelbrot.maskFullUpdate();

        fractal.perfectlyColorValues();
        Application.repaintMandelbrotWindow();

    }
}

fn full_domain_as_wrapped_parts() {
    todo!()
}
