use palettes::Function;
use rgb::RGB;
use pixel_states::DomainElementState;
use crate::palettes;

/**
 * Colors for Mandelbrot image based on Mandelbrot element's state
 */
fn color_for_state(el: DomainElement) {
    match el.state() {
        /* most of the elements are going to be */
        DomainElementState::FinishedSuccessPast => FINISHED_SUCCESS_PAST,
        DomainElementState::HibernatedDeepBlack => HIBERNATED_DEEP_BLACK,
        DomainElementState::GoodPath => GOOD_PATH,
        DomainElementState::ActiveNew => ACTIVE_NEW,
        DomainElementState::FinishedSuccess => FINISHED_SUCCESS,
        DomainElementState::FinishedTooShort => FINISHED_TOO_SHORT,
        DomainElementState::FinishedTooLong => FINISHED_TOO_LONG
    }

    fn max(r: u8, g: u8, b: u8) -> u8 {
        if a(r) >= a(g) && a(r) >= a(b) {
            r
        } else if a(g) >= a(r) && a(g) >= a(b) {
            g
        } else if a(b) >= a(r) && a(b) >= a(g) {
            b
        }
    }

    fn a(v: u8) -> u8 {
        v.abs()
    }

    /**
     * Fill color spectrum with colors between colors:
     * @param from     color for lower values
     * @param to       color for higher values
     * @param function defines gradient of color change
     */
    fn to_palette(mut spectrum: Vec<RGB<u8>>, function: Function, from: RGB<u8>, to: RGB<u8>) {
        let r_from = from.r;
        let g_from = from.g;
        let b_from = from.b;

        let r_dif = to.r - r_from;
        let g_dif = to.g - g_from;
        let b_dif = to.b - b_from;

        let max_dif = a(max(r_dif, g_dif, b_dif));

        let r_step: f64 = (r_dif / max_dif) as f64;
        let g_step: f64 = (g_dif / max_dif) as f64;
        let b_step: f64 = (b_dif / max_dif) as f64;

        let mut stop = false;

        let tr = to.r;
        let tg = to.g;
        let tb = to.b;

        let rgb255 = 255;

        for i in 0..a(max_dif) {
            let d: f64 = (i / max_dif) as f64;
            let v: f64 = function_result(d, &function);
            let value = v * max_dif;

            let rr = r_from + (value * r_step);
            let gg = g_from + (value * g_step);
            let bb = b_from + (value * b_step);

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
            spectrum.push(RGB: new(rr, gg, bb));

            if stop {
                break;
            }
        }
    }
}

/**
 * Calculates how much should color in smooth color palette change
 *
 * @param d        : 0 <= d <= 1
 * @param function defines gradient of change from color "from" (d=0) to color "to" (d=1)
 */
fn function_result(d: f64, function: &Function) -> f64 {
    match function {
        Function::linear1 => d,
        Function::linear3 => d * 3,
        Function::linear7 => d * 7,
        Function::quadratic => d * d,
        Function::q3 => d * d * d,
        Function::q4 => d * d * d * d,
        Function::q5 => d * d * d * d * d,
        Function::exp => d.exp() - 1,
        Function::exp2 => (d * d).exp() - 1,
        Function::circleDown => (1 - (d * d)).sqrt(),
        Function::circleUp => 1 - (1 - (d * d)).sqrt(),
    }
}
