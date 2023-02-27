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

// Fill colour spectrum with colours between colours:
// from     : colour for lower values
// to       : colour for higher values
// function : defines gradient of colour change
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
    let max_dif_abs = a(max_dif as i32);
    println!("max dif: {}", max_dif);

    let r_step: f64 = (r_dif as f64 / max_dif_abs as f64) as f64;
    let g_step: f64 = (g_dif as f64 / max_dif_abs as f64) as f64;
    let b_step: f64 = (b_dif as f64 / max_dif_abs as f64) as f64;
    println!("step r: {}", r_step);
    println!("step g: {}", g_step);
    println!("step b: {}", b_step);

    let mut stop = false;

    let rgb255 = 255.0;

    let mut spectrum: Vec<Rgb<u8>> = Vec::new();
    let forward = function_result(0.0, &function) != 1.0;
    println!("spectrum forward: {}", forward);

    for i in 0..max_dif_abs + 1 {
        let d: f64 = (i as f64 / max_dif) as f64;
        // optimized dif on interval <0, 1>
        // 0 -> 1, like circle up is forward
        // 1 -> 0, like circle down is backwards
        let v: f64 = function_result(d, &function);
        let value = v * max_dif_abs as f64;

        let mut r_new = r_from as f64 + (value * r_step);
        let mut g_new = g_from as f64 + (value * g_step);
        let mut b_new = b_from as f64 + (value * b_step);
        if i == 0 {
            println!("v: {}", v);
            println!("r_new={} = {} + ({} * {})", r_new, r_from, value, r_step);
        }

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

        let r_stop = if r_dif > 0 { (r_to as f64) < r_new } else { (r_to as f64) > r_new };
        let g_stop = if g_dif > 0 { (g_to as f64) < g_new } else { (g_to as f64) > g_new };
        let b_stop = if b_dif > 0 { (b_to as f64) < b_new } else { (b_to as f64) > b_new };

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

        // Add colours to Palette
        spectrum.push(Rgb([r_new as u8, g_new as u8, b_new as u8]));
        // println!("{} {} {}", r_new as u8, g_new as u8, b_new as u8);
        if stop {
            break;
        }
    }

    let spec_from;
    let spec_to;
    if forward {
        spec_from = spectrum.get(0).unwrap();
        spec_to = spectrum.get(spectrum.len() - 1).unwrap();
    } else {
        // functions (1 -> 0) like circle down
        spec_from = spectrum.get(spectrum.len() - 1).unwrap();
        spec_to = spectrum.get(0).unwrap();
    }
    let spec_r_from = spec_from.channels()[0] as u8;
    let spec_g_from = spec_from.channels()[1] as u8;
    let spec_b_from = spec_from.channels()[2] as u8;
    let spec_r_to = spec_to.channels()[0] as u8;
    let spec_g_to = spec_to.channels()[1] as u8;
    let spec_b_to = spec_to.channels()[2] as u8;
    // assert spectrum so that the coloring can be perfect
    assert_eq!(r_from, spec_r_from);
    assert_eq!(g_from, spec_g_from);
    assert_eq!(b_from, spec_b_from);
    assert_eq!(r_to, spec_r_to);
    assert_eq!(g_to, spec_g_to);
    assert_eq!(b_to, spec_b_to);
    println!("color spectrum certified");
    spectrum
}

// Calculates how much should colour in smooth colour palette change
// function : defines gradient of change from colour "from" (d=0) to colour "to" (d=1)
// d : 0 <= d <= 1
fn function_result(d: f64, function: &Function) -> f64 {
    match function {
        Function::Linear1 => d,
        Function::Linear3 => d * 3.0,
        Function::Linear7 => d * 7.0,
        Function::Quadratic => d * d,
        Function::Exp => d.exp() - 1.0,
        Function::Exp2 => (d * d).exp() - 1.0,
        Function::CircleDown => (1.0 - (d * d)).sqrt(),
        Function::CircleUp => 1.0 - (1.0 - (d * d)).sqrt(),
    }
}
