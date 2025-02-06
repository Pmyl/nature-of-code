use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::pt2;
use nannou::{Draw, Event};
use nature_of_code::utils::friction::Friction;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::mover::Mover;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2))
}

struct State {
    mover: Mover,
    mouse_pressed: bool,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            mover: Mover::new_simple(pt2(320., 100.), 5.),
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
        match event {
            Event::WindowEvent {
                simple: Some(wevent),
                ..
            } => match wevent {
                WindowEvent::MousePressed(_) => {
                    self.mouse_pressed = true;
                }
                WindowEvent::MouseReleased(_) => {
                    self.mouse_pressed = false;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
