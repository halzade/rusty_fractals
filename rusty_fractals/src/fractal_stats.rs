use crate::constants::TAKE_MEASURES_AT_FRAME;
use crate::data_image::DataImage;
use std::sync::Mutex;

pub struct Stats {
    data: Mutex<StatsData>,
}

impl Stats {
    pub fn paths_new_points_amount_add(&self, path_length: usize) {
        let data = &mut self.data.lock().unwrap();

        data.paths_new_points_amount += path_length as u32;
    }
}

pub struct StatsData {
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
    #[rustfmt::skip]
    fn remember_this(&self, data_image: &DataImage) {
        let data = &mut self.data.lock().unwrap();

        println!("new_elements_long  {}", data.new_elements_long);
        println!("pixels_value_total {}", data.pixels_value_total);
        println!("paths_total_amount {}", data.paths_total_amount);

        data.new_elements_long_measure = data.new_elements_long;
        data.pixels_value_total_measure = data.pixels_value_total;
        data.paths_total_amount_measure = data.paths_total_amount;
        data.average_path_length_measure =
            (data.pixels_value_total as f64 / data.paths_total_amount as f64) as u32;

        // from data image
        data.pixels_value_best_measure = data_image.best_four_chunks_value();

        data.new_elements_long_tolerance = (data.new_elements_long_measure as f64 * 0.5) as u32;
        data.pixels_value_total_tolerance = (data.pixels_value_total_measure as f64 * 0.5) as u32;
        data.paths_total_amount_tolerance = (data.paths_total_amount_measure as f64 * 0.5) as u32;
        data.pixels_value_best_tolerance = (data.pixels_value_best_measure as f64 * 0.5) as u32;

        println!("elementsLong_measure        {} ", data.new_elements_long_measure);
        println!("pixels_value_total_measure  {} ", data.pixels_value_total_measure);
        println!("pixels_value_best_measure   {} ", data.pixels_value_best_measure);
        println!("paths_total_amount_measure  {} ", data.paths_total_amount_measure);
        println!("average_path_length_measure {} ", data.average_path_length_measure);
    }

    #[rustfmt::skip]
    pub fn update(&self, data_image: &DataImage, it: u32) {
        // Check if Stats should remember this iteration data for subsequent comparison
        if it == TAKE_MEASURES_AT_FRAME {
            self.remember_this(data_image);
        }

        /* Subsequent comparison */
        if it > TAKE_MEASURES_AT_FRAME {
            let data = &mut self.data.lock().unwrap();

            // Total value
            data.not_enough_pixels_total_value = false;
            if data.pixels_value_total < data.pixels_value_total_measure {
                data.not_enough_pixels_total_value = data.pixels_value_total_measure
                    - data.pixels_value_total
                    > data.pixels_value_total_tolerance;
            }
            data.too_many_pixels_total_value = false;
            if data.pixels_value_total > data.pixels_value_total_measure {
                data.too_many_pixels_total_value = data.pixels_value_total
                    - data.pixels_value_total_measure
                    > data.pixels_value_total_tolerance;
            }
            data.less_pixels_total_value =
                data.pixels_value_total < data.pixels_value_total_measure;

            // Best domain chunks, chunks with most image points
            data.not_enough_pixels_best_value = false;
            data.pixels_value_best = data_image.best_four_chunks_value();
            if data.pixels_value_best < data.pixels_value_best_measure {
                data.not_enough_pixels_best_value = data.pixels_value_best_measure
                    - data.pixels_value_best
                    > data.pixels_value_best_tolerance;
            }
            data.less_pixels_best_value = data.pixels_value_best < data.pixels_value_best_measure;

            // Paths
            data.too_many_paths_total = false;
            if data.paths_total_amount > data.paths_total_amount_measure {
                data.too_many_paths_total = data.paths_total_amount
                    - data.paths_total_amount_measure
                    > data.paths_total_amount_tolerance;
            }

            // Mandelbrot long successful elements
            data.not_enough_long_elements = false;
            if data.new_elements_long < data.new_elements_long_measure {
                data.not_enough_long_elements = data.new_elements_long_measure
                    - data.new_elements_long
                    > data.new_elements_long_tolerance;
            }

            println!("not_enough_pixels_total_value {}", data.not_enough_pixels_total_value);
            println!("less_pixels_total_value       {}", data.less_pixels_total_value);
            println!("less_pixels_best_value        {} ({} < {})", data.less_pixels_best_value, data.pixels_value_best, data.pixels_value_best_measure);
            println!("too_many_pixels_total_value   {}", data.too_many_pixels_total_value);
            println!("too_many_paths_total          {}", data.too_many_paths_total);
            println!("not_enough_long_elements      {}", data.not_enough_long_elements);

            let average_path_length =
                data.pixels_value_total as f64 / data.paths_total_amount as f64;
            let new_elements_all =
                data.new_elements_long + data.new_elements_too_short + data.new_elements_too_long;
            let domain_elements_to_new_calculation_path_points =
                data.paths_new_points_amount as f64 / new_elements_all as f64;

            println!("average_path_length                             {} ({})", average_path_length, data.average_path_length_measure);
            println!("domain_elements_to_new_calculation_path_points: {}", domain_elements_to_new_calculation_path_points);
        }
    }

    pub fn clean(&mut self) {
        let data = &mut self.data.lock().unwrap();

        data.new_elements_too_long = 0;
        data.new_elements_too_short = 0;
        data.new_elements_long = 0;
        data.paths_total_amount = 0;
        data.pixels_value_total = 0;
        data.pixels_value_best = 0;
        data.paths_new_points_amount = 0;
    }

    pub fn print(&self) {
        let data = &mut self.data.lock().unwrap();

        println!("new_elements_too_long   {}", data.new_elements_too_long);
        println!("new_elements_too_short  {}", data.new_elements_too_short);
        println!("new_elements_long       {}", data.new_elements_long);
        println!("paths_total_amount      {}", data.paths_total_amount);
        println!("pixels_value_total      {}", data.pixels_value_total);
        println!("pixels_value_best       {}", data.pixels_value_best);
        println!("paths_new_points_amount {}", data.paths_new_points_amount);
    }
}

pub fn init() -> Stats {
    Stats {
        data: Mutex::new(StatsData {
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
        }),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
