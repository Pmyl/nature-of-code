use nannou::color::{BLACK, WHITE};
use nannou::geom::{pt2, Point2};
use nannou::Draw;
use nature_of_code::utils::pendulum::Pendulum;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    pendulum: Pendulum,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            pendulum: Pendulum::new(pt2(exercise.half_width(), 0.0), 175.0)
        }
    }
    
    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        draw.ellipse()
            .wh(Point2::ONE * 24.0)
             .xy(self.pendulum.bob);
        
        draw.line()
            .color(WHITE)
            .start(self.pendulum.pivot)
            .end(self.pendulum.bob);
    }

    fn step(&mut self, _: &ExerciseData) {
        self.pendulum.update();
    }
}
