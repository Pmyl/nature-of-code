use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::Draw;
use nannou::Event;
use nature_of_code::utils::body::Body;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    mover: Body,
    mouse_pressed: bool,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            mover: Body::new(pt2(300., 120.), 1., pt2(32., 32.)),
            mouse_pressed: false,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().wh(self.mover.size).xy(self.mover.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        if self.mouse_pressed {
            let wind_force = pt2(0.1, 0.);
            self.mover.apply_force(wind_force);
        }

        let gravity_force = pt2(0., 0.1);
        self.mover.apply_force(gravity_force);
        self.mover.update();

        if self.mover.position.y <= 0. || self.mover.position.y >= exercise.height() as f32 {
            self.mover.velocity.y *= -1.;
        }

        if self.mover.position.x <= 0. || self.mover.position.x >= exercise.width() as f32 {
            self.mover.velocity.x *= -1.;
        }
    }

    fn handle_event(&mut self, event: Event, _exercise: &ExerciseData) {
        if let Some(pressed) = event.pressed_state_changed() {
            self.mouse_pressed = pressed;
        }
    }
}
