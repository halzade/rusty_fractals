use fltk::{app, enums::Color, prelude::*, window::Window};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// The Application struct that manages the Window and interacts with Machine
pub struct Application {
    pub window: Arc<Mutex<Window>>, // Shared ownership of the window via Mutex
    pub machine: Arc<Mutex<Machine>>, // Mutex to manage shared access to the Machine
}

impl Application {
    // Create a new application with a 200x200 window
    pub fn new() -> Self {
        // Create an FLTK window (200x200) and set its initial background color to red
        let mut window = Window::new(100, 100, 200, 200, "Window Example");
        window.set_color(Color::Red);
        window.show();

        let window = Arc::new(Mutex::new(window));
        let machine = Arc::new(Mutex::new(Machine::new()));

        // Create the Application
        let app = Application {
            window: Arc::clone(&window),
            machine: Arc::clone(&machine),
        };

        // Set the Machine's reference to the Application's Window
        machine.lock().unwrap().set_application_window(Arc::clone(&window));

        app
    }

    // Triggers the calculation on the Machine
    pub fn trigger_calculation(&self) {
        let machine = Arc::clone(&self.machine);

        println!("Triggering calculation on the Machine...");

        // Spawn a new thread to avoid blocking the GUI
        thread::spawn(move || {
            machine.lock().unwrap().perform_calculation();
        });
    }
}

// The Machine struct that performs calculations
pub struct Machine {
    pub app_window: Option<Arc<Mutex<Window>>>, // Reference to Application's Window
}

impl Machine {
    pub fn new() -> Self {
        Machine { app_window: None }
    }

    pub fn set_application_window(&mut self, window: Arc<Mutex<Window>>) {
        self.app_window = Some(window);
    }

    // Simulated long-running calculation
    pub fn perform_calculation(&self) {
        println!("Machine is performing calculations...");

        // Simulate a long calculation
        thread::sleep(Duration::from_secs(2));

        println!("Machine has completed calculations.");

        // Update the window color to green after the calculation
        if let Some(window) = &self.app_window {
            // Lock the window and change its background color
            let mut window = window.lock().unwrap();
            window.set_color(Color::Green);
            window.redraw(); // Trigger the redraw immediately

            // Ensure that the FLTK event loop wakes up to process the redraw
            app::awake();
        }
    }
}

fn main() {
    // Create an FLTK app instance
    let app = app::App::default();

    // Create the Application
    let application = Application::new();

    // Trigger the Machine's calculation
    application.trigger_calculation();

    // Keep the application in the event loop to remain responsive
    app.run().unwrap();
}