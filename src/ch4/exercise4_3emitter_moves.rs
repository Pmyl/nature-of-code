use nannou::color::{Rgba, BLACK};
use nannou::geom::{pt2, Point2};
use nannou::Draw;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::particle_emitter::ParticleEmitter;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    emitter: ParticleEmitter,
    mouse_position: Point2,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        let mut emitter = ParticleEmitter::new(exercise.half_position());
        emitter.forces.push(pt2(0.0, 0.05));

        State {
            emitter,
            mouse_position: Point2::ZERO,
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        self.mouse_position = event.get_position(exercise).unwrap_or(self.mouse_position);
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
    }

    fn step(&mut self, _: &ExerciseData) {
        let particle = self.emitter.add_particle();
        particle.velocity = pt2(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..0.0),
        );

        self.emitter.origin = self.mouse_position;
        self.emitter.update_with_forces();
    }
}
