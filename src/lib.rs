pub mod utils;

use nannou::{app::ViewFn, App, Draw};
use nannou::geom::Point2;
use nannou::prelude::pt2;

pub struct Exercise {
    width: u32,
    height: u32,
    scale: u32,
}

impl Exercise {
    pub const fn new(width: u32, height: u32, scale: u32) -> Self {
        Self {
            width,
            height,
            scale,
        }
    }
    pub fn size(&self) -> Point2 {
        pt2(self.width as f32, self.height as f32)
    }

    pub fn init_with_view<Model: 'static>(&self, app: &App, view: ViewFn<Model>) {
        let _ = app
            .new_window()
            .size(self.width * self.scale, self.height * self.scale)
            .view(view)
            .build()
            .unwrap();
    }

    pub fn draw(&self, app: &App) -> Draw {
        app.draw()
            .scale_x(self.scale as f32)
            .scale_y(-(self.scale as f32))
            .x_y(
                -(self.width as f32) / (self.scale as f32),
                -(self.height as f32) / (self.scale as f32),
            )
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }
}

pub fn map_range(value: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let from_min = from_range.0;
    let from_max = from_range.1;
    let to_min = to_range.0;
    let to_max = to_range.1;

    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}
