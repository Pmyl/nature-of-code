use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::{pt2, Point2};
use nannou::{event::Update, App, Draw, Event, Frame};
use nature_of_code::Exercise;

const EXERCISE: Exercise = Exercise::new(640, 240, 2);

pub fn run() {
    nannou::app(model).update(update).event(event).run();
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        position: pt2(100., 100.),
        velocity: pt2(2.5, 2.),
        acceleration: pt2(-0.001, 0.01),
        mouse_position: Point2::ZERO,
    }
}

fn event(_app: &App, state: &mut State, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(wevent),
            ..
        } => match wevent {
            WindowEvent::MouseMoved(mouse) => {
                state.mouse_position = pt2(
                    mouse.x / EXERCISE.scale() as f32 + (EXERCISE.width() / 2) as f32,
                    -mouse.y / EXERCISE.scale() as f32 + (EXERCISE.height() / 2) as f32,
                )
            }
            _ => {}
        },
        _ => {}
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
    mouse_position: Point2,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);
        draw.ellipse().width(48.).height(48.).xy(self.position);
    }

    pub fn step(&mut self) {
        let mut direction = self.mouse_position - self.position;
        direction = direction.normalize();
        self.acceleration = direction * 0.5;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0., 10.);
        self.position += self.velocity;

        if self.position.x < 0. || self.position.x > EXERCISE.width() as f32 {
            self.velocity.x *= -1.;
        }

        if self.position.y < 0. || self.position.y > EXERCISE.height() as f32 {
            self.velocity.y *= -1.;
        }
    }
}
