use nannou::color::BLACK;
use nannou::geom::{pt2, pt3, Point2};
use nannou::Draw;
use nature_of_code::utils::oscillator::Oscillator;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use std::array;

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    oscillators: Vec<Oscillator>,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            oscillators: array::from_fn::<_, 30, _>(|i: usize| {
                let delta_angle = 0.2;
                Oscillator::new(
                    pt2(0., i as f32 * delta_angle),
                    pt2(0., 0.05),
                )
            })
            .into_iter()
            .collect::<Vec<_>>(),
        }
    }

    fn show(&self, draw: &Draw, exercise: &ExerciseData) {
        draw.background().color(BLACK);

        let translated = draw.translate(pt3(0., exercise.height() as f32 / 2., 0.));

        let number_of_oscillators = self.oscillators.len();
        for (i, oscillator) in self.oscillators.iter().enumerate() {
            let target_point = pt2(
                i as f32 * exercise.width() as f32 / number_of_oscillators as f32,
                f32::sin(oscillator.angle.y) * exercise.height() as f32 / 2.0,
            );

            translated.ellipse().xy(target_point).wh(Point2::ONE * 30.0);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        for oscillator in self.oscillators.iter_mut() {
            oscillator.update()
        }
    }
}
