use crate::data_image::{color_for_state, DataImage};
use crate::fractal::{FractalConfig, FractalMath, MemType, Optimizer};
use crate::machine;
use crate::machine::Machine;
use crate::pixel_states::{is_active_new, DomainElementState};
use fltk::app::{event_button, event_coords, event_key};
use fltk::enums::{Color, Event, Key};
use fltk::window::DoubleWindow;
use fltk::{app, draw, prelude::*, window::Window};
use image::{Pixel, Rgb};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/**
 * Application owns Machine
 */
pub struct Application<F, M>
where
    F: FractalMath<M> + 'static,
    M: MemType<M> + 'static,
{
    /* DoubleWindow class provides a **double-buffered** window.
    - In double buffering:
    - All drawing operations are first performed in an **off-screen buffer**.
    - Once the drawing is complete, the off-screen buffer is copied (or "flipped") onto the screen in a single operation.
    - This eliminates flickering during redraws, as the user only sees the final, fully-drawn frame.*/
    pub window: Arc<RwLock<DoubleWindow>>, // Shared ownership of the GUI Window
    application_data: Arc<RwLock<ApplicationData>>,
    pub machine_arc: Arc<RwLock<Machine<'static, F, M>>>,
    pub is_shutting_down: Arc<AtomicBool>,
}

struct ApplicationData {
    pub last_max_value: u64,
}

fn init_o<'lt, F, M>(
    config: &FractalConfig,
    fractal: F,
    oo: Option<Optimizer>,
) -> Arc<RwLock<Application<F, M>>>
where
    F: FractalMath<M> + 'static,
    M: MemType<M> + 'static,
{
    let mut window = Window::default();
    let name = config.name;

    window.set_label(name);
    window.set_size(config.width_xl as i32, config.height_yl as i32);

    window.end();
    window.show();

    let machine = machine::init_o(&config, fractal, oo);
    let machine_arc = Arc::new(RwLock::new(machine));

    let application = Application {
        window: Arc::new(RwLock::new(window)),
        application_data: Arc::new(RwLock::new(ApplicationData { last_max_value: 0 })),
        machine_arc,
        is_shutting_down: Arc::new(Default::default()),
    };

    let application_arc = Arc::new(RwLock::new(application));

    // Set Application ref for Machine
    application_arc
        .write()
        .unwrap()
        .machine_arc
        .write()
        .unwrap()
        .set_application_ref(application_arc.clone());

    application_arc
}

pub fn execute<F, M>(config: FractalConfig, fractal: F)
where
    F: FractalMath<M> + 'static,
    M: MemType<M> + 'static,
{
    execute_o(config, fractal, None)
}
/**
 * start the application
 */
pub fn execute_o<F, M>(config: FractalConfig, fractal: F, oo: Option<Optimizer>)
where
    F: FractalMath<M> + 'static,
    M: MemType<M> + 'static,
{
    println!("application.execute()");

    let app = app::App::default();
    let application_arc = init_o(&config, fractal, oo);

    // Window actions
    application_arc.read().unwrap().init_window_actions();

    let machine_arc_clone = application_arc.read().unwrap().machine_arc.clone();

    println!("calculation - new thread ");
    let task = move || {
        /*
         * execute fractal calculation
         */
        machine_arc_clone.read().unwrap().execute_calculation();
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
impl<F, M> Application<F, M>
where
    F: FractalMath<M> + 'static,
    M: MemType<M> + 'static,
{
    pub fn init_window_actions(&self) {
        println!("init_window_actions()");

        let shutdown_flag = self.is_shutting_down.clone();

        // clone Arc, not Machine
        let machine_ref = self.machine_arc.clone();

        self.window
            .write()
            .unwrap()
            .handle(move |_, event| match event {
                Event::KeyDown => {
                    let ek = event_key();
                    match ek {
                        Key::Escape => {
                            println!("exit");
                            shutdown_flag.store(true, Ordering::Relaxed); // Signal shutdown
                            app::awake(); // Wake the app so it can break the event loop
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
                            machine_ref
                                .read()
                                .unwrap()
                                .zoom_in_recalculate_pixel_positions();
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

                        machine_ref
                            .read()
                            .unwrap()
                            .move_target(x as usize, y as usize);

                        machine_ref
                            .read()
                            .unwrap()
                            .zoom_in_recalculate_pixel_positions();
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
                let width = data_image.width_xp;
                let height = data_image.height_yp;

                let pixel_colors: Vec<Option<Rgb<u8>>> = (0..height)
                    .flat_map(|y| (0..width).map(move |x| data_image.color_at(x, y)))
                    .collect();

                let mut window = self.window.write().unwrap();
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
     * This method paints states from data_image
     * For finished states it uses color instead
     *
     * ------
     * STATES
     * ------
     */
    pub fn paint_partial_calculation_result_states(&self, data_image: &DataImage) {
        match app::lock() {
            Ok(_) => {
                let width = data_image.width_xp;
                let height = data_image.height_yp;

                // pixel states and image colors
                let pixel_states: Vec<(u64, DomainElementState, Option<Rgb<u8>>)> = (0..height)
                    .flat_map(|y| {
                        (0..width).map(move |x| {
                            let (value, state, color_opt) = data_image.values_state_color_at(x, y);
                            let rgb_color = color_opt.map(|c| Rgb(c.0));
                            (value, state, rgb_color)
                        })
                    })
                    .collect();

                // clone Arc
                let app_data = self.application_data.clone();

                let mut window = self.window.write().unwrap();

                window.draw(move |_| {
                    /* --------------------------------------------------------------------------------
                     * All painting must be done within draw() method. Otherwise it doesn't do anything
                     * ----------------------------------------------------------------------------- */

                    for y in 0..height {
                        for x in 0..width {

                            // read data
                            let (value, state, color_index_o) = pixel_states[y * width + x];
                            let color: Rgb<u8>;
                            if is_active_new(state) {

                                // paint state
                                color = color_for_state(state);
                            } else {

                                // finished, use color
                                match color_index_o {
                                    Some(ci) => {
                                        color = ci;
                                    }
                                    None => {
                                        let mv = app_data.read().unwrap().last_max_value;
                                        if value > mv {
                                            app_data.write().unwrap().last_max_value = value;
                                        }
                                        // make color 3x brighter
                                        // (0-1) * 255
                                        let mut cv = (value as f64 * 3.0 / mv as f64) * 255.0;
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

    /**
     * paint only pixel states
     */
    pub fn paint_pixel_states(&self, data_image: &DataImage) {
        match app::lock() {
            Ok(_) => {
                let width = data_image.width_xp;
                let height = data_image.height_yp;

                // pixel states and image colors
                let pixel_states: Vec<DomainElementState> = (0..height)
                    .flat_map(|y| (0..width).map(move |x| data_image.state_at(x, y)))
                    .collect();

                let mut window = self.window.write().unwrap();

                window.draw(move |_| {
                    /* --------------------------------------------------------------------------------
                     * All painting must be done within draw() method. Otherwise it doesn't do anything
                     * ----------------------------------------------------------------------------- */

                    for y in 0..height {
                        for x in 0..width {

                            // read data
                            let state = pixel_states[y * width + x];
                            let color = color_for_state(state);
                            draw_colored_point(x, y, &color);
                        }
                    }
                });
                // Trigger redraw events from the main thread
                window.redraw();
                app::awake();
            }
            Err(_) => {
                println!("paint_pixel_states(): app::lock() failed");
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
