use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use ColorDepth::Rgb8;
use fltk::{frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_key};
use fltk::enums::{ColorDepth, Event, Key};
use fltk::image::RgbImage;
use fltk::window::DoubleWindow;
use rusty_fractals_common::data_image::DataImage;

pub struct AppWindow {
    pub window: DoubleWindow,
    pub frame: Frame,
}

pub fn init(fractal_name: &'static str, width: usize, height: usize) -> AppWindow {
    AppWindow {
        window: Window::default().with_label(fractal_name).with_size(width as i32, height as i32).center_screen(),
        frame: Frame::new(0, 0, width as i32, height as i32, ""),
    }
}

impl AppWindow {
    pub fn show(&mut self, initial_image: &Vec<u8>, width: usize, height: usize) -> App {
        let app = App::default();
        let image_rgb = RgbImage::new(initial_image, width as i32, height as i32, Rgb8).unwrap();
        self.frame.set_image(Some(image_rgb));
        self.window.add(&self.frame);
        println!("a");
        self.window.handle(move |_, event| match event {
            Event::KeyDown => {
                let ek = event_key();
                if ek == Key::Escape {
                    println!("exit");
                    app.quit();
                    true
                } else if ek == Key::IsoKey {
                    println!("pressed {}", ek.to_char().unwrap());
                    true
                } else {
                    println!("key {}", ek.to_char().unwrap());
                    false
                }
            }
            _ => false,
        });
        self.window.end();
        self.window.show();
        app
    }

    pub fn refresh(&mut self, data_image: &DataImage) {
        let image_rgb = RgbImage::new(data_image.image().as_raw(), data_image.width as i32, data_image.height as i32, Rgb8).unwrap();
        self.frame.set_image(Some(image_rgb));
        self.window.redraw();
    }
}

pub fn refresh(data_image: &DataImage, arc_mutex_window: &Arc<Mutex<AppWindow>>) {
    let mut mutex_guard = arc_mutex_window.lock().unwrap();
    let mut app_window = mutex_guard.borrow_mut();
    let image_rgb = RgbImage::new(data_image.image().as_raw(), data_image.width as i32, data_image.height as i32, Rgb8).unwrap();
    app_window.frame.set_image(Some(image_rgb));
    app_window.window.redraw();
    app_window.refresh(&data_image);
}

