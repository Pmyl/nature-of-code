use nannou::geom::{pt2, Point2, Vec3};
use nannou::Draw;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    theta: f32,
    r: f32,
    point: Point2,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            theta: 0.0,
            r: 0.0,
            point: Point2::ZERO,
        }
    }

    fn show(&self, draw: &Draw, exercise: &ExerciseData) {
        draw.translate(Vec3::new(
            exercise.width() as f32 / 2.,
            exercise.height() as f32 / 2.,
            0.,
        ))
        .ellipse()
        .xy(self.point)
        .wh(Point2::ONE * 5.0);
    }

    fn step(&mut self, _: &ExerciseData) {
        let x = self.r * self.theta.cos();
        let y = self.r * self.theta.sin();

        self.point = pt2(x, y);

        self.r += 0.02;
        self.theta += 0.02;
    }
}
