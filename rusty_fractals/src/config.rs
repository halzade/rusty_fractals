use crate::fractal::FractalCalculationType::{DynamicSequenceNebula, StaticImageMandelbrot, StaticImageNebula, StaticSequenceMandelbrot, StaticSpectralImageEuler};
use crate::fractal::OrbitType::Ignore;
use crate::fractal::{FractalConfig, OrbitType};
use crate::palettes::PaletteName;
use crate::resolution_multiplier::ResolutionMultiplier;
use crate::resolution_multiplier::ResolutionMultiplier::Single;

pub struct NebulaImage {
    // fractal config
    pub name: &'static str,
    pub orbits: OrbitType,
    // calculation config
    pub iteration_min: u64,
    pub iteration_max: u64,
    pub resolution_multiplier: ResolutionMultiplier,
    pub palette: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
}
pub struct NebulaVideo {
    // fractal config
    pub name: &'static str,
    pub orbits: OrbitType,
    // calculation config
    pub iteration_min: u64,
    pub iteration_max: u64,
    pub resolution_multiplier: ResolutionMultiplier,
    pub palette: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
    // calculation update config
    pub update_max: u64,
    pub update_min: u64,
}

pub struct MandelbrotImage {
    // fractal config
    pub name: &'static str,
    // calculation config
    pub iteration_max: u64,
    pub palette: PaletteName,
    pub palette_zero: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
}

pub struct MandelbrotVideo {
    // fractal config
    pub name: &'static str,
    // calculation config
    pub iteration_max: u64,
    pub palette: PaletteName,
    pub palette_zero: PaletteName,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
}

pub struct EulerImage {
    // fractal config
    pub name: &'static str,
    pub orbits: OrbitType,
    // calculation config
    pub iteration_min: u64,
    pub iteration_max: u64,
    pub resolution_multiplier: ResolutionMultiplier,
    // area config
    pub width_x: usize,
    pub height_y: usize,
    pub width_re: f64,
    pub center_re: f64,
    pub center_im: f64,
}

impl NebulaImage {
    pub fn init(&self) -> FractalConfig {
        FractalConfig {
            name: self.name,
            orbits: self.orbits,
            fractal_calc_type: StaticImageNebula,
            iteration_min: self.iteration_min,
            iteration_max: self.iteration_max,
            resolution_multiplier: self.resolution_multiplier,
            palette: self.palette,
            palette_zero: PaletteName::Nothing,
            width_xl: self.width_x,
            width_xp: self.width_x + 1, // for x = 1, two borders left and right
            height_yl: self.height_y,
            height_yp: self.height_y + 1,
            width_re: self.width_re,
            center_re: self.center_re,
            center_im: self.center_im,
            update_max: 1,
            update_min: 0,
        }
    }
}

impl NebulaVideo {
    pub fn init(&self) -> FractalConfig {
        FractalConfig {
            name: self.name,
            orbits: self.orbits,
            fractal_calc_type: DynamicSequenceNebula,
            iteration_min: self.iteration_min,
            iteration_max: self.iteration_max,
            resolution_multiplier: self.resolution_multiplier,
            palette: self.palette,
            palette_zero: PaletteName::Nothing,
            width_xl: self.width_x,
            width_xp: self.width_x + 1,
            height_yl: self.height_y,
            height_yp: self.height_y + 1,
            width_re: self.width_re,
            center_re: self.center_re,
            center_im: self.center_im,
            update_max: self.update_max,
            update_min: self.update_min,
        }
    }
}

impl MandelbrotImage {
    pub fn init(&self) -> FractalConfig {
        FractalConfig {
            name: self.name,
            orbits: Ignore,
            fractal_calc_type: StaticImageMandelbrot,
            iteration_min: 0,
            iteration_max: self.iteration_max,
            resolution_multiplier: Single,
            palette: self.palette,
            palette_zero: self.palette_zero,
            width_xl: self.width_x,
            width_xp: self.width_x + 1,
            height_yl: self.height_y,
            height_yp: self.height_y + 1,
            width_re: self.width_re,
            center_re: self.center_re,
            center_im: self.center_im,
            update_max: 1,
            update_min: 0,
        }
    }
}

impl MandelbrotVideo {
    pub fn init(&self) -> FractalConfig {
        FractalConfig {
            name: self.name,
            orbits: Ignore,
            fractal_calc_type: StaticSequenceMandelbrot,
            iteration_min: 0,
            iteration_max: self.iteration_max,
            resolution_multiplier: Single,
            palette: self.palette,
            palette_zero: self.palette_zero,
            width_xl: self.width_x,
            width_xp: self.width_x + 1,
            height_yl: self.height_y,
            height_yp: self.height_y + 1,
            width_re: self.width_re,
            center_re: self.center_re,
            center_im: self.center_im,
            update_max: 1,
            update_min: 0,
        }
    }
}

impl EulerImage {
    pub fn init(&self) -> FractalConfig {
        FractalConfig {
            name: self.name,
            orbits: self.orbits,
            fractal_calc_type: StaticSpectralImageEuler,
            iteration_min: self.iteration_min,
            iteration_max: self.iteration_max,
            resolution_multiplier: self.resolution_multiplier,
            palette: PaletteName::Nothing,
            palette_zero: PaletteName::Nothing,
            width_xl: self.width_x,
            width_xp: self.width_x + 1,
            height_yl: self.height_y,
            height_yp: self.height_y + 1,
            width_re: self.width_re,
            center_re: self.center_re,
            center_im: self.center_im,
            update_max: 1,
            update_min: 0,
        }
    }
}
