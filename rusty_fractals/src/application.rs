use crate::area::Area;
use crate::data_image::{colour_for_state, DataImage};
use crate::fractal::{FractalConfig, FractalMath, MemType};
use crate::pixel_states::DomainElementState;
use crate::{machine, pixel_states};
use fltk::app::{event_button, event_coords, event_key};
use fltk::enums::{Color, Event, Key};
use fltk::window::DoubleWindow;
use fltk::{app, draw, prelude::*, window::Window};
use image::{Pixel, Rgb};
use std::sync::{Arc, Mutex};

pub struct Application {
    /* DoubleWindow class provides a **double-buffered** window.
    - In double buffering:
    - All drawing operations are first performed in an **off-screen buffer**.
    - Once the drawing is complete, the off-screen buffer is copied (or "flipped") onto the screen in a single operation.
    - This eliminates flickering during redraws, as the user only sees the final, fully-drawn frame.*/
    pub window: Arc<Mutex<DoubleWindow>>, // Shared ownership of the GUI Window
}

fn init(config: &FractalConfig) -> Arc<Mutex<Application>> {
    let mut window = Window::default();

    let width = config.width_x as i32;
    let height = config.height_y as i32;
    let name = config.name;

    window.set_label(name);
    window.set_size(width, height);

    window.end();
    window.show();

    Arc::new(Mutex::new(Application {
        window: Arc::new(Mutex::new(window)),
    }))
}

/**
 * start the application
 */
pub fn execute<F, M>(config: FractalConfig, fractal: F)
where
    F: FractalMath<M> + 'static,
    M: MemType<M>,
{
    println!("application.execute()");

    let app = app::App::default();
    let application_arc = init(&config);

    println!("show()");
    application_arc.lock().unwrap().init_window_actions();

    println!("calculation - new thread ");
    let task = move || {
        // clone arc, not application
        let mut machine = machine::init(&config, fractal);
        machine.set_application_ref(application_arc.clone());
        // execute fractal calculation
        machine.execute_calculation();
    };
    rayon::spawn_fifo(task);

    println!("run().unwrap()");
    // The last line of the program
    app.run().unwrap();

    println!("execute() end.");
}

/**
 * Use static calls to communicate between app and Machine
 */
impl Application {
    pub fn init_window_actions(&mut self) {
        println!("init_window_actions()");

        self.window
            .lock()
            .unwrap()
            .handle(move |_, event| match event {
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
                            // TODO self.machine.zoom_in_recalculate_pixel_positions();
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
                        //self.machine.move_target(x as usize, y as usize);
                        //self.machine.zoom_in_recalculate_pixel_positions();
                    }
                    false
                }
                _ => false,
            });
    }

    /**
     * This method paints only colors from data_image.
     * Use other painting methods to display the element states before and during calculation.
     *
     * ------
     * Colors
     * ------
     */
    pub fn paint_final_calculation_result_colors(&self, data_image: &DataImage) {
        match app::lock() {
            Ok(_) => {
                let width = data_image.width_x;
                let height = data_image.height_y;

                let pixel_colors: Vec<Option<Rgb<u8>>> = (0..height)
                    .flat_map(|y| (0..width).map(move |x| data_image.colour_at(x, y)))
                    .collect();

                let mut window = self.window.lock().unwrap();
                window.draw(move |_| {
                    // never use self in here
                    // locking / unlocking app for draw is not necessary, says so AI
                    // redraw() can't be called from draw()

                    for y in 0..height {
                        for x in 0..width {
                            let color_index = pixel_colors[y * width + x];
                            if let Some(color) = color_index {
                                draw_colored_point(x, y, &color);
                            }
                        }
                    }
                });

                // Trigger redraw events from the main thread
                window.redraw();
                app::awake();
            }
            Err(_) => {
                println!("paint_final_calculation_result_colors(): app::lock() failed");
            }
        }
        app::unlock();
    }

    /**
     * This method paints states from data_image.
     * For finished states it uses color instead
     *
     * ------
     * STATES
     * ------
     */
    pub fn paint_partial_calculation_result_states(
        &self,
        data_image: &DataImage,
        paint_path: bool,
        area: &Area,
    ) {
        match app::lock() {
            Ok(_) => {
                let width = data_image.width_x;
                let height = data_image.height_y;

                // pixel states and image colors
                let pixel_states: Vec<(u32, DomainElementState, Option<Rgb<u8>>)> = (0..height)
                    .flat_map(|y| {
                        (0..width).map(move |x| {
                            let (value, state, color_opt) = data_image.values3_at(x, y);
                            let rgb_color = color_opt.map(|c| Rgb(c.0));
                            (value, state, rgb_color)
                        })
                    })
                    .collect();

                // calculation path
                let mut path: Vec<[f64; 2]> = Vec::new();
                let mut area_copy = area.copy_data();
                if paint_path {
                    path = {
                        let mut locked_path = data_image.show_path.lock().unwrap();
                        std::mem::replace(&mut *locked_path, Default::default())
                        // Replace with a default value
                    };
                    area_copy = area.copy_data();
                }

                let max_value = Arc::new(Mutex::new(0));

                let mut window = self.window.lock().unwrap();

                window.draw(move |_| {
                    /* --------------------------------------------------------------------------------
                     * All painting must be done within draw() method. Otherwise it doesn't do anything
                     * ----------------------------------------------------------------------------- */

                    for y in 0..height {
                        for x in 0..width {
                            let (value, state, colour_index_o) = pixel_states[y * width + x];
                            let color: Rgb<u8>;
                            if !pixel_states::is_finished_any(state) {
                                // paint state
                                color = colour_for_state(state);
                            } else {
                                // finished, use color
                                match colour_index_o {
                                    Some(ci) => {
                                        color = ci;
                                    }
                                    None => {
                                        let mut mv = max_value.lock().unwrap();
                                        if value > *mv {
                                            *mv = value;
                                        }
                                        // make color (3x) brighter
                                        let mut cv = ((value * 3) as f64 / *mv as f64) * 255.0;
                                        if cv > 255.0 {
                                            cv = 255.0;
                                        }
                                        let c = cv as u8;
                                        color = Rgb([c, c, c]);
                                    }
                                }
                            }
                            draw_colored_point(x, y, &color);
                        }
                    }

                    if paint_path {
                        draw::set_draw_color(Color::from_rgb(255, 215, 0));
                        for p in path.as_slice() {
                            let (x, y) = area_copy.point_to_pixel(p[0], p[1]);
                            draw::draw_point(x as i32, y as i32);
                        }
                    }
                });
                // Trigger redraw events from the main thread
                window.redraw();
                app::awake();
            }
            Err(_) => {
                println!("paint_partial_calculation_result_states(): app::lock() failed");
            }
        }
        app::unlock();
    }
}

/* --------------
 * static methods
 * ----------- */

// called only from main thread within window.show() method
fn draw_colored_point(x: usize, y: usize, color: &Rgb<u8>) {
    let r = *color.channels().get(0).unwrap();
    let g = *color.channels().get(1).unwrap();
    let b = *color.channels().get(2).unwrap();

    draw::set_draw_color(Color::from_rgb(r, g, b));
    draw::draw_point(x as i32, y as i32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
