use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use image::{Rgb, RgbImage};
use crate::{domain_element, pixel_states};
use domain_element::DomainElement;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::{NEIGHBOURS};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use ResolutionMultiplier::{Square2, Square101, Square11, Square3, Square5, Square9, Square51};
use crate::pixel_states::{ACTIVE_NEW, DomainElementState, FINISHED, FINISHED_SUCCESS, FINISHED_SUCCESS_PAST, FINISHED_TOO_LONG, FINISHED_TOO_SHORT, GOOD_PATH, HIBERNATED_DEEP_BLACK};

pub struct Domain {
    pub width: usize,
    pub height: usize,
    domain_elements: Vec<Vec<Arc<Mutex<DomainElement>>>>,
}

impl Domain {
    pub fn get_el_triplet(&self, x: usize, y: usize) -> (DomainElementState, f64, f64) {
        let mutex_guard = self.domain_elements[x].get(y).expect("domain_elements problem").lock().unwrap();
        let el = mutex_guard.deref();
        (el.state, el.origin_re, el.origin_im)
    }

    pub fn get_el_state(&self, x: usize, y: usize) -> DomainElementState {
        self.get_el_triplet(x, y).0
    }

    pub fn set_finished_state(&self, x: usize, y: usize, state: DomainElementState) {
        self.domain_elements[x].get(y).expect("domain_elements problem").lock().unwrap().deref_mut().set_finished_state(state);
    }

    fn past(&self, x: usize, y: usize) {
        self.domain_elements[x].get(y).expect("domain_elements problem").lock().unwrap().deref_mut().past();
    }

    fn check_domain(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn shuffled_calculation_coordinates(&self) -> Vec<[u32; 2]> {
        let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                coordinates_xy.push([x, y]);
            }
        }
        coordinates_xy.shuffle(&mut thread_rng());
        coordinates_xy
    }

    // Don't do any wrapping the first time because Mandelbrot elements are not optimized.
    pub fn wrap(&self, origin_re: f64, origin_im: f64, rm: ResolutionMultiplier, area: &Area) -> Vec<[f64; 2]> {
        let mut ret = Vec::new();
        if rm == Square2 {
            let d = area.plank() / 3.0;
            ret.push([origin_re + d, origin_im + d]);
            ret.push([origin_re - d, origin_im - d]);
            ret.push([origin_re - d, origin_im + d]);
            ret.push([origin_re + d, origin_im - d]);
        } else {
            let multiplier: f64 = Domain::resolve_multiplier(rm);
            let d: f64 = area.plank() / multiplier;
            // multiplier was odd
            let half = ((multiplier - 1.0) / 2.0) as i32;
            // This fills the pixel with multiple points
            for x in -half..half {
                for y in -half..half {
                    if x != 0 || y != 0 {
                        ret.push([origin_re + (x as f64 * d), origin_im + (y as f64 * d)]);
                    }
                }
            }
        }
        ret
    }

    pub fn resolve_multiplier(rm: ResolutionMultiplier) -> f64 {
        match rm {
            ResolutionMultiplier::None => 1.0,
            Square3 => 3.0,
            Square5 => 5.0,
            Square9 => 9.0,
            Square11 => 11.0,
            Square51 => 51.0,
            Square101 => 101.0,
            Square2 => 1.0
        }
    }

    // Colors for Mandelbrot image based on Mandelbrot element's state
    pub fn color_for_state(state: DomainElementState) -> Rgb<u8> {
        match state {
            // most of the elements are going to be FinishedSuccessPast
            DomainElementState::FinishedSuccessPast => FINISHED_SUCCESS_PAST,
            DomainElementState::HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
            DomainElementState::GoodPath => GOOD_PATH,
            DomainElementState::ActiveNew => ACTIVE_NEW,
            DomainElementState::FinishedSuccess => FINISHED_SUCCESS,
            DomainElementState::FinishedTooShort => FINISHED_TOO_SHORT,
            DomainElementState::FinishedTooLong => FINISHED_TOO_LONG,
            DomainElementState::Finished => FINISHED
        }
    }

    pub fn domain_element_states_to_image(&self) -> RgbImage {
        let mut domain_image = RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                domain_image.put_pixel(x as u32, y as u32, Domain::color_for_state(self.get_el_state(x, y)));
            }
        }
        domain_image
    }

    pub fn recalculate_pixels_states(&self, area: &Area) {
        let width = area.width_x;
        let height = area.height_y;
        for y in 0..height {
            for x in 0..width {
                self.past(x, y);
            }
        }
    }


    // This is called after calculation finished, zoom was called and new area measures recalculated
    /*
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


        // If there is a conflict, two or more points moved to same pixel,don't drop conflicts around
        // Simply calculate new elements in the next calculation iteration. Because that would create really bad mess.

        for el in elements_to_move {
            // translate [px,py] to [re,im]
            (px, py) = self.domain_area.domain_point_to_result_pixel(el.origin_re, el.origin_im);

            let filled_already = self.domain_elements[px][py];
            if filled_already != null {
                // conflict
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
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        self.domain_elements[x][y] = hibernated_deep_black(m.re, m.im);
                    } else {
                        self.domain_elements[x][y] = active_new(m.re, m.im);
                    }
                } else {
                    // If relevant, mark it as element from previous calculation iteration
                    el.past();
                }
            }
        }

        elements_to_move.clear();
    }
    */

    // Verify if any neighbor px,py finished well, long or at least too short.
    // This method identifies deep black convergent elements of Mandelbrot set interior.
    // Don't do any calculation for those.
    /*
    fn all_neighbors_finished_too_long(&mut self, x: u32, y: u32) -> bool {
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..neigh {
            for b in -neigh..neigh {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if self.check_domain(xx, yy) {
                    let el = self.domain_elements[xx as usize][yy as usize];
                    if el.is_finished_success_any() || el.is_finished_too_short() {
                        return false;
                    }
                }
            }
        }
        true
    }
    */

    // all new elements are Active New
    // for wrapping, search only elements, which have some past well finished neighbors
    // previous calculation must be completed
    pub fn is_on_mandelbrot_horizon(&self, x: usize, y: usize) -> bool {
        let mut red = false;
        let mut black = false;
        let neigh = NEIGHBOURS as i32;
        for a in -neigh..neigh {
            for b in -neigh..neigh {
                let xx = x as i32 + a;
                let yy = y as i32 + b;
                if self.check_domain(xx, yy) {
                    let state = self.get_el_state(xx as usize, yy as usize);
                    if pixel_states::is_finished_success_past(state) {
                        red = true;
                    }
                    if pixel_states::is_hibernated(state) {
                        black = true;
                    }
                    // some of the neighbors were;
                    if red && black {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn state_from_path_length(iterator: u32, max: u32, min: u32) -> DomainElementState {
        if iterator == max {
            return DomainElementState::FinishedTooLong;
        }
        if iterator < min {
            return DomainElementState::FinishedTooShort;
        }
        DomainElementState::FinishedSuccess
    }
}

pub fn init_domain_elements(domain_area: &Area) -> Vec<Vec<Arc<Mutex<DomainElement>>>> {
    let mut vy: Vec<Vec<Arc<Mutex<DomainElement>>>> = Vec::new();
    for x in 0..domain_area.width_x {
        let mut vx: Vec<Arc<Mutex<DomainElement>>> = Vec::new();
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

pub fn init(domain_area: &Area) -> Domain {
    Domain {
        width: domain_area.width_x,
        height: domain_area.height_y,
        domain_elements: init_domain_elements(&domain_area),
    }
}