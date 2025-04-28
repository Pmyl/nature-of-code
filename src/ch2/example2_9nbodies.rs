use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::body::Body;
use nature_of_code::utils::gravitational_attraction::GravitationalAttraction;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};
use std::array;

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    bodies: Vec<Body>,
}

impl ExerciseState for State {
    fn new(exercise_data: &ExerciseData, _: &App) -> Self {
        State {
            bodies: array::from_fn::<_, 10, _>(|_: usize| {
                Body::new_body(
                    pt2(
                        thread_rng().gen_range(0.0..exercise_data.width() as f32),
                        thread_rng().gen_range(0.0..exercise_data.height() as f32),
                    ),
                    thread_rng().gen_range(2..=8) as f32,
                )
            })
            .into_iter()
            .collect::<Vec<_>>(),
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        for body in &self.bodies {
            draw.ellipse().wh(body.size).xy(body.position);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        for i in 0..self.bodies.len() {
            for j in 0..self.bodies.len() {
                if i != j {
                    let force =
                        GravitationalAttraction.calculate_force(&self.bodies[i], &self.bodies[j]);
                    self.bodies[i].apply_force(force);
                }
            }
        }

        for body in self.bodies.iter_mut() {
            body.update();
        }
    }
}
