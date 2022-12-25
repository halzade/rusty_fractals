use color_palette::Palette;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, MathCollatz};
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::mem_collatz::MemCollatz;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::None;
use rusty_fractals_image::color_palette;
use rusty_fractals_image::color_palettes::{PALETTE_BLUE_TO_WHITE, PALETTE_GRAY_TO_BLUE};

const NAME: &str = "Collatz Conjecture Mandelbrot";

struct CollatzConjectureMandelbrot {
    pub name: String,
}

impl MathCollatz for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.m.square();
        mc.plus_collatz(origin_re, origin_im);
    }
}

fn main() {
    println!("Started");

    let collatz = CollatzConjectureMandelbrot { name: NAME.to_string() };
    let definition = FractalDefinition {
        iteration_min: 0,
        iteration_max: 14800,
        area_size:  3.0,
        target_re: -0.882952991714172300,
        target_im: -0.214699221335319460,
        resolution_width: 1280,
        resolution_height: 720,
        resolution_multiplier: None,
        repeat: true,
        save_images: false,
        palette: PALETTE_GRAY_TO_BLUE
    };

    println!("Fractal {}", collatz.name);

    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 0 };
    collatz.math(&mut mc, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let collatz = CollatzConjectureMandelbrot { name: NAME.to_string() };
    let m = Mem { re: 0.0, im: 0.0 };
    let mut mc = MemCollatz { m, it: 0 };
    collatz.math(&mut mc, 1.0, 0.1);
    assert_eq!(mc.re(), 2.0);
    assert_eq!(mc.im(), 0.65);
}
