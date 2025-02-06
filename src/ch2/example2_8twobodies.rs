use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::Draw;
use nature_of_code::utils::body::Body;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use nature_of_code::utils::gravitational_attraction::GravitationalAttraction;

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    body1: Body,
    body2: Body,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        let mut body1 = Body::new_body(pt2(320., 40.), 8.);
        body1.velocity = pt2(1., 0.);

        let mut body2 = Body::new_body(pt2(320., 200.), 8.);
        body2.velocity = pt2(-1., 0.);

        State {
            body1,
            body2,
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        draw.ellipse().wh(self.body1.size).xy(self.body1.position);
        draw.ellipse().wh(self.body2.size).xy(self.body2.position);
    }

    fn step(&mut self, _: &ExerciseData) {
        GravitationalAttraction.apply_to(&mut self.body1, &self.body2);
        GravitationalAttraction.apply_to(&mut self.body2, &self.body1);
        self.body1.update();
        self.body2.update();
    }
}
