use std::sync::{Arc, Mutex};
use std::thread;
use fltk::app::Sender;
use rayon::prelude::*;
use rusty_fractals_common::{area, data_image, pixel_states};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{MandelbrotConfig, FractalMandelbrot, Conf, FractalName, Recalculate};
use rusty_fractals_common::data_image::{DataImage, state_from_path_length};
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_mandelbrot_values;
use crate::{machine, window};
use crate::machine::MachineRefresh;

// to calculate single image
pub struct MachineMandelbrot {
    conf: Conf,
    pub palette: Palette,
    pub palette_zero: Palette,
    sender: Sender<Vec<u8>>,
}

pub fn init(mandelbrot_config: MandelbrotConfig, sender: Sender<Vec<u8>>) -> MachineMandelbrot {
    MachineMandelbrot {
        conf: Conf {
            max: mandelbrot_config.iteration_max,
            min: 0,
        },
        palette: mandelbrot_config.palette,
        palette_zero: mandelbrot_config.palette_zero,
        sender,
    }
}

impl MachineRefresh for MachineMandelbrot {
    fn sender(&self) -> &Sender<Vec<u8>> {
        &self.sender
    }
}

// TODO
const MACHINE: Option<&'static MachineMandelbrot> = None;
const DATA: Option<&'static DataImage> = None;

pub fn mandelbrot_calculation_for<M: FractalMandelbrot + FractalName + Recalculate>(
    fractal: &'static M,
    mandelbrot_config: MandelbrotConfig,
    area_config: AreaConfig,
) {
    let width = area_config.width_x;
    let height = area_config.height_y;
    let (app, sender_machine) = window::show(fractal, data_image::image_init(width, height), width as i32, height as i32);

    let area: Area = area::init(&area_config);
    area::AREA.lock().unwrap().replace(area);

    let data = data_image::init_data_static();
    let machine = init(mandelbrot_config, sender_machine);

    // TODO
    // MACHINE.replace(&machine);
    // DATA.replace(&data);

    thread::spawn(move || {
        machine.calculate_mandelbrot(fractal, &data);
    });
    app.run().unwrap();
}

impl MachineMandelbrot {
    pub fn calculate_mandelbrot<M: FractalMandelbrot>(
        &self,
        fractal: &'static M,
        data_image: &DataImage,
    ) {
        println!("calculate_mandelbrot()");
        let coordinates_xy: Vec<[u32; 2]> = machine::shuffled_calculation_coordinates();
        let refresh_lock = Arc::new(Mutex::new(true));
        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation_mandelbrot(&xy, fractal, &data_image);
            // window refresh
            self.refresh_main(&refresh_lock, &data_image);
        });
        data_image.recalculate_pixels_states();
        perfectly_colour_mandelbrot_values(&data_image, &self.palette, &self.palette_zero);
        self.refresh_final(data_image);
    }

    fn chunk_calculation_mandelbrot<M: FractalMandelbrot>(
        &self, xy: &[u32; 2],
        fractal: &M,
        data_image: &DataImage,
    ) {
        let (x_from, x_to, y_from, y_to) = machine::chunk_boundaries(xy, data_image.width, data_image.height);
        for x in x_from..x_to {
            for y in y_from..y_to {
                let (state, origin_re, origin_im) = data_image.state_origin_at(x, y);
                // TODO, calculate only ActiveNew elements, copy quad and quid
                if !pixel_states::is_finished_any(state) {
                    // calculation
                    let (iterator, quad) = fractal.calculate_mandelbrot_path(self.conf.max, origin_re, origin_im);
                    let state = state_from_path_length(iterator, iterator, 0, self.conf.max);
                    data_image.set_pixel_mandelbrot(x, y, iterator, quad, state, self.conf.max);
                }
            }
        }
    }

    pub fn conf_mut(&mut self) -> &mut Conf {
        &mut self.conf
    }
}

pub fn recalculate<M: FractalMandelbrot>(fractal: &'static M) {
    println!("recalculate()");
    thread::spawn(move || {
        MACHINE.unwrap().calculate_mandelbrot(fractal, DATA.unwrap());
    });
}
