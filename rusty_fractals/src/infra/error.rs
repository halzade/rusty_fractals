use thiserror::Error;

#[derive(Error, Debug)]
pub enum FractalError {
    #[error("point ({re}, {im}) is outside the domain area")]
    PointOutOfBounds { re: f64, im: f64 },

    #[error("pixel coordinate ({x}, {y}) is outside the screen dimensions")]
    PixelOutOfBounds { x: usize, y: usize },
}
