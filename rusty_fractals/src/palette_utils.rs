use image::{Pixel, Rgb};
use palettes::Function;

use crate::palettes;

fn max(r: i32, g: i32, b: i32) -> i32 {
    if a(r) >= a(g) && a(r) >= a(b) {
        return r;
    } else if a(g) >= a(r) && a(g) >= a(b) {
        return g;
    } else if a(b) >= a(r) && a(b) >= a(g) {
        return b;
    }
    panic!()
}

fn a(v: i32) -> u8 {
    v.abs() as u8
}

// Fill color spectrum with colors between colors:
// from     : color for lower values
// to       : color for higher values
// function : defines gradient of color change
pub fn make_spectrum(function: Function, from: Rgb<u8>, to: Rgb<u8>) -> Vec<Rgb<u8>> {
    let r_from = from.channels()[0];
    let g_from = from.channels()[1];
    let b_from = from.channels()[2];

    let r_to = to.channels()[0];
    let g_to = to.channels()[1];
    let b_to = to.channels()[2];

    // result may be negative number
    let r_dif = r_to as i32 - r_from as i32;
    let g_dif = g_to as i32 - g_from as i32;
    let b_dif = b_to as i32 - b_from as i32;

    println!("rgb from {} {} {} ", r_from, g_from, b_from);
    println!("rgb to   {} {} {} ", r_to, g_to, b_to);
    println!("rgb dif  {} {} {} ", r_dif, g_dif, b_dif);

    let max_dif = max(r_dif, g_dif, b_dif) as f64;
    let max_dif_abs = a(max_dif as i32) as u32;
    println!("max dif: {}", max_dif);

    let r_step: f64 = r_dif as f64 / max_dif_abs as f64;
    let g_step: f64 = g_dif as f64 / max_dif_abs as f64;
    let b_step: f64 = b_dif as f64 / max_dif_abs as f64;
    println!("step r: {}", r_step);
    println!("step g: {}", g_step);
    println!("step b: {}", b_step);

    let mut stop = false;

    let rgb255 = 255.0;

    let mut spectrum: Vec<Rgb<u8>> = Vec::new();

    // first color
    spectrum.push(from.clone());

    for i in 1..max_dif_abs {
        // if from i=0, then d could be -0
        let d: f64 = i as f64 / max_dif_abs as f64;
        // optimized dif on interval <0, 1>
        // 0 -> 1, like circle up is forward
        // 1 -> 0, like circle down is backwards
        let v: f64 = function_result(d, &function);
        let value = v * max_dif_abs as f64;

        let mut r_new = r_from as f64 + (value * r_step);
        let mut g_new = g_from as f64 + (value * g_step);
        let mut b_new = b_from as f64 + (value * b_step);

        if r_new > rgb255 {
            r_new = rgb255;
            stop = true;
        }
        if g_new > rgb255 {
            g_new = rgb255;
            stop = true;
        }
        if b_new > rgb255 {
            b_new = rgb255;
            stop = true;
        }

        if r_new < 0.0 {
            r_new = 0.0;
            stop = true;
        }
        if g_new < 0.0 {
            g_new = 0.0;
            stop = true;
        }
        if b_new < 0.0 {
            b_new = 0.0;
            stop = true;
        }

        let r_stop = if r_dif > 0 {
            (r_to as f64) < r_new
        } else {
            (r_to as f64) > r_new
        };
        let g_stop = if g_dif > 0 {
            (g_to as f64) < g_new
        } else {
            (g_to as f64) > g_new
        };
        let b_stop = if b_dif > 0 {
            (b_to as f64) < b_new
        } else {
            (b_to as f64) > b_new
        };

        if r_stop {
            r_new = r_to as f64;
            stop = true;
        }
        if g_stop {
            g_new = g_to as f64;
            stop = true;
        }
        if b_stop {
            b_new = b_to as f64;
            stop = true;
        }

        // Add colo§rs to Palette
        spectrum.push(Rgb([r_new as u8, g_new as u8, b_new as u8]));
        // println!("{} {} {}", r_new as u8, g_new as u8, b_new as u8);
        if stop {
            break;
        }
    }

    // last color
    spectrum.push(to.clone());

    spectrum
}

// Calculates how much should color in smooth color palette change
// function : defines gradient of change from color "from" (d=0) to color "to" (d=1)
// d : 0 <= d <= 1
fn function_result(d: f64, function: &Function) -> f64 {
    match function {
        Function::Linear1 => d,
        Function::Linear3 => d * 3.0,
        Function::Linear7 => d * 7.0,
        Function::Quadratic => d * d,
        Function::Exp => d.exp() - 1.0,
        Function::Exp2 => (d * d).exp() - 1.0,
        Function::CircleDown => (1.0 - (d * d)).sqrt(), // TODO not ok
        Function::CircleUp => 1.0 - (1.0 - (d * d)).sqrt(),
    }
}

pub fn init_default() -> Vec<Rgb<u8>> {
    let mut spectrum: Vec<Rgb<u8>> = Vec::new();
    spectrum.push(Rgb([255, 0, 0]));
    spectrum.push(Rgb([0, 255, 0]));
    spectrum.push(Rgb([0, 0, 255]));
    spectrum
}

#[cfg(test)]
mod tests {
    use crate::palette_utils::{function_result, make_spectrum};
    use crate::palettes::Function::Linear1;
    use image::{Pixel, Rgb};

    #[test]
    fn test_function_result() {
        let f = Linear1;
        assert_eq!(function_result(0.0, &f), 0.0);
        assert_eq!(function_result(0.1, &f), 0.1);
        assert_eq!(function_result(0.5, &f), 0.5);
        assert_eq!(function_result(1.0, &f), 1.0);
    }

    #[test]
    fn test_make_spectrum() {
        let b1: Rgb<u8> = Rgb([0, 0, 0]);
        let b2: Rgb<u8> = Rgb([2, 2, 2]);
        // light to dark
        let res = make_spectrum(Linear1, b1, b2);

        let r1 = res.get(0).unwrap().channels()[0];
        let r2 = res.get(1).unwrap().channels()[0];
        let r3 = res.get(2).unwrap().channels()[0];
        assert_eq!(r1, 0);
        assert_eq!(r2, 1);
        assert_eq!(r3, 2);
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_make_spectrum_5() {
        let b1: Rgb<u8> = Rgb([0, 4, 2]);
        let b2: Rgb<u8> = Rgb([4, 1, 4]);
        // light to dark
        let res = make_spectrum(Linear1, b1, b2);

        let r3 = res.get(2).unwrap().channels()[0];
        let g3 = res.get(2).unwrap().channels()[1];
        let b3 = res.get(2).unwrap().channels()[2];
        assert_eq!(r3, 2);
        assert_eq!(g3, 2);
        assert_eq!(b3, 3);
        assert_eq!(res.len(), 5);
    }

    #[test]
    fn test_make_spectrum_inv() {
        let b2: Rgb<u8> = Rgb([2, 2, 2]);
        let b1: Rgb<u8> = Rgb([0, 0, 0]);
        // dark to light
        let res = make_spectrum(Linear1, b2, b1);
        let r1 = res.get(0).unwrap().channels()[0];
        let r2 = res.get(1).unwrap().channels()[0];
        let r3 = res.get(2).unwrap().channels()[0];
        assert_eq!(r1, 2);
        assert_eq!(r2, 1);
        assert_eq!(r3, 0);
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_make_spectrum_inv_5() {
        let b2: Rgb<u8> = Rgb([4, 2, 0]);
        let b1: Rgb<u8> = Rgb([0, 0, 4]);
        // dark to light
        let res = make_spectrum(Linear1, b2, b1);
        let r1 = res.get(0).unwrap().channels()[0];
        let g1 = res.get(0).unwrap().channels()[1];
        let b1 = res.get(0).unwrap().channels()[2];
        assert_eq!(r1, 4);
        assert_eq!(g1, 2);
        assert_eq!(b1, 0);

        let r3 = res.get(2).unwrap().channels()[0];
        let g3 = res.get(2).unwrap().channels()[1];
        let b3 = res.get(2).unwrap().channels()[2];
        assert_eq!(r3, 2);
        assert_eq!(g3, 1);
        assert_eq!(b3, 2);

        let r5 = res.get(4).unwrap().channels()[0];
        let g5 = res.get(4).unwrap().channels()[1];
        let b5 = res.get(4).unwrap().channels()[2];
        assert_eq!(r5, 0);
        assert_eq!(g5, 0);
        assert_eq!(b5, 4);

        assert_eq!(res.len(), 5);
    }
}
