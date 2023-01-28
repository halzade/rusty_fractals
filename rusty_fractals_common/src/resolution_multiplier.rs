#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum ResolutionMultiplier {
    // Single point at the center of Mandelbrot pixel
    None,

    // One point at the center of the pixel and four square corners
    Square2,

    // Fill each pixel wil matrix NxN points. Use odd numbers to fill pixels perfectly
    Square3,
    Square5,
    Square11,
    Square51,
    Square101,
}
