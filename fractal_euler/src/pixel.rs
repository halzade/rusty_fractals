private final int[][] elementsStaticScreenRed;
private final int[][] elementsStaticScreenGreen;
private final int[][] elementsStaticScreenBlue;

public PixelsEulerFinebrotImpl() {
log.debug("constructor");
log.debug("[" + RESOLUTION_WIDTH + "][" + RESOLUTION_HEIGHT + "]");
this.elementsStaticScreenRed = new int[RESOLUTION_WIDTH][RESOLUTION_HEIGHT];
this.elementsStaticScreenGreen = new int[RESOLUTION_WIDTH][RESOLUTION_HEIGHT];
this.elementsStaticScreenBlue = new int[RESOLUTION_WIDTH][RESOLUTION_HEIGHT];
}

fn add(int x, int y, Spectra spec) {
    switch(spec)
    {
        case
        red -> elementsStaticScreenRed[x][y] += 1;
        case
        green -> elementsStaticScreenGreen[x][y] += 1;
        case
        blue -> elementsStaticScreenBlue[x][y] += 1;
    }
}

fn clear() {
    log.debug("clear()");
    for (int y = 0; y < RESOLUTION_HEIGHT; y+ +) {
        for (int x = 0; x < RESOLUTION_WIDTH; x+ +) {
            elementsStaticScreenRed[x][y] = 0;
            elementsStaticScreenGreen[x][y] = 0;
            elementsStaticScreenBlue[x][y] = 0;
        }
    }
}

fn value_at(int x, int y, Spectra spec) {
    switch(spec)
    {
        case
        red -> {
        return elementsStaticScreenRed[x][y];
    }
        case
        green -> {
        return elementsStaticScreenGreen[x][y];
    }
        case
        blue -> {
        return elementsStaticScreenBlue[x][y];
    }
        default -> throw
        new
        RuntimeException("unknown spectra");
    }
}

fn set(int x, int y, Spectra spec, int colorValue) {
    switch(spec)
    {
        case
        red -> elementsStaticScreenRed[x][y] = colorValue;
        case
        green -> elementsStaticScreenGreen[x][y] = colorValue;
        case
        blue -> elementsStaticScreenBlue[x][y] = colorValue;
    }
}

/* For simplicity Euler Fractal uses only three explicitly defined spectra */
pub enum Spectra { Red, Green, Blue }
}