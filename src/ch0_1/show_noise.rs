use nannou::color::Rgba;
use nannou::geom::pt2;
use nannou::noise::{NoiseFn, Perlin};
use nannou::Draw;
use nature_of_code::{map_range, ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(400, 400, 2));
}

struct State {
    width: u32,
    height: u32,
    noise: Perlin,
    frames: usize,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            width: exercise.width(),
            height: exercise.height(),
            noise: Perlin::new(),
            frames: 0,
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        for x in 0..self.width {
            for y in 0..self.height {
                let noise = self.noise.get([
                    x as f64 / 100.0 + self.frames as f64 / 10.,
                    y as f64 / 100.0 - self.frames as f64 / 10.,
                ]);
                let mapped_noise = map_range(noise as f32, (-1., 1.), (0., 1.));

                draw.line()
                    .x(x as f32)
                    .y(y as f32)
                    .color(Rgba::new(mapped_noise, mapped_noise, mapped_noise, 1.))
                    .stroke_weight(1.0)
                    .points(pt2(0., 0.), pt2(0., 1.));
            }
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        self.frames += 1;
    }
}
