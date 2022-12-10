enum ResolutionMultiplier {
    /**
     * Single point at the center of Mandelbrot pixel
     */
    None,

    /**
     * One point at the center of the pixel and
     * Two opposite square corners altering each zoom iteration
     * center point remains the only relevant re,im point for state of Mandelbrot domain pixel (hibernated, active, active new, etc)
     */
    SquareAlter,

    /**
     * Fill each pixel wil NxN points.
     * The center point is in the middle of a pixel, use odd numbers to fill pixels perfectly
     */
    Square3,
    Square5,
    Square11,
    Square51,
    Square101,
}
