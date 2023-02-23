use std::thread;
use std::sync::{Arc, Mutex};
use rusty_fractals_common::data_image;
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::palettes::ResultConfig;
use crate::{machine, window};

// to calculate sequence of images for zoom video
pub struct Engine {
    pub calculation_config: CalculationConfig,
    pub app_config: AppConfig,
}

impl Engine {
    pub fn calculate_nebula_zoom(
        fractal: &'static impl Fractal,
        width: usize,
        height: usize,
        calculation_config: CalculationConfig,
        result_config: ResultConfig,
        area_config: AreaConfig,
    ) {
        let machine = machine::init(&calculation_config, result_config, &area_config);
        let mut data_image = data_image::init_data_image(machine.area());
        let mut app_window = window::init(fractal.name(), width, height);
        let app = app_window.show(&data_image.image_init(), width, height);
        let mutex_window = Arc::new(Mutex::new(app_window));
        thread::spawn(move || {
            for it in 1.. {
                println!("{}:", it);
                machine.calculate(fractal, &data_image, &mutex_window);
                data_image.recalculate_pixels_positions_for_this_zoom(machine.area());

                /*
                translate_paths_to_pixel_grid();
                perfectly_colour_result_values();
                repaint_mandelbrot_window();
                fractal.update();
                image_pixels.clear()
                zoom_in();
                */
            };
        });
        app.run().unwrap();
    }
}
