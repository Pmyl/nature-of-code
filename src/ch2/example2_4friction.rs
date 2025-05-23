use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::body::Body;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::friction::Friction;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2))
}

struct State {
    mover: Body,
    mouse_pressed: bool,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            mover: Body::new_simple(pt2(320., 100.), 5.),
            mouse_pressed: false,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().wh(self.mover.size).xy(self.mover.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        if self.mouse_pressed {
            let wind_force = pt2(0.5, 0.);
            self.mover.apply_force(wind_force);
        }

        let gravity = Gravity(pt2(0., 0.1));
        gravity.apply_to(&mut self.mover);

        if self.mover.contact_edges(exercise.size()) {
            Friction(0.1).apply_to(&mut self.mover);
        }

        self.mover.bounce_edges(exercise.size(), 0.9);
        self.mover.update();
    }

    fn handle_event(&mut self, event: nannou::Event, _exercise: &ExerciseData) {
        if let Some(pressed) = event.pressed_state_changed() {
            self.mouse_pressed = pressed;
        }
    }
}
