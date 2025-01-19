use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::{pt2, Point2};
use nannou::Event;
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::Exercise;

const EXERCISE: Exercise = Exercise::new(640, 240, 2);

pub fn run() {
    nannou::app(model).update(update).event(event).run();
}

fn event(_app: &App, state: &mut State, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(wevent),
            ..
        } => match wevent {
            WindowEvent::KeyPressed(nannou::event::Key::Up) => {
                state.up_pressed = true;
            }
            WindowEvent::KeyReleased(nannou::event::Key::Up) => {
                state.up_pressed = false;
            }
            WindowEvent::KeyPressed(nannou::event::Key::Down) => {
                state.down_pressed = true;
            }
            WindowEvent::KeyReleased(nannou::event::Key::Down) => {
                state.down_pressed = false;
            }
            _ => {}
        },
        _ => {}
    }
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        position: pt2(100., 100.),
        velocity: pt2(1., 0.),
        acceleration: pt2(0., 0.),
        up_pressed: false,
        down_pressed: false,
    }
}

fn update(_app: &App, walker: &mut State, _update: Update) {
    walker.step();
}

fn view(app: &App, state: &State, frame: Frame) {
    let draw = EXERCISE.draw(app);
    state.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct State {
    position: Point2,
    velocity: Point2,
    acceleration: Point2,
    up_pressed: bool,
    down_pressed: bool,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);
        draw.ellipse().width(48.).height(48.).xy(self.position);
    }

    pub fn step(&mut self) {
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

        if self.position.x < 0. || self.position.x > EXERCISE.width() as f32 {
            self.velocity.x *= -1.;
        }

        if self.position.y < 0. || self.position.y > EXERCISE.height() as f32 {
            self.velocity.y *= -1.;
        }
    }
}
