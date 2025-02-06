use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::Draw;
use nature_of_code::utils::gravitational_attraction::GravitationalAttraction;
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
        let mut mover = Mover::new_simple(pt2(300., 50.), 2.);
        mover.velocity = pt2(1., 0.);
        State {
            movers: [mover]
                .into_iter()
                .collect::<Vec<_>>(),
            attractor: Mover::new(pt2(320., 120.), 20., pt2(40., 40.)),
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

    fn step(&mut self, _: &ExerciseData) {
        for mover in self.movers.iter_mut() {
            GravitationalAttraction.apply_to(mover, &self.attractor);
            mover.update();
        }
    }
}
