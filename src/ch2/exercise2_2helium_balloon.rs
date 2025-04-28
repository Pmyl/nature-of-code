use nannou::color::BLACK;
use nannou::geom::{pt2, Point2};
use nannou::noise::{NoiseFn, Perlin};
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::body::Body;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    mover: Body,
    noise: Perlin,
    frames: usize,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            mover: Body::new(pt2(200., 240.), 10., pt2(20., 26.)),
            noise: Perlin::new(),
            frames: 0,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().wh(self.mover.size).xy(self.mover.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
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

        if self.mover.position.x <= 0. || self.mover.position.x >= exercise.width() as f32 {
            self.mover.velocity.x *= -0.3;
        }
    }
}
