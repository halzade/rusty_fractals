use crate::domain_area::DomainArea;
use crate::{domain_area, domain_element};
use domain_element::DomainElement;

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
    fn make_chunk(&self, x_from: usize, x_to: usize, y_from: usize, y_to: usize) -> Vec<&DomainElement> {
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


    fn all_chunks(&self) {
        let mut wrapped: u32 = 0;
        let mut not_wrapped: u32 = 0;

        let chunk_size_x = self.width / 20;
        let chunk_size_y = self.height / 20;
        /* All the pixel (domain) will be split to multiple chunks */
        for x in 0..19 {
            for y in 0..19 {
                let chunk_of_elements = self.make_chunk(
                    (x * chunk_size_x) as usize, ((x + 1) * chunk_size_x) as usize,
                    (y * chunk_size_y) as usize, ((y + 1) * chunk_size_y) as usize,
                );
            }
        }


        /* Switch wrapping the next time */
        // TODO not here: firstDomainExecution = false;
        // TODO not here: odd = !odd;
    }


    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    fn wrap() {
        if (RESOLUTION_MULTIPLIER == square_alter) {
            final double
            d = AreaMandelbrot.plank() / 3;
            if odd {
                domainFull.add(activeNew(elementZero.originRe + d, elementZero.originIm + d));
                domainFull.add(activeNew(elementZero.originRe - d, elementZero.originIm - d));
            } else {
                domainFull.add(activeNew(elementZero.originRe - d, elementZero.originIm + d));
                domainFull.add(activeNew(elementZero.originRe + d, elementZero.originIm - d));
            }
        } else {
            let multiplier;
            switch(RESOLUTION_MULTIPLIER)
            {
                case
                square_3 -> multiplier = 3;
                case
                square_5 -> multiplier = 5;
                case
                square_11 -> multiplier = 11;
                case
                square_51 -> multiplier = 51;
                case
                square_101 -> multiplier = 101;
                default -> throw
                new
                RuntimeException("unknown RESOLUTION_MULTIPLIER");
            }

            final double
            pn = AreaMandelbrot.plank() / multiplier;
            let half = (multiplier - 1) / 2;
            /* This fills the pixel with multiple points */
            for x in -half..half {
                for y in -half..half {
                    if x != 0 || y != 0 {
                        domainFull.add(activeNew(elementZero.originRe + (x * pn), elementZero.originIm + (y * pn)));
                    }
                    /* else do nothing, there already is element0 for the center of this pixel */
                }
            }
        }
    }

    fn mask_full_update() {
        for y in 0..HEIGHT_Y {
            for x in 0..WIDTH_X {
                MaskMandelbrotImage.setRGB(x, y, colorForState(elementsStaticMandelbrot[x][y]).getRGB());
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
                let el = elementsStaticMandelbrot[xx][yy];
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
                filledAlready = elementsStaticMandelbrot[m.px][m.py];
                if filledAlready != null {
                    /* conflict */
                    if filledAlready.hasWorseStateThen(el) {
                        /*
                         * Replace by element with better state
                         * Better to delete the other one, then to drop it to other empty px.
                         * That would cause problem with optimization, better calculate new and shiny px.
                         */
                        elementsStaticMandelbrot[m.px][m.py] = el;
                    }
                } else {
                    /* Good, there is no conflict */
                    elementsStaticMandelbrot[m.px][m.py] = el;
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
                el = elementsStaticMandelbrot[x][y];
                if (el == null) {
                    AreaMandelbrot.screenToDomainCarry(m, x, y);
                    if (allNeighborsFinishedTooLong(x, y)) {
                        /* Calculation for some positions should be skipped as they are too far away form any long successful divergent position */
                        elementsStaticMandelbrot[x][y] = hibernatedDeepBlack(m.re, m.im);
                    } else {
                        elementsStaticMandelbrot[x][y] = activeNew(m.re, m.im);
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
    fn all_neighbors_finished_too_long(x: u32, y: u32) -> bool {
        MaskMandelbrotElement
        el;
        for a in -neighbours..neighbours {
            for b in -neighbours..neighbours {
                let xx = x + a;
                let yy = y + b;
                if check_domain(xx, yy) {
                    el = elementsStaticMandelbrot[xx][yy];
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
    fn is_on_mandelbrot_horizon(x: u32, y: u32) -> bool {
        let mut red = false;
        let mut black = false;
        for a in -NEIGHBOURS..neighbours {
            for b in -neighbours..neighbours {
                let xx = x + a;
                let yy = y + b;
                if check_domain(xx, yy) {
                    el = elementsStaticMandelbrot[xx][yy];
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

fn init_domain_elements(&self, domain_area: DomainArea) -> Vec<Vec<DomainElement>> {
    let mut vy: Vec<Vec<DomainElement>> = Vec::new();
    for x in 0..domain_area.widhtWIDTH_X {
        let mut vx: Vec<DomainElement> = Vec::new();
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
