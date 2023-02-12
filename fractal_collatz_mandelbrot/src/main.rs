use rusty_fractals_common::fractal::Fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_core::mem_collatz::MemCollatz;
use rusty_fractals_result::palettes::palette_gray_to_blue;

const NAME: &str = "Collatz Conjecture Mandelbrot";

struct CollatzConjectureMandelbrot {
    pub name: String,
}

impl Fractal<MemCollatz> for CollatzConjectureMandelbrot {
    fn math(&self, mc: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        mc.m.square();
        mc.plus_collatz(origin_re, origin_im);
    }

    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
}

fn main() {
    /*
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
        palette: palette_gray_to_blue()
    */

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
