pub enum ResolutionMultiplier {
    // Single point at the center of Mandelbrot pixel
    None,

    // One point at the center of the pixel and two opposite square corners, altering each zoom iteration
    // The center point remains the only relevant re,im point for the state of domain pixel (hibernated, active, active new, etc)
    SquareAlter,

    // Fill each pixel wil matrix NxN points. Use odd numbers to fill pixels perfectly
    Square3,
    Square5,
    Square11,
    Square51,
    Square101,
}
