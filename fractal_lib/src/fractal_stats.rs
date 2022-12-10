use log::{info, trace};

// Take snapshot of data for comparison at well colored frame.
const TAKE_MEASURES_AT_FRAME: u32 = 20;

pub struct Stats {
    new_elements_too_long: i32,
    new_elements_too_short: i32,
    new_elements_long: i32,

    // All paths including previous calculations
    // The amount of newly added paths is not the same as the amount of red elementLong
    paths_total_amount: i32,

    paths_new_points_amount: i32,
    pixels_value_total: i32,
    pixels_value_best: i32,

    not_enough_pixels_total_value: bool,
    less_pixels_total_value: bool,
    too_many_pixels_total_value: bool,
    pub not_enough_pixels_best_value: bool,
    pub less_pixels_best_value: bool,
    pub too_many_paths_total: bool,
    not_enough_long_elements: bool,

    new_elements_long_measure: i32,
    new_elements_long_tolerance: i32,
    paths_total_amount_measure: i32,
    paths_total_amount_tolerance: i32,
    pixels_value_total_measure: i32,
    pixels_value_total_tolerance: i32,
    pixels_value_best_measure: i32,
    pixels_value_best_tolerance: i32,
    average_path_length_measure: i32,
}

impl Stats {
    fn remember_this(&mut self) {
        debug!("new_elements_long  {}", self.new_elements_long);
        debug!("pixels_value_total {}", self.pixels_value_total);
        debug!("paths_total_amount {}", self.paths_total_amount);

        self.new_elements_long_measure = self.new_elements_long;
        self.pixels_value_total_measure = self.pixels_value_total;
        self.paths_total_amount_measure = self.paths_total_amount;
        self.average_path_length_measure =
            (self.pixels_value_total as f64 / self.paths_total_amount as f64) as i32;
        // TODO self.pixels_value_best_measure = PixelsFinebrot.bestFourChunksValue();

        self.new_elements_long_tolerance = (self.new_elements_long_measure as f64 * 0.5) as i32;
        self.pixels_value_total_tolerance = (self.pixels_value_total_measure as f64 * 0.5) as i32;
        self.paths_total_amount_tolerance = (self.paths_total_amount_measure as f64 * 0.5) as i32;
        self.pixels_value_best_tolerance = (self.pixels_value_best_measure as f64 * 0.5) as i32;

        info!(
            "elementsLong_measure        {} ",
            self.new_elements_long_measure
        );
        info!(
            "pixels_value_total_measure  {} ",
            self.pixels_value_total_measure
        );
        info!(
            "pixels_value_best_measure   {} ",
            self.pixels_value_best_measure
        );
        info!(
            "paths_total_amount_measure  {} ",
            self.paths_total_amount_measure
        );
        info!(
            "average_path_length_measure {} ",
            self.average_path_length_measure
        );
    }

    pub(crate) fn update(&mut self, it: u32) {
        // Check if Stats should remember this iteration data for subsequent comparison
        if it == TAKE_MEASURES_AT_FRAME {
            self.remember_this();
        }

        /* Subsequent comparison */
        if it > TAKE_MEASURES_AT_FRAME {
            // Total value
            self.not_enough_pixels_total_value = false;
            if self.pixels_value_total < self.pixels_value_total_measure {
                self.not_enough_pixels_total_value = self.pixels_value_total_measure
                    - self.pixels_value_total
                    > self.pixels_value_total_tolerance;
            }
            self.too_many_pixels_total_value = false;
            if self.pixels_value_total > self.pixels_value_total_measure {
                self.too_many_pixels_total_value = self.pixels_value_total
                    - self.pixels_value_total_measure
                    > self.pixels_value_total_tolerance;
            }
            self.less_pixels_total_value =
                self.pixels_value_total < self.pixels_value_total_measure;

            // Best domain chunks, chunks with most image points
            self.not_enough_pixels_best_value = false;
            // TODO self.pixels_value_best = PixelsFinebrot.bestFourChunksValue();
            if self.pixels_value_best < self.pixels_value_best_measure {
                self.not_enough_pixels_best_value = self.pixels_value_best_measure
                    - self.pixels_value_best
                    > self.pixels_value_best_tolerance;
            }
            self.less_pixels_best_value = self.pixels_value_best < self.pixels_value_best_measure;

            // Paths
            self.too_many_paths_total = false;
            if self.paths_total_amount > self.paths_total_amount_measure {
                self.too_many_paths_total = self.paths_total_amount
                    - self.paths_total_amount_measure
                    > self.paths_total_amount_tolerance;
            }

            // Mandelbrot long successful elements
            self.not_enough_long_elements = false;
            if self.new_elements_long < self.new_elements_long_measure {
                self.not_enough_long_elements = self.new_elements_long_measure
                    - self.new_elements_long
                    > self.new_elements_long_tolerance;
            }

            debug!(
                "not_enough_pixels_total_value {}",
                self.not_enough_pixels_total_value
            );
            debug!(
                "less_pixels_total_value       {}",
                self.less_pixels_total_value
            );
            debug!(
                "less_pixels_best_value        {} ({} < {})",
                self.less_pixels_best_value, self.pixels_value_best, self.pixels_value_best_measure
            );
            debug!(
                "too_many_pixels_total_value   {}",
                self.too_many_pixels_total_value
            );
            debug!(
                "too_many_paths_total          {}",
                self.too_many_paths_total
            );
            debug!(
                "not_enough_long_elements      {}",
                self.not_enough_long_elements
            );

            let average_path_length =
                self.pixels_value_total as f64 / self.paths_total_amount as f64;
            let new_elements_all =
                self.new_elements_long + self.new_elements_too_short + self.new_elements_too_long;
            let domain_elements_to_new_calculation_path_points =
                self.paths_new_points_amount as f64 / new_elements_all as f64;

            info!(
                "average_path_length                             {} ({})",
                average_path_length, self.average_path_length_measure
            );
            info!(
                "domain_elements_to_new_calculation_path_points: {}",
                domain_elements_to_new_calculation_path_points
            );
        }
    }

    pub fn clean(&mut self) {
        self.new_elements_too_long = 0;
        self.new_elements_too_short = 0;
        self.new_elements_long = 0;
        self.paths_total_amount = 0;
        self.pixels_value_total = 0;
        self.pixels_value_best = 0;
        self.paths_new_points_amount = 0;
    }

    pub fn print(&self) {
        trace!("new_elements_too_long   {}", self.new_elements_too_long);
        trace!("new_elements_too_short  {}", self.new_elements_too_short);
        trace!("new_elements_long       {}", self.new_elements_long);
        trace!("paths_total_amount      {}", self.paths_total_amount);
        trace!("pixels_value_total      {}", self.pixels_value_total);
        trace!("pixels_value_best       {}", self.pixels_value_best);
        trace!("paths_new_points_amount {}", self.paths_new_points_amount);
    }
}
