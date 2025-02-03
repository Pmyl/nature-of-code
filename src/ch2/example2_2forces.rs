use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::pt2;
use nannou::Draw;
use nannou::Event;
use nature_of_code::utils::mover::Mover;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    mover1: Mover,
    mover2: Mover,
    mouse_pressed: bool,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            mover1: Mover::new_simple(pt2(100., 30.), 10.),
            mover2: Mover::new_simple(pt2(400., 30.), 2.),
            mouse_pressed: false,
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().wh(self.mover1.size).xy(self.mover1.position);
        draw.ellipse().wh(self.mover2.size).xy(self.mover2.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        step_mover(self.mouse_pressed, &mut self.mover1, &exercise);
        step_mover(self.mouse_pressed, &mut self.mover2, &exercise);
    }

    fn handle_event(&mut self, event: Event, _exercise: &ExerciseData) {
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

fn step_mover(mouse_pressed: bool, mover: &mut Mover, exercise: &ExerciseData) {
    if mouse_pressed {
        let wind_force = pt2(0.1, 0.);
        mover.apply_force(wind_force);
    }

    let gravity_force = pt2(0., 0.1);
    mover.apply_force(gravity_force);
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
