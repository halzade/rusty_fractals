pub struct FractalImagePixel {
    pixel_value: u32,
    px: u32,
    py: u32,
}

fn add(int x, int y) {
    elementsStaticFinebrot[x][y] += 1;
}

fn clear() {
    log.debug("clear");
    for (int y = 0; y < RESOLUTION_HEIGHT; y+ +) {
        for (int x = 0; x < RESOLUTION_WIDTH; x+ +) {
            elementsStaticFinebrot[x][y] = 0;
        }
    }
}

fn value_at(int x, int y) {
    return elementsStaticFinebrot[x][y];
}

fn best_four_chunks_value() {
    log.debug("best_four_chunks_value()");
    final int
    chunkSizeX = RESOLUTION_WIDTH / 20;
    final int
    chunkSizeY = RESOLUTION_HEIGHT / 20;
    final ArrayList < Integer > values = new
    ArrayList < > (20 * 20);
    for (int x = 0; x < 20; x+ +) {
        for (int y = 0; y < 20; y+ +) {
            values.add(chunkValue(
                x * chunkSizeX, (x + 1) * chunkSizeX,
                y * chunkSizeY, (y + 1) * chunkSizeY,
            )
            );
        }
    }
    values.sort(Collections.reverseOrder());

    int
    sum = 0;
    for (int i = 0; i < 4; i+ +) {
        int
        v = values.get(i);
        sum += v;
    }
    log.debug("best_four_chunks_value() sum: " + sum);
    return sum;
}

fn chunk_value(int xFrom, int xTo, int yFrom, int yTo) -> u32 {
    int
    sum = 0;
    for (int x = xFrom; x < xTo; x+ +) {
        for (int y = yFrom; y < yTo; y+ +) {
            sum += elementsStaticFinebrot[x][y];
        }
    }
    return sum;
}