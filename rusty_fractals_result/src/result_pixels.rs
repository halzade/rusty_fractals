use log::{debug};

pub struct ResultPixels {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<u32>>,
}

impl ResultPixels {
    pub fn add(&mut self, x: usize, y: usize) {
        self.pixels[x][y] += 1;
    }

    pub fn clear(&mut self) {
        debug!("clear");
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[x][y] = 0;
            }
        }
    }

    pub fn value_at(&mut self, x: usize, y: usize) -> u32 {
        self.pixels[x][y]
    }

    pub fn best_four_chunks_value(&self) -> u32 {
        debug!("best_four_chunks_value()");
        let chunk_size_x = self.width / 20;
        let chunk_size_y = self.height / 20;
        let mut values: Vec<u32> = Vec::new();
        for x in 0..19 {
            for y in 0..19 {
                values.push(self.chunk_value(
                    x * chunk_size_x, (x + 1) * chunk_size_x,
                    y * chunk_size_y, (y + 1) * chunk_size_y,
                ));
            }
        }
        values.sort_by(|first, second| second.cmp(first));

        let mut sum = 0;
        for i in 0..4 {
            let v = values.get(i);
            match v {
                Some(v) => sum += v,
                None => panic!(),
            }
        }
        debug!("best_four_chunks_value() sum: {}", sum);
        sum
    }

    fn chunk_value(&self, x_from: usize, x_to: usize, y_from: usize, y_to: usize) -> u32 {
        let mut sum = 0;
        for x in x_from..x_to {
            for y in y_from..y_to {
                sum += self.pixels[x][y];
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