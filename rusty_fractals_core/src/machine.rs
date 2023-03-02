use std::sync::{Arc, Mutex};
use std::thread;
use fltk::app;
use fltk::app::Sender;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rusty_fractals_common::{area, data_image, pixel_states};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal::{FractalConfig, Fractal, Conf};
use rusty_fractals_common::data_image::{DataImage, state_from_path_length};
use rusty_fractals_common::fractal_log::now;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::perfect_colour_distribution::perfectly_colour_nebula_values;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use crate::window;

// to calculate single image
pub struct Machine {
    pub conf: Conf,
    pub palette: Palette,
    resolution_multiplier: ResolutionMultiplier,
    sender: Sender<Vec<u8>>,
}

pub trait MachineRefresh {
    fn sender(&self) -> &Sender<Vec<u8>>;
    fn refresh_main(&self, refresh_lock: &Arc<Mutex<bool>>, data: &DataImage) {
        if std::mem::replace(&mut refresh_lock.lock().unwrap(), false) {
            *refresh_lock.lock().unwrap() = false;
            now("refresh_main()");
            let image_rgb = data.image_temp(false, None);
            self.sender().send(image_rgb);
            app::sleep(0.02);
            *refresh_lock.lock().unwrap() = true;
        }
    }
    fn refresh_wrap(&self, refresh_lock: &Arc<Mutex<bool>>, data: &DataImage, area: &Area) {
        if std::mem::replace(&mut refresh_lock.lock().unwrap(), false) {
            *refresh_lock.lock().unwrap() = false;
            now("refresh_wrap()");
            let image_rgb = data.image_temp(true, Some(area));
            self.sender().send(image_rgb);
            app::sleep(0.02);
            // allow to set another display path
            *data.show_path_update.lock().unwrap() = true;
            *refresh_lock.lock().unwrap() = true;
        }
    }
    fn refresh_final(&self, data: &DataImage) {
        now("refresh_final()");
        let image_rgb = data.image_result();
        self.sender().send(image_rgb);
    }
}

impl MachineRefresh for Machine {
    fn sender(&self) -> &Sender<Vec<u8>> {
        &self.sender
    }
}

pub fn init(fractal_config: FractalConfig, sender: Sender<Vec<u8>>) -> Machine {
    Machine {
        conf: Conf {
            max: fractal_config.iteration_max,
            min: fractal_config.iteration_min,
        },
        resolution_multiplier: fractal_config.resolution_multiplier,
        palette: fractal_config.palette,
        sender,
    }
}

pub fn nebula_calculation_for(
    fractal: &'static impl Fractal,
    fractal_config: FractalConfig,
    area_config: AreaConfig,
) {
    let width = area_config.width_x;
    let height = area_config.height_y;
    let (app, sender_machine) = window::show(fractal.name(), data_image::image_init(width, height), width as i32, height as i32);

    let area: Area = area::init(&area_config);
    let plank = area.plank();
    area::AREA.lock().unwrap().replace(area);

    let data = data_image::init_data_static();
    let machine = init(fractal_config, sender_machine);
    thread::spawn(move || {
        machine.calculate(fractal, &data, plank);
    });
    app.run().unwrap();
}

impl Machine {
    pub fn calculate(&self, fractal: &impl Fractal, data_image: &DataImage, plank: f64) {
        println!("calculate()");
        let coordinates_xy: Vec<[u32; 2]> = shuffled_calculation_coordinates();
        let refresh_lock = Arc::new(Mutex::new(true));

        let lo = area::AREA.lock().unwrap();
        let area_o = lo.as_ref();
        let area = area_o.unwrap().clone();

        coordinates_xy.par_iter().for_each(|xy| {
            // calculation
            self.chunk_calculation(&xy, fractal, &data_image, area);
            // window refresh
            self.refresh_main(&refresh_lock, &data_image);
        });
        data_image.recalculate_pixels_states();

        if self.resolution_multiplier != ResolutionMultiplier::Single {
            println!("calculate() with wrap");
            // previous calculation completed, calculate more elements
            coordinates_xy.par_iter().for_each(|xy| {
                // calculation
                self.chunk_calculation_with_wrap(&xy, fractal, data_image, plank, area);
                // window refresh
                self.refresh_wrap(&refresh_lock, &data_image, area);
            });
        }
        perfectly_colour_nebula_values(&data_image, &self.palette);
        self.refresh_final(data_image);
    }

    // in sequence executes as 20x20 parallel for each image part/chunk
    fn chunk_calculation(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        data: &DataImage,
        area: &Area,
    ) {
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, data.width, data.height);
        for x in x_from..x_to {
            for y in y_from..y_to {
                self.calculate_path_xy(x, y, fractal, data, area);
            }
        }
    }

    fn chunk_calculation_with_wrap(
        &self, xy: &[u32; 2],
        fractal: &impl Fractal,
        data: &DataImage,
        plank: f64,
        area: &Area,
    ) {
        if self.resolution_multiplier == ResolutionMultiplier::Single {
            panic!()
        }
        let (x_from, x_to, y_from, y_to) = chunk_boundaries(xy, data.width, data.height);
        for x in x_from..x_to {
            for y in y_from..y_to {
                if data.is_on_mandelbrot_horizon(x, y) {
                    let (origin_re, origin_im) = data.origin_at(x, y);
                    let wrap = data.wrap(origin_re, origin_im, self.resolution_multiplier.clone(), plank);
                    // within the same pixel
                    for [re, im] in wrap {
                        fractal.calculate_path(area, self.conf.min, self.conf.max, re, im, data, true);
                    }
                }
            }
        }
    }

    fn calculate_path_xy(
        &self, x: usize, y: usize,
        fractal: &impl Fractal,
        data: &DataImage,
        area: &Area,
    ) {
        let (state, origin_re, origin_im) = data.state_origin_at(x, y);
        if pixel_states::is_active_new(state) {
            let (iterator, path_length) = fractal.calculate_path(area, self.conf.min, self.conf.max, origin_re, origin_im, data, false);
            let state = state_from_path_length(iterator, path_length, self.conf.min, self.conf.max);
            data.set_pixel_state(x, y, state);
        }
    }

    pub fn conf_mut(&mut self) -> &mut Conf {
        &mut self.conf
    }
}

pub fn chunk_boundaries(xy: &[u32; 2], width: usize, height: usize) -> (usize, usize, usize, usize) {
    let chunk_size_x = (width / 20) as u32;
    let chunk_size_y = (height / 20) as u32;
    ((xy[0] * chunk_size_x) as usize, ((xy[0] + 1) * chunk_size_x) as usize,
     (xy[1] * chunk_size_y) as usize, ((xy[1] + 1) * chunk_size_y) as usize)
}

pub fn shuffled_calculation_coordinates() -> Vec<[u32; 2]> {
    let mut coordinates_xy: Vec<[u32; 2]> = Vec::new();
    for x in 0..20 {
        for y in 0..20 {
            coordinates_xy.push([x, y]);
        }
    }
    coordinates_xy.shuffle(&mut thread_rng());
    coordinates_xy
}
