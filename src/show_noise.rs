use nannou::noise::{OpenSimplex, Seedable};
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::Exercise;

const EXERCISE: Exercise = Exercise::new(400, 400, 2);

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        width: EXERCISE.width(),
        height: EXERCISE.height(),
        noise: OpenSimplex::new().set_seed(987654),
    }
}

fn update(_app: &App, walker: &mut State, _update: Update) {
    walker.step();
}

fn view(app: &App, walker: &State, frame: Frame) {
    let draw = EXERCISE.draw(app);

    walker.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct State {
    width: u32,
    height: u32,
    noise: OpenSimplex,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        //
    }

    pub fn step(&mut self) {
        //
    }
}
