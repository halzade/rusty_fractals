pub struct FractalImagePixel {
    pixel_value: u32,
    px: u32,
    py: u32,
}

fn add(x: u32, y: u32) {
    elementsStaticFinebrot[x][y] += 1;
}

fn clear() {
    log.debug("clear");
    for y in 0..RESOLUTION_HEIGHT {
        for x in 0..RESOLUTION_WIDTH {
            elementsStaticFinebrot[x][y] = 0;
        }
    }
}

fn value_at(x: u32, y: u32) {
    return elementsStaticFinebrot[x][y];
}

fn best_four_chunks_value() -> u32 {
    log.debug("best_four_chunks_value()");
    let chunk_size_x = RESOLUTION_WIDTH / 20;
    let chunk_size_y = RESOLUTION_HEIGHT / 20;
    let mut values: Vec<u32> = Vec::new();
    for x in 0..20 {
        for y in 0..20 {
            values.push(chunkValue(
                x * chunk_size_x, (x + 1) * chunk_size_x,
                y * chunk_size_y, (y + 1) * chunk_size_y,
            ));
        }
    }
    values.sort().reverse();

    let mut sum = 0;
    for i in 0..4 {
        let v = values.get(i);
        sum += v;
    }
    log.debug("best_four_chunks_value() sum: " + sum);
    sum
}

fn chunk_value(x_from: i32, x_to: i32, y_from: i32, y_to: i32) -> u32 {
    let mut sum = 0;
    for x in x_from..x_to {
        for y in y_from..y_to {
            sum += elementsStaticFinebrot[x][y];
        }
    }
    sum
}