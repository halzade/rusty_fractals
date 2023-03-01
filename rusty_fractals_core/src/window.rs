use fltk::{app, frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_button, event_coords, event_key, Receiver, Sender};
use fltk::enums::{ColorDepth, Event, Key};
use fltk::image::RgbImage;
use ColorDepth::Rgb8;
use rusty_fractals_common::area;
use rusty_fractals_common::area::{Area, AreaConfig};

pub fn show(fractal_name: &'static str, initial_image: Vec<u8>, area_config: AreaConfig) -> (App, Area, Sender<Vec<u8>>) {
    let width = area_config.width_x;
    let height = area_config.height_y;
    let area: Area = area::init(&area_config);
    let init_image_rgb = RgbImage::new(&initial_image, width as i32, height as i32, Rgb8).unwrap();
    let app = App::default();
    let mut window = Window::default().with_label(fractal_name).with_size(width as i32, height as i32).center_screen();
    let mut frame = Frame::new(0, 0, width as i32, height as i32, "");
    frame.set_image(Some(init_image_rgb));
    window.add(&frame);

    let (sender_machine, receiver_frame): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = app::channel();

    window.handle(move |_, event| match event {
        Event::KeyDown => {
            let ek = event_key();
            if ek == Key::Escape {
                println!("exit");
                app.quit();
                true
            } else {
                let key = ek.to_char().unwrap();
                if key == 'i' {
                    println!("i");
                    return true;
                } else if key == 's' {
                    println!("s");
                    return true;
                }
                false
            }
        }
        Event::Released => {
            // mouse button click
            let left = event_button() == 1;
            if left {
                let (x, y) = event_coords();
                println!("c: {} {}", x, y);
                // to change target coordinates
                // sender_window.send([x as usize, y as usize]);
                area::move_target(x as usize, y as usize);
            }
            false
        }
        _ => false,
    });
    app::add_idle3(move |_| {
        let bo = receiver_frame.recv();
        match bo {
            None => {}
            Some(data) => {
                println!("refresh");
                let image_rgb = RgbImage::new(data.as_slice(), width as i32, height as i32, Rgb8).unwrap();
                frame.set_image(Some(image_rgb));
                app::unlock();
                // rendering must be done from main thread
                app::awake();
                app::redraw();
                println!("refresh ok");
            }
        }
    });

    window.end();
    window.show();
    (app, area, sender_machine)
}
