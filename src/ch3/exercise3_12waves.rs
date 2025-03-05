use nannou::color::BLACK;
use nannou::geom::{pt2, pt3, Point2};
use nannou::Draw;
use nature_of_code::utils::oscillator::Oscillator;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use std::array;
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    waves: Vec<Vec<(Oscillator, f32)>>,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            waves: array::from_fn::<_, 3, _>(|wave_i| {
                let delta_angle = thread_rng().gen_range(0.1..0.4);
                let velocity = thread_rng().gen_range(0.05..0.1);
                array::from_fn::<_, 30, _>(|i: usize| {
                    (Oscillator::new(pt2(0., i as f32 * delta_angle), pt2(0., velocity)), 20. + 20. * wave_i as f32)
                })
                .into_iter()
                .collect::<Vec<_>>()
            })
            .into_iter()
            .collect::<Vec<_>>(),
        }
    }

    fn show(&self, draw: &Draw, exercise: &ExerciseData) {
        draw.background().color(BLACK);

        let translated = draw.translate(pt3(0., exercise.height() as f32 / 2., 0.));

        let number_of_oscillators = self.waves[0].len();

        for oscillator_i in 0..number_of_oscillators {
            let mut y = 0.;
            for wave_i in 0..self.waves.len() {
                y = y + f32::sin(self.waves[wave_i][oscillator_i].0.angle.y) * self.waves[wave_i][oscillator_i].1;
            }

            let target_point = pt2(
                oscillator_i as f32 * exercise.width() as f32 / number_of_oscillators as f32,
                y
            );

            translated.ellipse().xy(target_point).wh(Point2::ONE * 30.0);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        for wave in self.waves.iter_mut() {
            for oscillator in wave {
                oscillator.0.update()
            }
        }
    }
}
