use nannou::color::Rgba;
use nannou::Draw;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand_distr::Distribution;
use rand_distr::Normal;

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 300, 2));
}

struct State;
impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        Self
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
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
    }

    fn step(&mut self, _: &ExerciseData) {}
}
