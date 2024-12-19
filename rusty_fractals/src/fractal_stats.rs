use crate::constants::TAKE_MEASURES_AT_FRAME;
use crate::data_image::DataImage;

pub struct Stats {
    new_elements_too_long: u32,
    new_elements_too_short: u32,
    new_elements_long: u32,

    // All paths including previous calculations
    // The amount of newly added paths is not the same as the amount of red elementLong
    paths_total_amount: u32,

    paths_new_points_amount: u32,
    pixels_value_total: u32,
    pixels_value_best: u32,

    not_enough_pixels_total_value: bool,
    less_pixels_total_value: bool,
    too_many_pixels_total_value: bool,

    pub not_enough_pixels_best_value: bool,
    pub less_pixels_best_value: bool,
    pub too_many_paths_total: bool,
    not_enough_long_elements: bool,

    new_elements_long_measure: u32,
    new_elements_long_tolerance: u32,
    paths_total_amount_measure: u32,
    paths_total_amount_tolerance: u32,
    pixels_value_total_measure: u32,
    pixels_value_total_tolerance: u32,
    pixels_value_best_measure: u32,
    pixels_value_best_tolerance: u32,
    average_path_length_measure: u32,
}

impl Stats {
    fn remember_this(&mut self, data: &DataImage) {
        println!("new_elements_long  {}", self.new_elements_long);
        println!("pixels_value_total {}", self.pixels_value_total);
        println!("paths_total_amount {}", self.paths_total_amount);

        self.new_elements_long_measure = self.new_elements_long;
        self.pixels_value_total_measure = self.pixels_value_total;
        self.paths_total_amount_measure = self.paths_total_amount;
        self.average_path_length_measure =
            (self.pixels_value_total as f64 / self.paths_total_amount as f64) as u32;
        self.pixels_value_best_measure = data.best_four_chunks_value();

        self.new_elements_long_tolerance = (self.new_elements_long_measure as f64 * 0.5) as u32;
        self.pixels_value_total_tolerance = (self.pixels_value_total_measure as f64 * 0.5) as u32;
        self.paths_total_amount_tolerance = (self.paths_total_amount_measure as f64 * 0.5) as u32;
        self.pixels_value_best_tolerance = (self.pixels_value_best_measure as f64 * 0.5) as u32;

        // @formatter:off
        println!("elementsLong_measure        {} ", self.new_elements_long_measure);
        println!("pixels_value_total_measure  {} ", self.pixels_value_total_measure);
        println!("pixels_value_best_measure   {} ", self.pixels_value_best_measure);
        println!("paths_total_amount_measure  {} ", self.paths_total_amount_measure);
        println!("average_path_length_measure {} ", self.average_path_length_measure);
        // @formatter:on
    }

    pub fn update(&mut self, data: &DataImage, it: u32) {
        // Check if Stats should remember this iteration data for subsequent comparison
        if it == TAKE_MEASURES_AT_FRAME {
            self.remember_this(data);
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
            self.pixels_value_best = data.best_four_chunks_value();
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

            // @formatter:off
            println!("not_enough_pixels_total_value {}", self.not_enough_pixels_total_value);
            println!("less_pixels_total_value       {}", self.less_pixels_total_value);
            println!("less_pixels_best_value        {} ({} < {})", self.less_pixels_best_value, self.pixels_value_best, self.pixels_value_best_measure);
            println!("too_many_pixels_total_value   {}", self.too_many_pixels_total_value);
            println!("too_many_paths_total          {}", self.too_many_paths_total);
            println!("not_enough_long_elements      {}", self.not_enough_long_elements);
            // @formatter:on

            let average_path_length =
                self.pixels_value_total as f64 / self.paths_total_amount as f64;
            let new_elements_all =
                self.new_elements_long + self.new_elements_too_short + self.new_elements_too_long;
            let domain_elements_to_new_calculation_path_points =
                self.paths_new_points_amount as f64 / new_elements_all as f64;

            // @formatter:off
            println!("average_path_length                             {} ({})", average_path_length, self.average_path_length_measure);
            println!("domain_elements_to_new_calculation_path_points: {}", domain_elements_to_new_calculation_path_points);
            // @formatter:on
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
        println!("new_elements_too_long   {}", self.new_elements_too_long);
        println!("new_elements_too_short  {}", self.new_elements_too_short);
        println!("new_elements_long       {}", self.new_elements_long);
        println!("paths_total_amount      {}", self.paths_total_amount);
        println!("pixels_value_total      {}", self.pixels_value_total);
        println!("pixels_value_best       {}", self.pixels_value_best);
        println!("paths_new_points_amount {}", self.paths_new_points_amount);
    }
}

pub fn init() -> Stats {

    // TODO
    // data: Mutex::new(area_data),

    Stats {
        new_elements_too_long: 0,
        new_elements_too_short: 0,
        new_elements_long: 0,
        paths_total_amount: 0,
        paths_new_points_amount: 0,
        pixels_value_total: 0,
        pixels_value_best: 0,
        not_enough_pixels_total_value: false,
        less_pixels_total_value: false,
        too_many_pixels_total_value: false,
        not_enough_pixels_best_value: false,
        less_pixels_best_value: false,
        too_many_paths_total: false,
        not_enough_long_elements: false,
        new_elements_long_measure: 0,
        new_elements_long_tolerance: 0,
        paths_total_amount_measure: 0,
        paths_total_amount_tolerance: 0,
        pixels_value_total_measure: 0,
        pixels_value_total_tolerance: 0,
        pixels_value_best_measure: 0,
        pixels_value_best_tolerance: 0,
        average_path_length_measure: 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
