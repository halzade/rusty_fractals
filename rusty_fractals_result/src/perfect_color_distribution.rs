/**
 * Method used for perfect coloring is
 * - Gather all screen pixels and order them by value
 * - Count how many pixels should be colored by each color from spectrum
 * - Zero elements and noise color by the lowest color
 * - Color all significant pixels ordered by value
 */
PerfectColorDistribution


protected final Comparator<FinebrotPixel> comparator = Comparator.comparingInt(FinebrotPixel::pixelValue);

protected final Comparator<MandelbrotPixel> comparatorMandelbrot = (a, b) -> {
int c = compare(a.pixelValue, b.pixelValue);
if (c == 0) {
return compare(a.qiad, b.qiad);
}
return c;
};

protected final Comparator<MandelbrotPixel> comparatorMandelbrotZero = Comparator.comparingDouble(MandelbrotPixel::quad);

/**
 * Finebrot pixels, order by value
 */
static final List<FinebrotPixel> pixels = new ArrayList< > ();


fn perfectlyColorValues() {
    int
    zeroValueElements = 0;

    /* read screen values */
    for (int y = 0; y < RESOLUTION_HEIGHT; y+ +) {
        for (int x = 0; x < RESOLUTION_WIDTH; x+ +) {
            int
            v = PixelsFinebrot.valueAt(x, y);
            if (v <= coloringThreshold) {
                zeroValueElements + +;
            }
            pixels.add(new FinebrotPixel(v, x, y));
        }
    }

    /*
     *  order pixels from the smallest to the highest value
     */
    pixels.sort(comparator);

    final int
    allPixelsTotal = RESOLUTION_WIDTH * RESOLUTION_HEIGHT;
    final int
    allPixelsNonZero = allPixelsTotal - zeroValueElements;
    final int
    paletteColorCount = Palette.colorResolution();
    final int
    singleColorUse = ((int)((double) allPixelsNonZero / (double) paletteColorCount));
    final int
    left = allPixelsNonZero - (paletteColorCount * singleColorUse);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + allPixelsTotal);
    log.debug("--------------------------->" + (zeroValueElements + left + (singleColorUse * paletteColorCount)));
    log.debug("Zero value pixels to paint: " + zeroValueElements);
    log.debug("Non zero pixels to paint:   " + allPixelsNonZero);
    log.debug("Spectrum, available colors: " + paletteColorCount);
    log.debug("Pixels per each color:      " + singleColorUse);
    log.debug("left:                       " + left);
    log.debug("------------------------------------");

    /* pixel index */
    int
    pi;
    FinebrotPixel
    sp;

    /* paint mismatched pixel amount with the least value colour */
    for (pi = 0; pi < left + zeroValueElements; pi+ +) {
        sp = pixels.get(pi);
        FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(0).getRGB());
    }

    /* color all remaining pixels, these are order by value */
    for (int paletteColourIndex = 0; paletteColourIndex < paletteColorCount; paletteColourIndex+ +) {
        for (int ci = 0; ci < singleColorUse; ci+ +) {
            /* color all these pixels with same color */
            sp = pixels.get(pi + +);
            if (sp.pixelValue() <= coloringThreshold) {
                /* color zero-value elements and low-value-noise with the darkest color */
                FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(0).getRGB());
            } else {
                /* perfect-color all significant pixels */
                FinebrotImage.setRGB(sp.px(), sp.py(), Palette.getSpectrumValue(paletteColourIndex).getRGB());
            }
        }
    }
    log.debug("painted:                   " + pi);

    /*
     * Behold, the coloring is perfect
     */

    log.debug("clear pixels");
    pixels.clear();
}




/**
 * Finebrot pixels, order by value
 */
static final List<FinebrotPixel> pixelsRed = new ArrayList< > ();
static final List<FinebrotPixel> pixelsGreen = new ArrayList< > ();
static final List<FinebrotPixel> pixelsBlue = new ArrayList< > ();

fn perfectly_color_values_euler() {
    int
    zeroValueElementsRed = 0;
    int
    zeroValueElementsGreen = 0;
    int
    zeroValueElementsBlue = 0;

    /* identify zero and low-value elements as zero or noise */
    final int
    threshold = 1;

    /* read screen values */
    for (int y = 0; y < RESOLUTION_HEIGHT; y+ +) {
        for (int x = 0; x < RESOLUTION_WIDTH; x+ +) {
            int
            r = PixelsEulerFinebrot.valueAt(x, y, red);
            int
            g = PixelsEulerFinebrot.valueAt(x, y, green);
            int
            b = PixelsEulerFinebrot.valueAt(x, y, blue);
            if (r <= threshold) {
                zeroValueElementsRed + +;
            }
            if (g <= threshold) {
                zeroValueElementsGreen + +;
            }
            if (b <= threshold) {
                zeroValueElementsBlue + +;
            }
            pixelsRed.add(new FinebrotPixel(r, x, y));
            pixelsGreen.add(new FinebrotPixel(g, x, y));
            pixelsBlue.add(new FinebrotPixel(b, x, y));
        }
    }

    /*
     *  order pixels from the smallest to the highest value
     */
    pixelsRed.sort(comparator);
    pixelsGreen.sort(comparator);
    pixelsBlue.sort(comparator);

    final int
    allPixelsTotal = RESOLUTION_WIDTH * RESOLUTION_HEIGHT;
    final int
    allPixelsNonZeroRed = allPixelsTotal - zeroValueElementsRed;
    final int
    allPixelsNonZeroGreen = allPixelsTotal - zeroValueElementsGreen;
    final int
    allPixelsNonZeroBlue = allPixelsTotal - zeroValueElementsBlue;
    final int
    paletteColorCount = PaletteEuler3.colorResolution(); // same
    final int
    singleColorUseRed = ((int)((double) allPixelsNonZeroRed / (double) paletteColorCount));
    final int
    singleColorUseGreen = ((int)((double) allPixelsNonZeroGreen / (double) paletteColorCount));
    final int
    singleColorUseBlue = ((int)((double) allPixelsNonZeroBlue / (double) paletteColorCount));
    final int
    leftRed = allPixelsNonZeroRed - (paletteColorCount * singleColorUseRed);
    final int
    leftGreen = allPixelsNonZeroGreen - (paletteColorCount * singleColorUseGreen);
    final int
    leftBlue = allPixelsNonZeroBlue - (paletteColorCount * singleColorUseBlue);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + allPixelsTotal);
    log.debug("--------------------------->" + (zeroValueElementsRed + leftRed + (singleColorUseRed * paletteColorCount)));
    log.debug("--------------------------->" + (zeroValueElementsGreen + leftGreen + (singleColorUseGreen * paletteColorCount)));
    log.debug("--------------------------->" + (zeroValueElementsBlue + leftBlue + (singleColorUseBlue * paletteColorCount)));
    log.debug("Zero value pixels to paint: " + zeroValueElementsRed);
    log.debug("Zero value pixels to paint: " + zeroValueElementsGreen);
    log.debug("Zero value pixels to paint: " + zeroValueElementsBlue);
    log.debug("Non zero pixels to paint:   " + allPixelsNonZeroRed);
    log.debug("Non zero pixels to paint:   " + allPixelsNonZeroGreen);
    log.debug("Non zero pixels to paint:   " + allPixelsNonZeroBlue);
    log.debug("Spectrum, available colors: " + paletteColorCount);
    log.debug("Pixels per each color:      " + singleColorUseRed);
    log.debug("Pixels per each color:      " + singleColorUseGreen);
    log.debug("Pixels per each color:      " + singleColorUseBlue);
    log.debug("left:                       " + leftRed);
    log.debug("left:                       " + leftGreen);
    log.debug("left:                       " + leftBlue);
    log.debug("------------------------------------");

    /* pixel index */
    int
    piRed;
    FinebrotPixel
    sp;
    /* paint mismatched pixel amount with the least value colour */
    for (piRed = 0; piRed < leftRed + zeroValueElementsRed; piRed+ +) {
        sp = pixelsRed.get(piRed);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), red, 0);
    }
    /* color all remaining pixels, these are order by value */
    for (int paletteColourIndex = 0; paletteColourIndex < paletteColorCount; paletteColourIndex+ +) {
        for (int ci = 0; ci < singleColorUseRed; ci+ +) {
            /* color all these pixels with same color */
            sp = pixelsRed.get(piRed + +);
            if (sp.pixelValue() <= threshold) {
                PixelsEulerFinebrot.set(sp.px(), sp.py(), red, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), red, PaletteEuler3.getSpectrumValueRed(paletteColourIndex).getRed());
            }
        }
    }

    int
    piGreen;
    for (piGreen = 0; piGreen < leftGreen + zeroValueElementsGreen; piGreen+ +) {
        sp = pixelsGreen.get(piGreen);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), green, 0);
    }
    /* color all remaining pixels, these are order by value */
    for (int paletteColourIndex = 0; paletteColourIndex < paletteColorCount; paletteColourIndex+ +) {
        for (int ci = 0; ci < singleColorUseGreen; ci+ +) {
            /* color all these pixels with same color */
            sp = pixelsGreen.get(piGreen + +);
            if (sp.pixelValue() <= threshold) {
                /* color zero-value elements and low-value-noise with the darkest color */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), green, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), green, PaletteEuler3.getSpectrumValueGreen(paletteColourIndex).getGreen());
            }
        }
    }

    int
    piBlue;
    for (piBlue = 0; piBlue < leftBlue + zeroValueElementsBlue; piBlue+ +) {
        sp = pixelsBlue.get(piBlue);
        PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, 0);
    }
    /* color all remaining pixels, these are order by value */
    for (int paletteColourIndex = 0; paletteColourIndex < paletteColorCount; paletteColourIndex+ +) {
        for (int ci = 0; ci < singleColorUseBlue; ci+ +) {
            /* color all these pixels with same color */
            sp = pixelsBlue.get(piBlue + +);
            if (sp.pixelValue() <= threshold) {
                /* color zero-value elements and low-value-noise with the darkest color */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, 0);
            } else {
                /* perfect-color all significant pixels */
                PixelsEulerFinebrot.set(sp.px(), sp.py(), blue, PaletteEuler3.getSpectrumValueBlue(paletteColourIndex).getBlue());
            }
        }
    }

    log.debug("painted:                   " + piRed);
    log.debug("painted:                   " + piGreen);
    log.debug("painted:                   " + piBlue);

    /*
     * read 3 screen colors
     * write image colors
     */
    for (int y = 0; y < RESOLUTION_HEIGHT; y+ +) {
        for (int x = 0; x < RESOLUTION_WIDTH; x+ +) {
            int
            r = PixelsEulerFinebrot.valueAt(x, y, red);
            int
            g = PixelsEulerFinebrot.valueAt(x, y, green);
            int
            b = PixelsEulerFinebrot.valueAt(x, y, blue);
            FinebrotImage.setRGB(x, y, new Color(r, g, b).getRGB());
        }
    }

    /*
     * Behold, the coloring is perfect
     */

    log.debug("clear pixels");
    pixelsRed.clear();
    pixelsGreen.clear();
    pixelsBlue.clear();
    PixelsEulerFinebrot.clear();
}


/**
 * Mandelbrot pixels, order by value
 */
static final List<MandelbrotPixel> pixels = new ArrayList<>();
static final List<MandelbrotPixel> pixelsZero = new ArrayList<>();

const NEIGHBOR_COORDINATES = new ArrayList<>(List.of(
/* up */
new int[]{-1, -1},
new int[]{0, -1},
new int[]{1, -1},
/* left */
new int[]{-1, 0},
/* right */
new int[]{1, 0},
/* down */
new int[]{-1, 1},
new int[]{0, 1},
new int[]{1, 1}
));
private final MandelbrotPixel[][] field = new MandelbrotPixel[RESOLUTION_WIDTH][RESOLUTION_HEIGHT];


fn perfectly_color_values_mandelbrot() {
log.debug("perfectlyColorValues()");

int zeroValueElements = 0;

/* read screen values */

for (int y = 0; y < RESOLUTION_HEIGHT; y++) {
for (int x = 0; x < RESOLUTION_WIDTH; x++) {
final MandelbrotElement el = PixelsMandelbrot.elAt(x, y);
if (el.value == 0) {
zeroValueElements++;
pixelsZero.add(MandelbrotPixelFactory.make(el, x, y));
} else {
MandelbrotPixel mp = MandelbrotPixelFactory.make(el, x, y);
pixels.add(mp);
field[x][y] = mp;
}
}
}

/*
 *  order pixels from the smallest to the highest value
 */
pixels.sort(comparatorMandelbrot);
pixelsZero.sort(comparatorMandelbrotZero);

final int allPixelsTotal = RESOLUTION_WIDTH * RESOLUTION_HEIGHT;
int allPixelsNonZero = allPixelsTotal - zeroValueElements;
int paletteColorCount = Palette.colorResolution();
int singleColorUse = ((int) ((double) allPixelsNonZero / (double) paletteColorCount));

final int left = allPixelsNonZero - (paletteColorCount * singleColorUse);

log.debug("------------------------------------");
log.debug("All pixels to paint:        " + allPixelsTotal);
log.debug("--------------------------->" + (zeroValueElements + left + (singleColorUse * paletteColorCount)));
log.debug("Zero value pixels to paint: " + zeroValueElements);
log.debug("Non zero pixels to paint:   " + allPixelsNonZero);
log.debug("Spectrum, available colors:>" + paletteColorCount);
log.debug("Pixels per each color:      " + singleColorUse);
log.debug("left:                       " + left);
log.debug("------------------------------------");


MandelbrotPixel mp;
int pi = 0;

/* paint mismatched pixel amount with the least but not the lowest value colour */
while (pi < left) {
mp = pixels.get(pi++);
MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(0).getRGB());
}

int paletteColourIndex = 0;
while (paletteColourIndex < paletteColorCount) {
for (int ci = 0; ci < singleColorUse; ci++) {
mp = pixels.get(pi++);
mp.colorValue(paletteColourIndex);
MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(paletteColourIndex).getRGB());
}
paletteColourIndex++;
}

Assert.assertEquals(pixels.size(), pi);

/*
 * Fix black dots caused by quad inverse imperfection
 * Keep incorrect qud results ~
 */

for (MandelbrotPixel mpp : pixels) {
final int averageColourIndex = acIfBlackDot(mpp);
if (averageColourIndex != -1) {
mpp.colorValue(averageColourIndex);
MandelbrotImage.setRGB(mpp.px, mpp.py, Palette.getSpectrumValue(averageColourIndex).getRGB());
}
}

/*
 * PAINT INSIDES OF MANDELBROT SET
 */

final int zeroPaletteColorCount = PaletteZero.colorResolution();
final int zeroSingleColorUse = ((int) ((double) zeroValueElements / (double) zeroPaletteColorCount));
final int zeroLeft = zeroValueElements - (zeroPaletteColorCount * zeroSingleColorUse);

log.info("zeroPaletteColorCount:    > " + zeroPaletteColorCount);
log.info("zeroSingleColorUse:       > " + zeroSingleColorUse);
log.info("zeroLeft:                 > " + zeroLeft);

int piz;
for (piz = 0; piz < zeroLeft; piz++) {
mp = pixelsZero.get(piz);
MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(0).getRGB());
}
for (int zeroPaletteColourIndex = 0; zeroPaletteColourIndex < zeroPaletteColorCount; zeroPaletteColourIndex++) {
for (int ci = 0; ci < zeroSingleColorUse; ci++) {
/* color all these pixels with same color */
mp = pixelsZero.get(piz++);
MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(zeroPaletteColourIndex).getRGB());
}
}

log.debug("painted:                   " + pi);

/*
 * Behold, the coloring is perfect
 */

log.debug("clear pixels");
pixels.clear();
pixelsZero.clear();
}

/**
 * Return average color of neighbour elements
 */
private int acIfBlackDot(MandelbrotPixel mp) {
final int pv = mp.pixelValue;
int sum = 0;
double neighbours = 0;
for (int[] c : NEIGHBOR_COORDINATES) {
final int a = mp.px + c[0];
final int b = mp.py + c[1];
final MandelbrotPixel n = checkDomain(a, b);
if (n != null) {
if (Math.abs(pv - n.pixelValue) > 2) {
/* verify only one value difference gradient */
return -1;
}
sum += n.colorValue;
neighbours++;
} else {
/* don't fix elements of edges */
return -1;
}
}

final int cv = mp.colorValue;
final int averageValue = (int) (sum / neighbours);

if (cv < averageValue - 5) {
/* darker */
return averageValue;
}
return -1;
}