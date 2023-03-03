use std::borrow::BorrowMut;
use rusty_fractals_common::{area, data_image, palettes};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::data_image::DataType::Static;
use rusty_fractals_common::fractal::{Conf, MandelbrotConfig};
use rusty_fractals_common::palette::Palette;

pub struct Application<'lt> {
    pub data: DataImage<'lt>,
    pub width: usize,
    pub height: usize,
    pub area: Area<'lt>,
    pub conf: Conf,
    pub palette: Palette<'lt>,
    pub palette_zero: Palette<'lt>,
}

impl<'lt> Application<'lt> {
    pub fn move_target_zoom_in_recalculate_pixel_positions(&mut self, x: usize, y: usize, is_mandelbrot: bool) {
        self.area.move_target(x, y);
        self.area.zoom_in();
        self.data.borrow_mut().recalculate_pixels_positions_for_next_calculation(&self.area, is_mandelbrot);
    }
}

pub fn init(area_config: AreaConfig, mandelbrot_config: MandelbrotConfig) -> Application {
    let area = area::init(area_config);
    Application {
        data: data_image::init(Static, &area),
        width: area.width_x,
        height: area.height_y,
        area,
        conf: Conf { min: 0, max: mandelbrot_config.iteration_max },
        palette: mandelbrot_config.palette,
        palette_zero: mandelbrot_config.palette_zero,
    }
}

pub fn init_none<'lt>() -> Application<'lt> {
    Application {
        data: data_image::init_none(),
        width: 0,
        height: 0,
        area: area::init_none(),
        conf: Conf { min: 0, max: 10 },
        palette: palettes::init_none(),
        palette_zero: palettes::init_none(),
    }
}
