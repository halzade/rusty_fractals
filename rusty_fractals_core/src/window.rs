use fltk::{app, frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_button, event_coords, event_key, Receiver, Sender};
use fltk::enums::{ColorDepth, Event, Key};
use fltk::image::RgbImage;
use ColorDepth::Rgb8;
use rusty_fractals_common::area;

pub fn show(fractal_name: &'static str, initial_image: Vec<u8>, width: i32, height: i32) -> (App, Sender<Vec<u8>>) {
    let init_image_rgb = RgbImage::new(&initial_image, width, height, Rgb8).unwrap();
    let app = App::default();
    let mut window = Window::default().with_label(fractal_name).with_size(width, height).center_screen();
    let mut frame = Frame::new(0, 0, width, height, "");
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
                let image_rgb = RgbImage::new(data.as_slice(), width, height, Rgb8).unwrap();
                frame.set_image(Some(image_rgb));
                app::unlock();
                // rendering must be done from main thread
                app::awake();
                app::redraw();
            }
        }
    });

    window.end();
    window.show();
    (app, sender_machine)
}
