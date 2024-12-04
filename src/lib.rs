use nannou::{app::ViewFn, App, Draw};

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
