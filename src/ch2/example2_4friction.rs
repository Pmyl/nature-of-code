use nannou::color::BLACK;
use nannou::event::WindowEvent;
use nannou::geom::pt2;
use nannou::Event;
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::utils::friction::Friction;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::mover::Mover;
use nature_of_code::ExerciseData;

const EXERCISE: ExerciseData = ExerciseData::new(640, 240, 2);

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
        mover: Mover::new_simple(pt2(320., 100.), 5.),
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

        draw.ellipse().wh(self.mover.size).xy(self.mover.position);
    }

    pub fn step(&mut self) {
        Self::step_mover(self.mouse_pressed, &mut self.mover);
    }

    pub fn step_mover(mouse_pressed: bool, mover: &mut Mover) {
        if mouse_pressed {
            let wind_force = pt2(0.5, 0.);
            mover.apply_force(wind_force);
        }

        let gravity = Gravity(pt2(0., 0.1));
        gravity.apply_to(mover);

        if mover.contact_edges(EXERCISE.size()) {
            Friction(0.1).apply_to(mover);
        }

        mover.bounce_edges(EXERCISE.size(), 0.9);
        mover.update();
    }
}
