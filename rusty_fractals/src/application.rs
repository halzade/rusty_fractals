use crate::area::Area;
use crate::data_image::{colour_for_state, DataImage};
use crate::fractal::CalculationType::StaticImage;
use crate::fractal::FractalType::MandelbrotType;
use crate::fractal::{FractalConfig, FractalMath};
use crate::machine;
use crate::machine::Machine;
use crate::pixel_states::is_finished_any;
use fltk::app::{event_button, event_coords, event_key, App};
use fltk::enums::{Color, Event, Key};
use fltk::{app, draw, prelude::*, window::Window};
use image::{Pixel, Rgb};
use std::sync::{Arc, Mutex};

pub struct Application<'lt, F: FractalMath> {
    machine: Arc<Mutex<Machine<'lt, F>>>,
    max_value: Arc<Mutex<u32>>, // Wrap in Arc
}

pub fn init<F: FractalMath>(config: FractalConfig, fractal: F) -> Application<'static, F> {
    Application {
        machine: Arc::new(Mutex::new(machine::init(&config, fractal))),
        max_value: Arc::new(Mutex::new(0)), // Wrap in Arc
    }
}

impl<F: FractalMath + 'static> Application<'static, F> {
    pub fn execute(self) {
        println!("application.execute()");

        println!("calculation");
        let is_mandelbrot = *&self.machine.lock().unwrap().fractal_type == MandelbrotType;
        let is_image = *&self.machine.lock().unwrap().calc_type == StaticImage;

        if is_mandelbrot {
            if is_image {
                // Fine fractal image
                &self.machine.lock().unwrap().calculate_mandelbrot();
            } else {
                // Fine fractal video
                &self.machine.lock().unwrap().calculate_mandelbrot_zoom();
            }
        } else {
            if is_image {
                // Hard fractal image
                println!("a");
                &self.machine.lock().unwrap().calculate_nebula();
                println!("b");
            } else {
                // Hard fractal video
                &self.machine.lock().unwrap().calculate_nebula_zoom();
            }
        }

        println!("show()");
        &self.show();

        println!("run().unwrap()");
        App::default().run().unwrap();

        println!("execute() end.");
    }

    pub fn show(&self) {
        println!("show()");

        let width = self.machine.lock().unwrap().width_x as i32;
        let height = self.machine.lock().unwrap().height_y as i32;
        let name = self.machine.lock().unwrap().name;

        let mut window = Window::default()
            .with_label(name)
            .with_size(width, height)
            .center_screen();

        // initialize window color, filled rectangle
        // draw::set_draw_color(Color::from_rgb(40, 180, 150));
        // draw::draw_rectf(0, 0, width, height);

        // Clone `Arc<Mutex<>>` for use in the closure
        let machine = Arc::clone(&self.machine);
        let max_value = Arc::clone(&self.max_value); // Now `max_value` is correctly wrapped in Arc

        window.draw(move |_| {
            /*
             * Never use self in here
             */

            // TODO println!("draw {}", cycle);

            let machine = machine.lock().unwrap(); // Access `machine` via the cloned Arc

            for y in 0..height {
                for x in 0..width {
                    let (value, state, _, _, colour_index_o) =
                        machine.data_image.values_at(x as usize, y as usize);
                    let colour: Rgb<u8>;
                    if !is_finished_any(state) {
                        colour = colour_for_state(state);
                    } else {
                        match colour_index_o {
                            Some(pixel_colour) => {
                                colour = pixel_colour;
                            }
                            None => {
                                let mut mv = max_value.lock().unwrap(); // Access `max_value` via the cloned Arc
                                if value > *mv {
                                    *mv = value;
                                }
                                // make color (3x) brighter
                                let mut cv = ((value * 3) as f64 / *mv as f64) * 255.0;
                                if cv > 255.0 {
                                    cv = 255.0;
                                }
                                let c = cv as u8;
                                colour = Rgb([c, c, c]);
                            }
                        }
                    }
                    let r = colour.channels().get(0).unwrap();
                    let g = colour.channels().get(1).unwrap();
                    let b = colour.channels().get(2).unwrap();
                    draw::set_draw_color(Color::from_rgb(*r, *g, *b));
                    draw::draw_point(x, y);
                }
            }
        });

        window.handle(move |_, event| match event {
            Event::KeyDown => {
                let ek = event_key();
                match ek {
                    Key::Escape => {
                        println!("exit");
                        // TODO self.app.quit(); // `self` is still directly referenced here
                    }
                    _ => {}
                }
                match ek.to_char().unwrap() {
                    'i' => {
                        println!("i");
                        true
                    }
                    's' => {
                        println!("s");
                        true
                    }
                    ' ' => {
                        println!("space bar");
                        // TODO probably not the right method
                        // TODO self.machine.lock().unwrap().zoom_in_recalculate_pixel_positions();
                        true
                    }
                    _ => false,
                }
            }
            Event::Released => {
                // mouse button click
                let left = event_button() == 1;
                if left {
                    let (x, y) = event_coords();
                    println!("c: {} {}", x, y);
                    //self.machine.lock().unwrap().move_target(x as usize, y as usize);
                    //self.machine.lock().unwrap().zoom_in_recalculate_pixel_positions();
                }
                false
            }
            _ => false,
        });

        window.end();
        window.show();

        println!("initiate redraw loop 0.2 s");
        app::add_idle3(move |_| {
            window.redraw();
            app::sleep(0.2);
        });
    }
}

/* --------------
 * static methods
 * ----------- */

pub fn paint_path(area: &Area, data: &DataImage) {
    let path = &data.show_path.lock().unwrap();
    let lr = app::lock();
    match lr {
        Ok(_) => {
            app::unlock();
            for p in path.as_slice() {
                let (x, y) = area.point_to_pixel(p[0], p[1]);
                draw::set_draw_color(Color::from_rgb(255, 215, 0));
                draw::draw_point(x as i32, y as i32);
            }
            // rendering must be done from main thread
            app::awake();
            app::redraw();
        }
        Err(_) => {
            println!("paint_path(): can't unlock app");
        }
    }
}

/**
 * rendering must be done from main thread
 */
pub fn paint_image_calculation_progress(xy: &[u32; 2], data: &DataImage) {
    let chunk_size_x = data.width / 20;
    let chunk_size_y = data.height / 20;

    let xx = xy[0] as usize;
    let yy = xy[1] as usize;

    let x_from = chunk_size_x * xx;
    let x_to = chunk_size_x * (xx + 1);
    let y_from = chunk_size_y * yy;
    let y_to = chunk_size_y * (yy + 1);

    let lr = app::lock();
    match lr {
        Ok(_) => {
            for y in y_from..y_to {
                for x in x_from..x_to {
                    let colour_index_o = data.colour_at(x, y);
                    match colour_index_o {
                        None => {
                            panic!();
                        }
                        Some(ci) => {
                            let r = ci.channels().get(0).unwrap();
                            let g = ci.channels().get(1).unwrap();
                            let b = ci.channels().get(2).unwrap();
                            draw::set_draw_color(Color::from_rgb(*r, *g, *b));
                            draw::draw_point(x as i32, y as i32);
                        }
                    }
                }
            }
            app::unlock();
            // rendering must be done from main thread
            app::awake();
            app::redraw();
        }
        Err(_) => {
            println!("paint_image_result(): can't unlock app");
        }
    }
}

pub fn paint_image_result(data: &DataImage) {
    let lr = app::lock();
    match lr {
        Ok(_) => {
            for y in 0..data.height {
                for x in 0..data.width {
                    let colour_index_o = data.colour_at(x, y);
                    match colour_index_o {
                        None => {
                            panic!();
                        }
                        Some(ci) => {
                            let r = ci.channels().get(0).unwrap();
                            let g = ci.channels().get(1).unwrap();
                            let b = ci.channels().get(2).unwrap();
                            draw::set_draw_color(Color::from_rgb(*r, *g, *b));
                            draw::draw_point(x as i32, y as i32);
                        }
                    }
                }
            }
            app::unlock();
            // rendering must be done from main thread
            app::awake();
            app::redraw();
        }
        Err(_) => {
            println!("paint_image_calculation_progress(): can't unlock app");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
