use crate::mathematician;
use crate::pixel::Spectra;
use crate::pixel::Spectra::{Blue, Green, Red};

fn translate_paths_to_pixel_grid() {
    /*
    for path in paths {
        for i in 0..path.size() - 1 {
            let tmp = path.get(i);
            // translate [re,im] to [px,py]
            AreaFinebrot.pointToPixel(m, tmp[0], tmp[1]);
            if m.good {
                added += 1;
                FractalEuler.colorsFor(m, i, path.size());
                PixelsEulerFinebrot.add(m.px, m.py, m.spectra);
            }
        }
    }
     */
}

fn colors_for(element_index: u32, path_length: u32) -> Spectra {
    if mathematician::is_prime(element_index) {
        return Red;
    }
    if mathematician::is_prime(path_length) {
        return Green;
    }
    Blue
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
