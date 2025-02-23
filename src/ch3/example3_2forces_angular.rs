use nannou::color::{BLACK, STEELBLUE};
use nannou::geom::{pt2, Point2, Vec3};
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
    attractor: Body,

}

impl ExerciseState for State {
    fn new(exercise_data: &ExerciseData) -> Self {
        State {
            bodies: array::from_fn::<_, 10, _>(|_: usize| {
                let mut body = Body::new_body(
                    pt2(thread_rng().gen_range(0.0..exercise_data.width() as f32), thread_rng().gen_range(0.0..exercise_data.height() as f32)),
                    thread_rng().gen_range(2..=8) as f32
                );
                body.velocity = pt2(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0));
                body
            }) .into_iter()
                .collect::<Vec<_>>(),
            attractor: Body::new_body(pt2(320., 120.), 10.)
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        for body in &self.bodies {
            let rotated = draw
                .translate(Vec3::new(body.position.x, body.position.y, 0.))
                .rotate(body.angle);

            rotated.ellipse().wh(body.size);

            rotated
                .line()
                .color(STEELBLUE)
                .start(Point2::ZERO)
                .end(pt2(body.size.x / 2., 0.));
        }

        draw.ellipse().wh(self.attractor.size).xy(self.attractor.position);
    }

    fn step(&mut self, _: &ExerciseData) {
        for i in 0..self.bodies.len() {
            let force = GravitationalAttraction.calculate_force(&self.bodies[i], &self.attractor);
            self.bodies[i].apply_force(force);
            self.bodies[i].angular_acceleration = (self.bodies[i].acceleration.x / 10.).clamp(-0.1, 0.1);
        }

        for body in self.bodies.iter_mut() {
            body.update();
        }
    }
}
