use std::f32::consts::PI;

use nannou::color::{BLACK, WHITE};
use nannou::geom::{pt2, pt3, Point2};
use nannou::Draw;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    frame: usize,
    x: f32,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State { frame: 0, x: 0.0 }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        let translated = draw.translate(pt3(320., 120., 0.));

        translated
            .line()
            .color(WHITE)
            .start(pt2(0., 0.))
            .end(pt2(self.x, 0.));

        translated
            .ellipse()
            .xy(pt2(self.x, 0.))
            .wh(Point2::ONE * 30.0);
    }

    fn step(&mut self, _: &ExerciseData) {
        let amplitude = 100.0;
        let period = 480.0;
        let angle = 2.0 * PI * self.frame as f32 / period;

        self.x = amplitude * f32::sin(angle);
        self.frame += 1;
    }
}
