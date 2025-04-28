use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::{pt2, Point2};
use nannou::Event;
use nannou::{App, Draw};
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    position: Point2,
    velocity: Point2,
    acceleration: Point2,
    up_pressed: bool,
    down_pressed: bool,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            position: pt2(100., 100.),
            velocity: pt2(1., 0.),
            acceleration: pt2(0., 0.),
            up_pressed: false,
            down_pressed: false,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().width(48.).height(48.).xy(self.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        if self.up_pressed {
            self.acceleration = pt2(0.01, 0.001);
        } else if self.down_pressed {
            self.acceleration = 0.01 * -self.velocity.normalize();
        } else {
            self.acceleration = Point2::ZERO;
        }

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0., 5.);
        self.position += self.velocity;

        if self.position.x < 0. || self.position.x > exercise.width() as f32 {
            self.velocity.x *= -1.;
        }

        if self.position.y < 0. || self.position.y > exercise.height() as f32 {
            self.velocity.y *= -1.;
        }
    }

    fn handle_event(&mut self, event: Event, _exercise: &ExerciseData) {
        match event {
            Event::WindowEvent {
                simple: Some(wevent),
                ..
            } => match wevent {
                WindowEvent::KeyPressed(nannou::event::Key::Up) => {
                    self.up_pressed = true;
                }
                WindowEvent::KeyReleased(nannou::event::Key::Up) => {
                    self.up_pressed = false;
                }
                WindowEvent::KeyPressed(nannou::event::Key::Down) => {
                    self.down_pressed = true;
                }
                WindowEvent::KeyReleased(nannou::event::Key::Down) => {
                    self.down_pressed = false;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
