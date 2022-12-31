use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_result::color_palette;
use rusty_fractals_result::color_palettes::PALETTE_BLACK_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::None;
use log::{info};

const NAME: &str = "Lotus";

struct Lotus {
    pub name: String,
}

impl Math for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");

    let lotus = Lotus { name: NAME.to_string() };
    let definition = FractalDefinition {
        iteration_min: 42,
        iteration_max: 8000,
        area_size: 9.5,
        target_re: 0.67748277351478,
        target_im: -1.18770078111202,
        resolution_width: 1920,
        resolution_height: 1080,
        resolution_multiplier: None,
        repeat: false,
        save_images: false,
        palette: PALETTE_BLACK_TO_WHITE,
    };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    info!("Fractal {}", lotus.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    lotus.math(&mut m, 1.0, 0.1);

    info!("Finished.");
}
