use nannou::{color::WHITE, event::Update, geom::pt2, rand::random_f32, App, Draw, Frame};
use rand::random;

const INTERNAL_WIDTH: u32 = 300;
const INTERNAL_HEIGHT: u32 = 300;

const WINDOW_WIDTH: u32 = INTERNAL_WIDTH * 2;
const WINDOW_HEIGHT: u32 = INTERNAL_HEIGHT * 2;

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Walker {
    let _ = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Walker::new(
        INTERNAL_WIDTH,
        INTERNAL_HEIGHT,
        RightDownTendencyWalkerStrategy,
    )
}

fn update(_app: &App, walker: &mut Walker, _update: Update) {
    walker.step();
}

fn view(app: &App, walker: &Walker, frame: Frame) {
    let draw = app.draw().scale_x(2.).scale_y(-2.).x_y(
        -(INTERNAL_WIDTH as f32) / 2.,
        -(INTERNAL_HEIGHT as f32) / 2.,
    );

    walker.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct Walker {
    x: f32,
    y: f32,
    step_strategy: Box<dyn WalkerStrategy>,
}

impl Walker {
    pub fn new(width: u32, height: u32, strategy: impl WalkerStrategy + 'static) -> Self {
        Self {
            x: width as f32 / 2.,
            y: height as f32 / 2.,
            step_strategy: Box::new(strategy),
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.line()
            .x(self.x)
            .y(self.y)
            .color(WHITE)
            .stroke_weight(1.0)
            .points(pt2(0., 0.), pt2(0., 1.));
    }

    pub fn step(&mut self) {
        let (x, y) = self.step_strategy.step();
        self.x += x;
        self.y += y;
    }
}

trait WalkerStrategy {
    fn step(&self) -> (f32, f32);
}

struct UniformWalkerStrategy;
impl WalkerStrategy for UniformWalkerStrategy {
    fn step(&self) -> (f32, f32) {
        let xstep = f32::floor((random_f32() * 3.) - 1.);
        let ystep = f32::floor((random_f32() * 3.) - 1.);

        (xstep, ystep)
    }
}

struct RightDownTendencyWalkerStrategy;
impl WalkerStrategy for RightDownTendencyWalkerStrategy {
    fn step(&self) -> (f32, f32) {
        let xstepf = f32::floor((random_f32() * 3.) - 1.);
        let ystepf = f32::floor((random_f32() * 3.) - 1.);

        let xstep_leaning = (xstepf + 0.05).min(1.0);
        let ystep_leaning = (ystepf + 0.05).min(1.0);

        (xstep_leaning, ystep_leaning)
    }
}
