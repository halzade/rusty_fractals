pub struct ResultPixels {
    pub width: usize,
    pub height: useize,
    pub pixels: Vec<Vec<u32>>,
}

impl ResultPixels {
    pub fn add(x: u32, y: u32) {
        pixels[x][y] += 1;
    }

    pub fn clear() {
        log.debug("clear");
        for y in 0..RESOLUTION_HEIGHT {
            for x in 0..RESOLUTION_WIDTH {
                pixels[x][y] = 0;
            }
        }
    }

    pub fn value_at(x: u32, y: u32) {
        return pixels[x][y];
    }

    pub fn best_four_chunks_value(&self) -> u32 {
        log.debug("best_four_chunks_value()");
        let chunk_size_x = width / 20;
        let chunk_size_y = height / 20;
        let mut values: Vec<u32> = Vec::new();
        for x in 0..19 {
            for y in 0..19 {
                values.push(self.chunk_value(
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
                sum += pixels[x][y];
            }
        }
        sum
    }
}

pub fn init(width: usize, height: usize) -> ResultPixels {
    let mut vx = Vec::new();
    for _ in 0..width - 1 {
        let mut vy = Vec::new();
        for _ in 0..height - 1 {
            vy.push(0);
        }
        vx.push(vy);
    }
    ResultPixels {
        width,
        height,
        pixels: vx,
    }
}