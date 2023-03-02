use std::borrow::BorrowMut;
use std::thread;
use rusty_fractals_common::{area, data_image, fractal_stats};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{FractalConfig, Fractal, Update, FractalMandelbrot, MandelbrotConfig, UpdateMandelbrot};
use rusty_fractals_common::perfect_colour_distribution::{perfectly_colour_nebula_values};
use crate::{machine, machine_mandelbrot, window};
use crate::machine::MachineRefresh;

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
    let width = area_config.width_x;
    let height = area_config.height_y;
    let (app, machine_sender) = window::show(fractal.name(), data_image::image_init(width, height), width as i32, height as i32);

    let area: Area = area::init(&area_config);
    area::AREA.lock().unwrap().replace(area);

    let mut data_image = data_image::init_data_dynamic();
    let mut machine = machine_mandelbrot::init(mandelbrot_config, machine_sender);
    thread::spawn(move || {
        let mut lo = area::AREA.lock().unwrap();
        let area_o = lo.as_mut();
        let area_here = area_o.unwrap();

        for it in 1.. {
            println!("{}:", it);
            machine.calculate_mandelbrot(fractal, &data_image);
            // prepare next frame
            area_here.zoom_in();
            data_image.recalculate_pixels_positions_for_next_calculation(area_here, true);
            fractal_update.update(machine.conf_mut());
        };
    });
    app.run().unwrap();
}

pub fn calculate_nebula_zoom(
    fractal: &'static impl Fractal,
    fractal_update: &'static impl Update,
    fractal_config: FractalConfig,
    area_config: AreaConfig,
) {
    let width = area_config.width_x;
    let height = area_config.height_y;
    let (app, machine_sender) = window::show(fractal.name(), data_image::image_init(width, height), width as i32, height as i32);

    let area: Area = area::init(&area_config);
    area::AREA.lock().unwrap().replace(area);

    let mut data_image = data_image::init_data_dynamic();
    let mut machine = machine::init(fractal_config, machine_sender);
    thread::spawn(move || {
        let stats = &mut fractal_stats::init();
        for it in 1.. {
            println!("{}:", it);
            let mut lo = area::AREA.lock().unwrap();
            let area_o = lo.as_mut();
            let area_here = area_o.unwrap().borrow_mut();
            machine.calculate(fractal, &data_image, area_here.plank());
            data_image.translate_all_paths_to_point_grid();
            perfectly_colour_nebula_values(&data_image, &machine.palette);
            // prepare next frame
            area_here.zoom_in();
            data_image.recalculate_pixels_positions_for_next_calculation(&area_here, false);
            machine.refresh_final(&data_image);
            stats.update(&data_image, it);
            fractal_update.update(machine.conf_mut(), stats);
        };
    });
    app.run().unwrap();
}
