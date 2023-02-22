struct Domain{
}

impl Domain {
    // pub fn set_finished_state(&self, x: usize, y: usize, state: DomainElementState) {
    //     self.domain_elements[x].get(y).expect("domain_elements problem").lock().unwrap().deref_mut().set_finished_state(state);
    // }

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
}
