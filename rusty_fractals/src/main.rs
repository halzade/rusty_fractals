use fltk::{app, enums::Color, prelude::*, window::Window};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// The Application struct manages the Window and coordinates with the Machine
pub struct Application {
    pub window: Arc<Mutex<Window>>, // Shared ownership of the GUI Window
}

impl Application {
    // Create a new application with an initial red window
    pub fn new() -> Arc<Mutex<Self>> {
        let mut window = Window::new(100, 100, 200, 200, "Window Example");
        window.set_color(Color::Red);
        window.show();

        Arc::new(Mutex::new(Self {
            window: Arc::new(Mutex::new(window)),
        }))
    }

    // Repaint the window with a new color
    pub fn repaint(&self, color: Color) {
        let mut window = self.window.lock().unwrap();
        window.set_color(color);
        window.redraw();
        app::awake(); // Ensure the event loop processes the change immediately
    }

    // Trigger a long-running machine calculation
    pub fn trigger_calculation(app_ref: Arc<Mutex<Self>>) {
        println!("Triggering calculation on the Machine...");

        thread::spawn(move || {
            let machine = Machine::new();
            machine.perform_calculation(app_ref); // Pass the application reference directly
        });
    }
}

// The Machine struct, which executes background calculations
pub struct Machine;

impl Machine {
    pub fn new() -> Self {
        Self {}
    }

    // Simulated long-running calculation
    pub fn perform_calculation(&self, app_ref: Arc<Mutex<Application>>) {
        println!("Machine is performing calculations...");

        // Simulate time-consuming computation
        thread::sleep(Duration::from_secs(2));

        println!("Machine has completed calculations.");

        // Notify the application to repaint the window
        let app = app_ref.lock().unwrap();
        app.repaint(Color::Green);
    }
}

// Application entry point
fn main() {
    let app = app::App::default();
    let application = Application::new();

    // Trigger the calculation on the Machine
    Application::trigger_calculation(application.clone());

    // Keep the main event loop running for the GUI
    app.run().unwrap();
}