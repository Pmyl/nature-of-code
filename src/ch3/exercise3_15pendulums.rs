use nannou::color::{BLACK, WHITE};
use nannou::geom::{pt2, Point2};
use nannou::Draw;
use nature_of_code::utils::pendulum::Pendulum;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(1024, 400, 2));
}

struct State {
    pendulums: Vec<Pendulum>,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            pendulums: vec![
                Pendulum::new(pt2(exercise.half_width(), 0.0), 100.0),
                Pendulum::new(pt2(exercise.half_width(), 55.0), 150.0),
                Pendulum::new(pt2(exercise.half_width(), 110.0), 120.0),
            ]
        }
    }
    
    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        for pendulum in self.pendulums.iter() {
            draw.ellipse()
                .wh(Point2::ONE * 24.0)
                .xy(pendulum.bob);

            draw.line()
                .color(WHITE)
                .start(pendulum.pivot)
                .end(pendulum.bob);
        }

    }

    fn step(&mut self, exercise: &ExerciseData) {
        let mut pivot = pt2(exercise.half_width(), 0.00);
        for p in self.pendulums.iter_mut() {
            p.pivot = pivot;
            p.update();
            pivot = p.bob;
        }
    }
}
