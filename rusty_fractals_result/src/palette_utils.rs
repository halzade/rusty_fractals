use image::{Pixel, Rgb};
use palettes::Function;

use crate::palettes;

fn max(r: i8, g: i8, b: i8) -> i8 {
    if a(r) >= a(g) && a(r) >= a(b) {
        return r;
    } else if a(g) >= a(r) && a(g) >= a(b) {
        return g;
    } else if a(b) >= a(r) && a(b) >= a(g) {
        return b;
    }
    panic!()
}

fn a(v: i8) -> u8 {
    v.abs() as u8
}

// Fill color spectrum with colors between colors:
// from     : color for lower values
// to       : color for higher values
// function : defines gradient of color change
pub fn make_spectrum(function: Function, mut from: Rgb<u8>, to: Rgb<u8>) -> Vec<Rgb<u8>> {
    let r_from = from.channels()[0];
    let g_from = from.channels()[1];
    let b_from = from.channels()[2];

    let tr = to.channels()[0];
    let tg = to.channels()[1];
    let tb = to.channels()[2];

    let r_dif = tr as i8 - r_from as i8;
    let g_dif = tg as i8 - g_from as i8;
    let b_dif = tb as i8 - b_from as i8;

    let max_dif = a(max(r_dif, g_dif, b_dif)) as f64;

    let r_step: f64 = (r_dif as f64 / max_dif) as f64;
    let g_step: f64 = (g_dif as f64 / max_dif) as f64;
    let b_step: f64 = (b_dif as f64 / max_dif) as f64;

    let mut stop = false;

    let rgb255 = 255;

    let mut spectrum: Vec<Rgb<u8>> = Vec::new();

    for i in 0..a(max_dif as i8) {
        let d: f64 = (i as f64 / max_dif) as f64;
        // optimized dif on interval <0, 1>
        let v: f64 = function_result(d, &function);
        let value = v * max_dif;

        let mut rr = r_from + (value * r_step) as u8;
        let mut gg = g_from + (value * g_step) as u8;
        let mut bb = b_from + (value * b_step) as u8;

        if rr > rgb255 {
            rr = rgb255;
            stop = true;
        }
        if gg > rgb255 {
            gg = rgb255;
            stop = true;
        }
        if bb > rgb255 {
            bb = rgb255;
            stop = true;
        }

        if rr < 0 {
            rr = 0;
            stop = true;
        }
        if gg < 0 {
            gg = 0;
            stop = true;
        }
        if bb < 0 {
            bb = 0;
            stop = true;
        }

        let r_stop = if r_dif > 0 {
            tr < rr
        } else {
            tr > rr
        };
        let g_stop = if g_dif > 0 {
            tg < gg
        } else {
            tg > gg
        };
        let b_stop = if b_dif > 0 {
            tb < bb
        } else {
            tb > bb
        };

        if r_stop {
            rr = tr;
            stop = true;
        }
        if g_stop {
            gg = tg;
            stop = true;
        }
        if b_stop {
            bb = tb;
            stop = true;
        }

        // Add colors to Palette
        spectrum.push(Rgb([rr, gg, bb]));

        if stop {
            break;
        }
    }
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
        Function::CircleDown => (1.0 - (d * d)).sqrt(),
        Function::CircleUp => 1.0 - (1.0 - (d * d)).sqrt(),
    }
}
