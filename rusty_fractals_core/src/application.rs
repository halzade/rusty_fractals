use std::borrow::BorrowMut;
use fltk::app;
use fltk::app::{Receiver, Sender};
use rusty_fractals_common::{area, data_image, palettes};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::data_image::DataType::Static;
use rusty_fractals_common::fractal::{Conf, FractalConfig, MandelbrotConfig};
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;

pub struct Application<'lt> {
    pub data: DataImage<'lt>,
    pub width: usize,
    pub height: usize,
    pub area: Area<'lt>,
    pub conf: Conf,
    pub palette: Palette<'lt>,
    //  mandelbrot specific
    pub palette_zero: Palette<'lt>,
    //  nebula specific
    pub resolution_multiplier: ResolutionMultiplier,
}

impl<'lt> Application<'lt> {
    pub fn move_target_zoom_in_recalculate_pixel_positions(&mut self, x: usize, y: usize, is_mandelbrot: bool) {
        self.area.move_target(x, y);
        self.area.zoom_in();
        self.data.borrow_mut().recalculate_pixels_positions_for_next_calculation(&self.area, is_mandelbrot);

        let image_rgb = self.data.image_temp(false, None);
        let (sender, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = app::channel();
        sender.send(image_rgb);
    }
}

pub fn init(area_config: AreaConfig, config: MandelbrotConfig) -> Application {
    let area = area::init(area_config);
    Application {
        data: data_image::init(Static, &area),
        width: area.width_x,
        height: area.height_y,
        area,
        conf: Conf { min: 0, max: config.iteration_max },
        palette: config.palette,
        palette_zero: config.palette_zero,
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}

pub fn init_nebula(area_config: AreaConfig, config: FractalConfig) -> Application {
    let area = area::init(area_config);
    Application {
        data: data_image::init(Static, &area),
        width: area.width_x,
        height: area.height_y,
        area,
        conf: Conf { min: 0, max: config.iteration_max },
        palette: config.palette,
        palette_zero: palettes::init_none(),
        resolution_multiplier: config.resolution_multiplier,
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
        resolution_multiplier: ResolutionMultiplier::Single,
    }
}
