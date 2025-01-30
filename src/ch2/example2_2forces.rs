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
        mover1: Mover::new_simple(pt2(100., 30.), 10.),
        mover2: Mover::new_simple(pt2(400., 30.), 2.),
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
    mover1: Mover,
    mover2: Mover,
    mouse_pressed: bool,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);

        draw.ellipse()
            .wh(self.mover1.size)
            .xy(self.mover1.position);

        draw.ellipse()
            .wh(self.mover2.size)
            .xy(self.mover2.position);
    }

    pub fn step(&mut self) {
        Self::step_mover(self.mouse_pressed, &mut self.mover1);
        Self::step_mover(self.mouse_pressed, &mut self.mover2);
    }

    pub fn step_mover(mouse_pressed: bool, mover: &mut Mover) {
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

        if mover.position.y >= EXERCISE.height() as f32 {
            mover.velocity.y *= -1.;
            mover.position.y = EXERCISE.height() as f32;
        }

        if mover.position.x >= EXERCISE.width() as f32 {
            mover.velocity.x *= -1.;
            mover.position.x = EXERCISE.width() as f32;
        }
    }
}
