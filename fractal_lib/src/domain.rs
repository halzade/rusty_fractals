use crate::domain_area::DomainArea;
use crate::{domain_area, domain_element, fractal};
use domain_element::MandelbrotElement;
use fractal::{HEIGHT_Y, WIDTH_X};

struct Domain {
    domain_area: domain_area::DomainArea,
    domain_elements: Vec<Vec<MandelbrotElement>>
}

impl Domain {
    /**
     * Makes small square subset of domain elements, will omit those already calculated.
     */
    fn make_chunk(
        &self,
        x_from: usize,
        x_to: usize,
        y_from: usize,
        y_to: usize,
    ) -> Vec<&MandelbrotElement> {
        let mut chunk: Vec<&MandelbrotElement> = Vec::new();
        for x in x_from..x_to {
            for y in y_from..y_to {
                let core_element: &MandelbrotElement = self.domain_elements[x]
                    .get(y)
                    .expect("domain_elements problem");
                if core_element.is_active_new() {
                    chunk.push(core_element);
                }
            }
        }
        chunk
    }
}

fn init_domain_elements(domain_area: DomainArea) -> Vec<Vec<MandelbrotElement>> {
    let mut vy: Vec<Vec<MandelbrotElement>> = Vec::new();
    for x in 0..WIDTH_X {
        let mut vx: Vec<MandelbrotElement> = Vec::new();
        for y in 0..HEIGHT_Y {
            vx.push(domain_element::init(
                domain_area.screen_to_domain_re(x),
                domain_area.screen_to_domain_im(y),
            ));
        }
        vy.push(vx);
    }
    vy
}
