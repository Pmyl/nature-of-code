use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::utils::mover::Mover;
use nature_of_code::Exercise;

const EXERCISE: Exercise = Exercise::new(640, 240, 2);

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        movers: vec![
            Mover::new_simple(pt2(100., 30.), 10.),
            Mover::new_simple(pt2(400., 30.), 2.),
        ],
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
    movers: Vec<Mover>,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);

        for mover in &self.movers {
            draw.ellipse()
                .wh(mover.size)
                .xy(mover.position);
        }
    }

    pub fn step(&mut self) {
        for mut mover in self.movers.iter_mut() {
            Self::step_mover(&mut mover);
        }
    }

    pub fn step_mover(mover: &mut Mover) {
        let center = pt2(EXERCISE.width() as f32 / 2., EXERCISE.height() as f32 / 2.);
        let max_force = 0.1;
        let horizontal_force = (center.x - mover.position.x) / center.x * max_force;
        let vertical_force = (center.y - mover.position.y) / center.y * max_force;
        mover.apply_force(pt2(horizontal_force, vertical_force));
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
