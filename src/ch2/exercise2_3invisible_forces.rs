use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::Draw;
use nature_of_code::utils::mover::Mover;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    movers: Vec<Mover>,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            movers: vec![
                Mover::new_simple(pt2(100., 30.), 10.),
                Mover::new_simple(pt2(400., 30.), 2.),
            ],
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        for mover in &self.movers {
            draw.ellipse().wh(mover.size).xy(mover.position);
        }
    }

    fn step(&mut self, exercise: &ExerciseData) {
        for mut mover in self.movers.iter_mut() {
            step_mover(&mut mover, &exercise);
        }
    }
}

fn step_mover(mover: &mut Mover, exercise: &ExerciseData) {
    let center = pt2(exercise.width() as f32 / 2., exercise.height() as f32 / 2.);
    let max_force = 0.1;
    let horizontal_force = (center.x - mover.position.x) / center.x * max_force;
    let vertical_force = (center.y - mover.position.y) / center.y * max_force;
    mover.apply_force(pt2(horizontal_force, vertical_force));
    mover.update();

    if mover.position.y <= 0. {
        mover.velocity.y *= -1.;
        mover.position.y = 0.;
    }

    if mover.position.x <= 0. {
        mover.velocity.x *= -1.;
        mover.position.x = 0.;
    }

    if mover.position.y >= exercise.height() as f32 {
        mover.velocity.y *= -1.;
        mover.position.y = exercise.height() as f32;
    }

    if mover.position.x >= exercise.width() as f32 {
        mover.velocity.x *= -1.;
        mover.position.x = exercise.width() as f32;
    }
}
