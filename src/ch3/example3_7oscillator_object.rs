use std::array;

use nannou::color::{BLACK, WHITE};
use nannou::geom::{pt2, pt3, Point2};
use nannou::Draw;
use nature_of_code::utils::oscillator::Oscillator;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    oscillators: Vec<(Oscillator, Point2)>,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            oscillators: array::from_fn::<_, 10, _>(|_: usize| {
                (
                    Oscillator::new(
                        pt2(0., 0.),
                        pt2(
                            thread_rng().gen_range(-0.02..0.02),
                            thread_rng().gen_range(-0.02..0.02),
                        ),
                    ),
                    pt2(
                        thread_rng().gen_range(20.0..exercise.width() as f32 / 2.),
                        thread_rng().gen_range(20.0..exercise.height() as f32 / 2.),
                    ),
                )
            })
            .into_iter()
            .collect::<Vec<_>>(),
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        let translated = draw.translate(pt3(320., 120., 0.));

        for (oscillator, amplitude) in self.oscillators.iter() {
            let target_point = pt2(
                f32::sin(oscillator.angle.x) * amplitude.x,
                f32::sin(oscillator.angle.y) * amplitude.y,
            );

            translated
                .line()
                .color(WHITE)
                .start(pt2(0., 0.))
                .end(target_point);

            translated.ellipse().xy(target_point).wh(Point2::ONE * 30.0);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        for (oscillator, _) in self.oscillators.iter_mut() {
            oscillator.update()
        }
    }
}
