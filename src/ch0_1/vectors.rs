use nannou::color::BLACK;
use nannou::geom::{pt2, Point2};
use nannou::noise::NoiseFn;
use nannou::noise::Perlin;
use nannou::Draw;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    position: Point2,
    velocity: Point2,
    acceleration: Point2,
    noise: Perlin,
    frames: usize,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            position: pt2(100., 100.),
            velocity: pt2(2.5, 2.),
            acceleration: pt2(-0.001, 0.01),
            noise: Perlin::new(),
            frames: 0,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.ellipse().width(48.).height(48.).xy(self.position);
    }

    fn step(&mut self, exercise: &ExerciseData) {
        self.frames += 1;
        self.acceleration = Point2::new(
            self.noise.get([self.frames as f64 / 10., 0.]) as f32,
            self.noise.get([0., self.frames as f64 / 10.]) as f32,
        );
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0., 10.);
        self.position += self.velocity;

        if self.position.x < 0. || self.position.x > exercise.width() as f32 {
            self.velocity.x *= -1.;
            self.acceleration.x *= -1.;
        }

        if self.position.y < 0. || self.position.y > exercise.height() as f32 {
            self.velocity.y *= -1.;
            self.acceleration.y *= -1.;
        }
    }
}
