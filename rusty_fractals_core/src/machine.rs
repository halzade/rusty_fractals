use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rusty_fractals_domain::domain;
use fractal_paths;
use crate::fractal;

// to calculate single image
pub struct Machine {
    pub domain: domain::Domain,
}

impl Machine {
    pub fn calculate(&self) {
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        // Calculate independently and in parallel each domain chunks
        coordinates_xy.into_par_iter().for_each(
            |xy| chunk_calculation(xy)
        );

        PathsFinebrot.translatePathsToPixelGrid();
        MaskMandelbrot.maskFullUpdate();

        fractal.perfectly_color_values();
        Application.repaint_mandelbrot_window();
    }

    fn chunk_calculation(&self, xy: [u32; 2]) {
        let chunk_size_x = self.width / 20;
        let chunk_size_y = self.height / 20;

        let wrapped_chunk = self.domain.make_chunk(
            (xy[0] * chunk_size_x) as usize, ((xy[0] + 1) * chunk_size_x) as usize,
            (xy[1] * chunk_size_y) as usize, ((xy[1] + 1) * chunk_size_y) as usize,
        );
        for el in wrapped_chunk {
            fractal_paths::calculate_path_finite(el);
        }
    }
}
