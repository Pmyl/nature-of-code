use nannou::color::{Rgba, BLACK};
use nannou::geom::{pt2, Point2};
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::particle_emitter::ParticleEmitter;
use nature_of_code::utils::repeller::Repeller;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    emitter: ParticleEmitter,
    repeller: Repeller,
}

impl ExerciseState for State {
    fn new(exercise_data: &ExerciseData, _: &App) -> Self {
        State {
            emitter: ParticleEmitter::new(pt2(exercise_data.half_width(), 20.)),
            repeller: Repeller {
                power: 100.,
                position: pt2(exercise_data.half_width(), 200.),
            },
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        for particle in &self.emitter.particles {
            draw.ellipse()
                .xy(particle.position)
                .wh(Point2::ONE * 10.0)
                .color(Rgba::new(
                    1.0,
                    1.0,
                    1.0,
                    particle.remaining_life / particle.total_life_span,
                ));
        }

        draw.ellipse()
            .xy(self.repeller.position)
            .wh(Point2::ONE * 32.0);
    }

    fn step(&mut self, _: &ExerciseData) {
        if let Some(particle) = self.emitter.add_particle() {
            particle.velocity = pt2(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..0.0),
            );
        }

        self.emitter.apply_force(&Gravity(pt2(0., 0.1)));
        self.emitter.apply_force(&self.repeller);
        self.emitter.update();
    }
}
