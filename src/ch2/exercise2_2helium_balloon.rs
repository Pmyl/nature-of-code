use nannou::color::BLACK;
use nannou::geom::{pt2, Point2};
use nannou::noise::{NoiseFn, Perlin};
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
        mover: Mover::new(pt2(200., 240.), 10., pt2(20., 26.)),
        noise: Perlin::new(),
        frames: 0,
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
    noise: Perlin,
    frames: usize,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);
        draw.ellipse()
            .wh(self.mover.size)
            .xy(self.mover.position);
    }

    pub fn step(&mut self) {
        self.frames += 1;
        let wind_force = Point2::new(
            self.noise.get([self.frames as f64 / 100., 0.]) as f32,
            self.noise.get([0., self.frames as f64 / 100.]) as f32,
        ) / 10.;
        let helium_force: Point2 = pt2(0., -0.005);
        self.mover.apply_force(helium_force);
        self.mover.apply_force(wind_force);
        self.mover.update();

        if self.mover.position.y <= 0. {
            self.mover.velocity.y *= -0.3;
        }

        if self.mover.position.x <= 0. || self.mover.position.x >= EXERCISE.width() as f32 {
            self.mover.velocity.x *= -0.3;
        }
    }
}
