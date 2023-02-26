use std::thread;
use std::sync::{Arc, Mutex};
use rusty_fractals_common::{data_image, fractal_stats};
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, Update, FractalMandelbrot, MandelbrotConfig, UpdateMandelbrot};
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_nebula_values;
use crate::{machine, machine_mandelbrot, window};

// to calculate sequence of images for zoom video
pub struct Engine {}

pub fn init() -> Engine {
    Engine {}
}

impl Engine {}

pub fn calculate_mandelbrot_zoom(
    fractal: &'static impl FractalMandelbrot,
    fractal_update: &'static impl UpdateMandelbrot,
    mandelbrot_config: MandelbrotConfig,
    area_config: AreaConfig,
) {
    let mut machine = machine_mandelbrot::init(mandelbrot_config, &area_config);
    let mut data_image = data_image::init_data_video(machine.area(), None);
    let width = machine.area().width_x;
    let height = machine.area().height_y;
    let mut app_window = window::init(fractal.name(), width, height);
    let app = app_window.show(&data_image.image_init(), width, height);
    let mutex_window = Arc::new(Mutex::new(app_window));
    thread::spawn(move || {
        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal, &data_image, &mutex_window);
            data_image.translate_all_paths_to_point_grid(machine.area());
            perfectly_colour_nebula_values(&data_image, &machine.palette);
            data_image.recalculate_pixels_positions_for_next_calculation(machine.area());
            window::refresh_maybe(&data_image, &mutex_window, None, Some(machine.area()));
            // prepare next frame
            machine.area_mut().zoom_in();
            data_image.clear_screen_pixel_values();
            fractal_update.update(machine.conf_mut());
        };
    });
    app.run().unwrap();
}

pub fn calculate_nebula_zoom(
    fractal: &'static impl Fractal,
    fractal_update: &'static impl Update,
    width: usize,
    height: usize,
    fractal_config: FractalConfig,
    area_config: AreaConfig,
) {
    let mut machine = machine::init(fractal_config, &area_config);
    let mut data_image = data_image::init_data_video(machine.area(), None);
    let mut app_window = window::init(fractal.name(), width, height);
    let app = app_window.show(&data_image.image_init(), width, height);
    let mutex_window = Arc::new(Mutex::new(app_window));
    thread::spawn(move || {
        let stats = &mut fractal_stats::init();
        for it in 1.. {
            println!("{}:", it);
            machine.calculate(fractal, &data_image, &mutex_window);
            data_image.translate_all_paths_to_point_grid(machine.area());
            perfectly_colour_nebula_values(&data_image, &machine.palette);
            data_image.recalculate_pixels_positions_for_next_calculation(machine.area());
            window::refresh_maybe(&data_image, &mutex_window, None, Some(machine.area()));
            // prepare next frame
            stats.update(&data_image, it);
            fractal_update.update(machine.conf_mut(), stats);
            data_image.clear_screen_pixel_values();
            machine.area_mut().zoom_in();
        };
    });
    app.run().unwrap();
}
