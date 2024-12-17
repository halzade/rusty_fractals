use crate::window;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::data_image::DataType::Static;
use rusty_fractals_common::data_px::{active_new, hibernated_deep_black};
use rusty_fractals_common::fractal::{FractalConfig, MandelbrotConfig};
use rusty_fractals_common::fractal_log::now;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::{area, data_image, palettes};

/**
 * Application is used to manage repeated calculation during zoom
 */
pub struct Application<'lt> {
    pub data_image: DataImage<'lt>,
    pub width: usize,
    pub height: usize,
    pub area: Area<'lt>,
    pub min: u32,
    pub max: u32,
    pub palette: Palette<'lt>,
    //  mandelbrot specific
    pub palette_zero: Palette<'lt>,
    //  nebula specific
    pub resolution_multiplier: ResolutionMultiplier,
}

impl<'lt> Application<'lt> {
    pub fn move_target(&self, x: usize, y: usize) {
        self.area.move_target(x, y);
    }

    pub fn zoom_in_recalculate_pixel_positions(&self, is_mandelbrot: bool) {
        self.area.zoom_in();
        window::paint_image_calculation_progress(&self.data_image);

        self.recalculate_pixels_positions_for_next_calculation(is_mandelbrot);
        window::paint_image_calculation_progress(&self.data_image);
    }

    pub fn zoom_in(&self) {
        self.area.zoom_in();
    }

    // This is called after calculation finished, a zoom-in was called and new area measures recalculated
    pub fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        println!("recalculate_pixels_positions_for_next_calculation()");
        // Scan all elements : old positions from previous calculation
        // Some elements will be moved to new positions
        // For all the moved elements, subsequent calculations will be skipped.
        let area = &self.area;
        let (cx, cy) = area.point_to_pixel(
            area.data.lock().unwrap().center_re,
            area.data.lock().unwrap().center_im,
        );
        now("1. move top left to center");
        for y in 0..cy {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("2. move top right to center");
        for y in 0..cy {
            for x in (cx..self.width).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("3. move bottom left to center");
        for y in (cy..self.height).rev() {
            for x in 0..cx {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        now("4. move bottom right to center");
        for y in (cy..self.height).rev() {
            for x in (cx..self.width).rev() {
                self.data_image.move_to_new_position(x, y, area);
            }
        }
        // Create new elements on positions where no px moved to
        now("fill empty places");
        let mut c_moved = 0;
        let mut c_created = 0;

        let res = area.screen_to_domain_re_copy();
        let ims = area.screen_to_domain_im_copy();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut mo_px = self.data_image.mo_px_at(x, y);
                if mo_px.is_none() {
                    c_created += 1;

                    let re = res[x];
                    let im = ims[y];

                    if self
                        .data_image
                        .all_neighbors_finished_bad(x, y, is_mandelbrot)
                    {
                        // Calculation for some positions should be skipped as they are too far away form any long successful divergent position
                        mo_px.replace(hibernated_deep_black(re, im));
                    } else {
                        mo_px.replace(active_new(re, im));
                    }
                } else {
                    c_moved += 1;
                }
            }
        }
        println!("moved:     {}", c_moved);
        println!("created:   {}", c_created);
        assert!(c_moved > 0);
        assert!(c_created > 0);
    }
}

pub fn init(area_config: AreaConfig, config: MandelbrotConfig) -> Application {
    let area: Area<'_> = area::init(area_config);
    let wx = area.data.lock().unwrap().width_x;
    let hy = area.data.lock().unwrap().height_y;
    Application {
        data_image: data_image::init(Static, &area),
        width: wx,
        height: hy,
        area,
        min: 0, // TODO config min?
        max: config.iteration_max,
        palette: config.palette,
        palette_zero: config.palette_zero,
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}

pub fn init_nebula(area_config: AreaConfig, config: FractalConfig) -> Application {
    let area = area::init(area_config);
    let wx = area.data.lock().unwrap().width_x;
    let hy = area.data.lock().unwrap().height_y;
    Application {
        data_image: data_image::init(Static, &area),
        width: wx,
        height: hy,
        area,
        min: 0,
        max: config.iteration_max,
        palette: config.palette,
        palette_zero: palettes::init_none(),
        resolution_multiplier: config.resolution_multiplier,
    }
}

pub fn init_none<'lt>() -> Application<'lt> {
    Application {
        data_image: data_image::init_none(),
        width: 0,
        height: 0,
        area: area::init_none(),
        min: 0,
        max: 100,
        palette: palettes::init_none(),
        palette_zero: palettes::init_none(),
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
