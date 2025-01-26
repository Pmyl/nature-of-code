use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::pt2;
use nannou::Event;
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::utils::mover::Mover;
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
            WindowEvent::MousePressed(_) => {
                state.mouse_pressed = true;
            }
            WindowEvent::MouseReleased(_) => {
                state.mouse_pressed = false;
            }
            _ => {}
        },
        _ => {}
    }
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        mover: Mover {
            position: pt2(300., 120.),
            mass: 1.,
            ..Default::default()
        },
        mouse_pressed: false,
    }
}

fn update(_app: &App, state: &mut State, _update: Update) {
    state.step();
}

fn view(app: &App, state: &State, frame: Frame) {
    let draw = EXERCISE.draw(app);
    state.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct State {
    mover: Mover,
    mouse_pressed: bool,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);
        draw.ellipse()
            .width(self.mover.mass * 16. * 2.)
            .height(self.mover.mass * 16. * 2.)
            .xy(self.mover.position);
    }

    pub fn step(&mut self) {
        if self.mouse_pressed {
            let wind_force = pt2(0.1, 0.);
            self.mover.apply_force(wind_force);
        }

        let gravity_force = pt2(0., 0.1);
        self.mover.apply_force(gravity_force);
        self.mover.update();

        if self.mover.position.y <= 0. || self.mover.position.y >= EXERCISE.height() as f32 {
            self.mover.velocity.y *= -1.;
        }

        if self.mover.position.x <= 0. || self.mover.position.x >= EXERCISE.width() as f32 {
            self.mover.velocity.x *= -1.;
        }
    }
}
