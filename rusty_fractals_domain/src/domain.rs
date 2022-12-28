use crate::domain_area::DomainArea;
use crate::{domain_area, domain_element, NEIGHBOURS};
use domain_element::DomainElement;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::SquareAlter;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Domain {
    pub width: u32,
    pub height: u32,
    pub domain_area: domain_area::DomainArea,
    pub domain_elements: Vec<Vec<DomainElement>>,
}

impl Domain {
    /**
     * Makes small square subset of domain elements, will omit those already calculated.
     */
    pub fn make_chunk(&self, x_from: usize, x_to: usize, y_from: usize, y_to: usize) -> Vec<&DomainElement> {
        let mut chunk: Vec<&DomainElement> = Vec::new();
        for x in x_from..x_to {
            for y in y_from..y_to {
                let core_element: &DomainElement = self.domain_elements[x]
                    .get(y)
                    .expect("domain_elements problem");
                if core_element.is_active_new() {
                    chunk.push(core_element);
                }
            }
        }
        chunk
    }

    fn check_domain(&self, x: u32, y: u32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn shuffled_calculation_coordinates(&self) -> Vec<[u32; 2]> {
        let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();
        for x in 0..19 {
            for y in 0..19 {
                coordinates_xy.push([x, y]);
            }
        }
        coordinates_xy.shuffle(&mut thread_rng())
    }


    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    fn wrap(&self, rm: ResolutionMultiplierm, odd: bool) {
        if rm == SquareAlter {
            let d = self.domain_area.plank() / 3;
            if odd {
                domainFull.add(activeNew(elementZero.originRe + d, elementZero.originIm + d));
                domainFull.add(activeNew(elementZero.originRe - d, elementZero.originIm - d));
            } else {
                domainFull.add(activeNew(elementZero.originRe - d, elementZero.originIm + d));
                domainFull.add(activeNew(elementZero.originRe + d, elementZero.originIm - d));
            }
        } else {
            let multiplier = self.resolve_multiplier(rm);

            let d = self.domain_area.plank() / multiplier;
            let half = (multiplier - 1) / 2;
            /* This fills the pixel with multiple points */
            for x in -half..half {
                for y in -half..half {
                    if x != 0 || y != 0 {
                        domainFull.add(activeNew(elementZero.originRe + (x * d), elementZero.originIm + (y * d)));
                    }
                    /* else do nothing, there already is element0 for the center of this pixel */
                }
            }
        }
    }

    fn resolve_multiplier(rm: ResolutionMultiplier) -> f64 {
        match rm {
            ResolutionMultiplier::None => 1.0,
            ResolutionMultiplier::Square3 => 3.0,
            ResolutionMultiplier::Square5 => 5.0,
            ResolutionMultiplier::Square11 => 11.0,
            ResolutionMultiplier::Square51 => 51.0,
            ResolutionMultiplier::Square101 => 101.0,
            ResolutionMultiplier::SquareAlter => 1.0
        }
    }

    fn mask_full_update() {
        for y in 0..HEIGHT_Y {
            for x in 0..WIDTH_X {
                MaskMandelbrotImage.setRGB(x, y, colorForState(domain_elements[x][y]).getRGB());
            }
        }
    }

    // This is called after calculation finished, zoom was called and new area measures recalculated
    fn recalculate_pixels_positions_for_this_zoom() {
        // Scan domain elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, all the next calculations will be skipped.

        let elements_to_move = Vec::new();

        for y in 0..HEIGHT_Y {
            for x in 0..WIDTH_X {
                let el = domain_elements[xx][yy];
                // There was already zoom in, the new area is smaller
                if AreaMandelbrot.contains(el.originRe, el.originIm) {
                    // Element did not move out of the zoomed in area
                    elements_to_move.push(el);
                }
            }
        }


        /*
         * If there is a conflict, two or more points moved to same pixel, then use the active one if there is any.
         * Don't drop conflicts around, simply calculate new elements in the next calculation iteration.
         */

        for el in elementsToRemember {
            /* translate [px,py] to [re,im] */
            AreaMandelbrot.pointToPixel(m, el.originRe, el.originIm);

            if m.good {
                filledAlready = domain_elements[m.px][m.py];
                if filledAlready != null {
                    /* conflict */
                    if filledAlready.hasWorseStateThen(el) {
                        /*
                         * Replace by element with better state
                         * Better to delete the other one, then to drop it to other empty px.
                         * That would cause problem with optimization, better calculate new and shiny px.
                         */
                        domain_elements[m.px][m.py] = el;
                    }
                } else {
                    /* Good, there is no conflict */
                    domain_elements[m.px][m.py] = el;
                }
            }
        }

        /*
         * Repaint with only moved elements
         */
        maskFullUpdate();
        Application.repaintMaskMandelbrotWindow();

        /*
         * Create new elements on positions where nothing was moved to
         */
        MaskMandelbrotElement
        el;
        for y in 0..RESOLUTION_HEIGHT {
            for x in 0..RESOLUTION_WIDTH {
                el = domain_elements[x][y];
                if (el == null) {
                    AreaMandelbrot.screenToDomainCarry(m, x, y);
                    if (allNeighborsFinishedTooLong(x, y)) {
                        /* Calculation for some positions should be skipped as they are too far away form any long successful divergent position */
                        domain_elements[x][y] = hibernatedDeepBlack(m.re, m.im);
                    } else {
                        domain_elements[x][y] = activeNew(m.re, m.im);
                    }
                } else {
                    /* If relevant, mark it as element from previous calculation iteration */
                    el.past();
                }
            }
        }

        elementsToRemember.clear();
    }

    /**
     * Verify if any neighbor px,py finished well, long or at least too short.
     * This method identifies deep black convergent elements of Mandelbrot set interior.
     * Don't do any calculation for those.
     */
    fn all_neighbors_finished_too_long(&self, x: u32, y: u32) -> bool {
        for a in -NEIGHBOURS..NEIGHBOURS {
            for b in -NEIGHBOURS..NEIGHBOURS {
                let xx = x + a;
                let yy = y + b;
                if self.check_domain(xx, yy) {
                    let el = self.domain_elements[xx][yy];
                    if el.isFinishedSuccessAny() || el.isFinishedTooShort() {
                        false
                    }
                }
            }
        }
        true
    }

    /**
     * All new elements are Active New
     * For wrapping, search only elements, which have some past well finished neighbors
     */
    fn is_on_mandelbrot_horizon(&self, x: u32, y: u32) -> bool {
        let mut red = false;
        let mut black = false;
        for a in -NEIGHBOURS..NEIGHBOURS {
            for b in -NEIGHBOURS..NEIGHBOURS {
                let xx = x + a;
                let yy = y + b;
                if self.check_domain(xx, yy) {
                    let el = &self.domain_elements[xx][yy];
                    if el.isFinishedSuccessPast() {
                        red = true;
                    }
                    if el.isHibernated() {
                        black = true;
                    }
                    if red & &black {
                        true
                    }
                }
            }
        }
        false
    }
}

fn init_domain_elements(domain_area: DomainArea) -> Vec<Vec<DomainElement>> {
    let mut vy: Vec<Vec<DomainElement>> = Vec::new();
    for x in 0..domain_area.widhtWIDTH_X {
        let mut vx: Vec<DomainElement> = Vec::new();
        for y in 0..domain_area.HEIGHT_Y {
            vx.push(domain_element::init(
                domain_area.screen_to_domain_re(x),
                domain_area.screen_to_domain_im(y),
            ));
        }
        vy.push(vx);
    }
    vy
}
