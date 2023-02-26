use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use fltk::{frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_key};
use fltk::enums::{ColorDepth, Event, Key};
use fltk::image::RgbImage;
use fltk::window::DoubleWindow;
use ColorDepth::Rgb8;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::REFRESH_MS;
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
        self.window.handle(move |_, event| match event {
            Event::KeyDown => {
                let ek = event_key();
                if ek == Key::Escape {
                    println!("exit");
                    app.quit();
                    true
                } else {
                    let key = ek.to_char().unwrap();
                    println!("key {}", key);
                    if key == 'i' {
                        return true;
                    } else if key == 's' {
                        return true;
                    }
                    false
                }
            }
            _ => false,
        });
        self.window.end();
        self.window.show();
        app
    }

    pub fn refresh(&mut self, data_image: &DataImage, final_image: bool, area_o: Option<&Area>) {
        let image_rgb = RgbImage::new(data_image.image(final_image, area_o).as_slice(), data_image.width as i32, data_image.height as i32, Rgb8).unwrap();
        let _ = fltk::app::lock();
        self.frame.set_image(Some(image_rgb));
        let _ = fltk::app::unlock();
        // rendering must be done from main thread
        fltk::app::awake();
        fltk::app::redraw();
    }
}

fn refresh_unlocked(data_image: &DataImage, arc_mutex_window: &Arc<Mutex<AppWindow>>, area_o: Option<&Area>) {
    let mut mutex_guard = arc_mutex_window.lock().unwrap();
    let app_window = mutex_guard.borrow_mut();
    match area_o {
        Some(area) => {
            app_window.refresh(data_image, false, Some(area));
        }
        None => {
            app_window.refresh(data_image, false, None);
        }
    }
}

pub fn refresh_maybe(data_image: &DataImage, arc_mutex_window: &Arc<Mutex<AppWindow>>, o_refresh_locker: Option<&Arc<Mutex<SystemTime>>>, area_o: Option<&Area>) {
    match o_refresh_locker {
        None => {
            refresh_unlocked(data_image, arc_mutex_window, area_o);
        }
        Some(refresh_locker) => {
            let ms = SystemTime::now().duration_since(*refresh_locker.lock().unwrap()).unwrap().as_millis();
            if ms > REFRESH_MS {
                refresh_unlocked(data_image, arc_mutex_window, area_o);
                *refresh_locker.lock().unwrap().deref_mut() = SystemTime::now();
            }
        }
    }
}

pub fn refresh_final(data_image: &DataImage, arc_mutex_window: &Arc<Mutex<AppWindow>>) {
    let mut mutex_guard = arc_mutex_window.lock().unwrap();
    let app_window = mutex_guard.borrow_mut();
    app_window.refresh(data_image, true, None);
}
