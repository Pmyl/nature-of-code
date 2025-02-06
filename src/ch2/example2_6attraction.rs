use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::prelude::Pow;
use nannou::Draw;
use nature_of_code::utils::mover::Mover;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    movers: Vec<Mover>,
    attractor: Mover,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            movers: [Mover::new_simple(pt2(320., 80.), 3.)]
                .into_iter()
                .collect::<Vec<_>>(),
            attractor: Mover::new_simple(pt2(320., 120.), 5.),
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        for mover in self.movers.iter() {
            draw.ellipse().wh(mover.size).xy(mover.position);
        }

        draw.ellipse()
            .wh(self.attractor.size)
            .xy(self.attractor.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        for mover in self.movers.iter_mut() {
            let mover_mass = mover.mass;
            let attractor_mass = self.attractor.mass;
            let distance = (mover.position - self.attractor.position).length();
            let direction = (self.attractor.position - mover.position).normalize();

            let force = mover_mass * attractor_mass * direction / distance.pow(2);

            mover.apply_force(force);
            mover.update();
            mover.bounce_edges(exercise.size(), 0.1);
        }
    }
}
