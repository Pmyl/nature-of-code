use nannou::color::BLACK;
use nannou::geom::{pt2, Point2};
use nannou::{Draw, Event};
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    position: Point2,
    velocity: Point2,
    acceleration: Point2,
    mouse_position: Point2,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            position: pt2(100., 100.),
            velocity: pt2(2.5, 2.),
            acceleration: pt2(-0.001, 0.01),
            mouse_position: Point2::ZERO,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().width(48.).height(48.).xy(self.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        let mut direction = self.mouse_position - self.position;
        direction = direction.normalize();
        self.acceleration = direction * 0.5;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0., 10.);
        self.position += self.velocity;

        if self.position.x < 0. || self.position.x > exercise.width() as f32 {
            self.velocity.x *= -1.;
        }

        if self.position.y < 0. || self.position.y > exercise.height() as f32 {
            self.velocity.y *= -1.;
        }
    }

    fn handle_event(&mut self, event: Event, exercise: &ExerciseData) {
        if let Some(pos) = event.get_position(exercise) {
            self.mouse_position = pos;
        }
    }
}
