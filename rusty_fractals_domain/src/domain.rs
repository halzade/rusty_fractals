use crate::{domain_element, resolution_multiplier};
use domain_element::DomainElement;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::SquareAlter;

use rand::thread_rng;
use rand::seq::SliceRandom;
use rusty_fractals_common::area;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::NEIGHBOURS;
use crate::domain_element::{active_new, hibernated_deep_black};
use crate::pixel_states::DomainElementState;

pub struct Domain {
    pub width: u32,
    pub height: u32,
    pub domain_area: area::Area,
    pub domain_elements: Vec<Vec<DomainElement>>,
    pub resolution_multiplier: resolution_multiplier::ResolutionMultiplier,
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

    fn check_domain(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn shuffled_calculation_coordinates(&self) -> Vec<[u32; 2]> {
        let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();
        for x in 0..19 {
            for y in 0..19 {
                coordinates_xy.push([x, y]);
            }
        }
        coordinates_xy.shuffle(&mut thread_rng());
        coordinates_xy
    }


    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    fn wrap(&self, rm: ResolutionMultiplier, odd: bool) {
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

    // Colors for Mandelbrot image based on Mandelbrot element's state
    pub fn color_for_state(el: DomainElement) {
        match el.state() {
            /* most of the elements are going to be */
            DomainElementState::FinishedSuccessPast => FINISHED_SUCCESS_PAST,
            DomainElementState::HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
            DomainElementState::GoodPath => GOOD_PATH,
            DomainElementState::ActiveNew => ACTIVE_NEW,
            DomainElementState::FinishedSuccess => FINISHED_SUCCESS,
            DomainElementState::FinishedTooShort => FINISHED_TOO_SHORT,
            DomainElementState::FinishedTooLong => FINISHED_TOO_LONG
        }
    }

    pub fn mask_full_update(&self) {
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                domain_image.put_pixel(x, y, palette_utils::color_for_state(self.domain_elements[x][y]).getRGB());
            }
        }
    }

    // This is called after calculation finished, zoom was called and new area measures recalculated
    pub fn recalculate_pixels_positions_for_this_zoom(&mut self) {
        // Scan domain elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, all the next calculations will be skipped.

        let mut elements_to_move = Vec::new();

        let width = self.domain_area.width_x;
        let height = self.domain_area.height_y;

        for y in 0..height {
            for x in 0..width {
                let el = self.domain_elements[xx][yy];
                // There was already zoom in, the new area is smaller
                if self.domain_area.contains(el.origin_re, el.origin_im) {
                    // Element did not move out of the zoomed in area
                    elements_to_move.push(el);
                }
            }
        }


        // If there is a conflict, two or more points moved to same pixel, then use the active one if there is any.
        // Don't drop conflicts around, simply calculate new elements in the next calculation iteration. Because that would create really bad mess.

        for el in elements_to_move {
            // translate [px,py] to [re,im]
            (px, py) = self.domain_area.domain_point_to_result_pixel(el.origin_re, el.origin_im);

            let filled_already = self.domain_elements[px][py];
            if filled_already != null {
                /* conflict */
                if filled_already.has_worse_state_then(el) {
                    // Replace by element with better state
                    // Better to delete the other one, then to drop it to other empty pixel.
                    // That would cause problem with optimization, better calculate new and shiny pixel
                    self.domain_elements[px][py] = el;
                }
            } else {
                // Good, there is no conflict
                self.domain_elements[m.px][m.py] = el;
            }
        }

        // Repaint with only moved elements
        maskFullUpdate();
        Application.repaintMaskMandelbrotWindow();

        // Create new elements on positions where nothing was moved to
        for y in 0..height {
            for x in 0..width {
                let el = self.domain_elements[x][y];
                if el == null {
                    self.domain_area.screenToDomainCarry(m, x, y);
                    if allNeighborsFinishedTooLong(x, y) {
                        /* Calculation for some positions should be skipped as they are too far away form any long successful divergent position */
                        self.domain_elements[x][y] = hibernated_deep_black(m.re, m.im);
                    } else {
                        self.domain_elements[x][y] = active_new(m.re, m.im);
                    }
                } else {
                    /* If relevant, mark it as element from previous calculation iteration */
                    el.past();
                }
            }
        }

        elements_to_move.clear();
    }

    /**
     * Verify if any neighbor px,py finished well, long or at least too short.
     * This method identifies deep black convergent elements of Mandelbrot set interior.
     * Don't do any calculation for those.
     */
    fn all_neighbors_finished_too_long(&mut self, x: u32, y: u32) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..neigh {
            for b in -neigh..neigh {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if self.check_domain(xx, yy) {
                    let el = self.domain_elements[xx as usize][yy as usize];
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
        let neigh = NEIGHBOURS as i16;
        for a in -neigh..neigh {
            for b in -neigh..neigh {
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
                    if red && black {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn init_domain_elements(domain_area: Area) -> Vec<Vec<DomainElement>> {
    let mut vy: Vec<Vec<DomainElement>> = Vec::new();
    for x in 0..domain_area.width_x {
        let mut vx: Vec<DomainElement> = Vec::new();
        for y in 0..domain_area.height_y {
            vx.push(domain_element::init(
                domain_area.screen_to_domain_re(x),
                domain_area.screen_to_domain_im(y),
            ));
        }
        vy.push(vx);
    }
    vy
}
