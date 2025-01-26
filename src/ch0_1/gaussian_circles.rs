use nannou::color::Rgba;
use nannou::{App, Frame};
use nature_of_code::Exercise;
use rand_distr::Distribution;
use rand_distr::Normal;

const EXERCISE: Exercise = Exercise::new(640, 300, 2);

pub fn run() {
    nannou::app(model).run();
}

fn model(app: &App) {
    EXERCISE.init_with_view(app, view);
}

fn view(app: &App, _: &(), frame: Frame) {
    let draw = EXERCISE.draw(app);

    let normal = Normal::new(0., 1.).unwrap();

    let x: f32 = normal.sample(&mut rand::thread_rng()) * 60. + 320.;
    let y: f32 = normal.sample(&mut rand::thread_rng()) * 60. + 150.;

    let r: f32 = normal.sample(&mut rand::thread_rng()) * 0.1 + 0.5;
    let g: f32 = normal.sample(&mut rand::thread_rng()) * 0.1 + 0.5;
    let b: f32 = normal.sample(&mut rand::thread_rng()) * 0.1 + 0.5;

    draw.ellipse()
        .color(Rgba::new(r, g, b, 0.1))
        .width(16.)
        .height(16.)
        .x(x)
        .y(y);

    draw.to_frame(app, &frame).unwrap()
}
