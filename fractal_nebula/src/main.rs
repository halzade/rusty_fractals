use resolution_multiplier::ResolutionMultiplier::SquareAlter;
use rusty_fractals_common::area;
use rusty_fractals_core::fractal::{AppConfig, CalculationConfig, Math, ResultConfig};
use rusty_fractals_core::machine::Machine;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_domain::domain::{init_domain_elements, Domain};
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_result::palettes::palette_blue_to_white;
use image::{DynamicImage, ImageBuffer, ImageResult, Rgb};
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk::app::App;
use fltk::enums::ColorDepth;
use fltk::image::RgbImage;

struct Nebula {}

impl Math<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name = "Nebula";

    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_cfg = area::AreaConfig {
        width_re: 7.0,
        center_re: 0.0,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white(),
    };

    println!("Fractal {}", name);

    let nebula = Nebula {};
    let domain_area = area::init(area_cfg);
    let domain = Domain {
        width: domain_area.width_x,
        height: domain_area.height_y,
        domain_area: &domain_area,
        domain_elements: init_domain_elements(&domain_area),
        resolution_multiplier: SquareAlter,
    };
    let mut machine = Machine {
        area: &domain_area,
        domain: &domain,
        calculation_config,
        app_config,
        result_config,
    };

    let (domain_image, result_image) = machine.calculate(&nebula);

    let w = domain_image.width() as i32;
    let h = domain_image.height() as i32;
    let domain_image_rgb = RgbImage::new(&domain_image.into_raw(), w, h, ColorDepth::Rgb8).unwrap();
    let result_image_rgb = RgbImage::new(&result_image.into_raw(), w, h, ColorDepth::Rgb8).unwrap();

    let app = App::default();
    let mut wind = Window::new(100, 100, 800, 400, name);
    let mut domain_frame = Frame::new(0, 0, 400, 400, "");
    let mut result_frame = Frame::new(400, 0, 400, 400, "");
    domain_frame.set_image(Some(domain_image_rgb));
    result_frame.set_image(Some(result_image_rgb));
    wind.add(&domain_frame);
    wind.add(&result_frame);
    wind.end();
    wind.show();
    app.run().unwrap();
}

#[test]
fn test_math() {
    let nebula = Nebula {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
