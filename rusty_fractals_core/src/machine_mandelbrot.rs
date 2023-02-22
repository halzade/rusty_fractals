use std::sync::{Arc, Mutex};
use std::thread;
use rayon::prelude::*;
use rusty_fractals_common::{area, data_image};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{CalculationConfig, FractalMandelbrot};
use rusty_fractals_common::data_image::{DataImage, state_from_path_length};
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::palettes::{ResultConfigMandelbrot};
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_mandelbrot_values;
use crate::{machine, window};
use crate::window::AppWindow;

// to calculate single image
pub struct MachineMandelbrot {
    area: Area,
    iteration_min: u32,
    iteration_max: u32,
    palette: Palette,
    palette_zero: Palette,
}

pub fn init(calculation_config: &CalculationConfig, result_config: ResultConfigMandelbrot, area_config: &AreaConfig) -> MachineMandelbrot {
    let area = area::init(&area_config);
    MachineMandelbrot {
        area,
        iteration_min: calculation_config.iteration_min,
        iteration_max: calculation_config.iteration_max,
        palette: result_config.palette,
        palette_zero: result_config.palette_zero,
    }
}

pub fn mandelbrot_calculation_for(
    fractal: &'static impl FractalMandelbrot,
    width: usize,
    height: usize,
    calculation_config: CalculationConfig,
    result_config: ResultConfigMandelbrot,
    area_config: AreaConfig,
) {
    let machine = init(&calculation_config, result_config, &area_config);
    let data_image = data_image::init_data_image(machine.area());

    // let name2 = *name.clone();
    let mut app_window = window::init(fractal.name(), width, height);
    let app = app_window.show(&data_image.image_init().as_raw(), width, height);
    let mutex_window = Arc::new(Mutex::new(app_window));

    thread::spawn(move || {
        machine.calculate_mandelbrot(fractal, &data_image, mutex_window);
    });
    app.run().unwrap();
}

impl MachineMandelbrot {
    pub fn calculate_mandelbrot(&self, fractal: &impl FractalMandelbrot, data_image: &DataImage, app_window: Arc<Mutex<AppWindow>>) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = machine::shuffled_calculation_coordinates();
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(&xy, fractal, &data_image);
            // window refresh
            window::refresh_maybe(data_image, &app_window);
        });
        perfectly_colour_mandelbrot_values(&data_image, &self.palette, &self.palette_zero);
        window::refresh_final(data_image, &app_window);
    }

    fn chunk_calculation_mandelbrot(
        &self, xy: &[u32; 2],
        fractal: &impl FractalMandelbrot,
        data_image: &DataImage,
    ) {
        let (x_from, x_to, y_from, y_to) = machine::chunk_boundaries(xy, self.area.width_x, self.area.height_y);
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (origin_re, origin_im) = data_image.origin_at(x, y);
                // calculation
                let (iterator, quad) = fractal.calculate_mandelbrot_path(self.iteration_max, origin_re, origin_im);
                let state = state_from_path_length(iterator, iterator, self.iteration_min, self.iteration_max);
                data_image.set_pixel_mandelbrot(x, y, iterator, quad, state, self.iteration_max);
            }
        }
    }

    pub fn area(&self) -> &Area {
        &self.area
    }
}
