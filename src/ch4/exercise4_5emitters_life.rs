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
    emitters: Vec<ParticleEmitter>,
    mouse_clicked: bool,
    mouse_position: Point2,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            emitters: vec![],
            mouse_clicked: false,
            mouse_position: pt2(0.0, 0.0)
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        self.mouse_clicked = event.pressed_state_changed().unwrap_or(self.mouse_clicked);
        self.mouse_position = event.get_position(exercise).unwrap_or(self.mouse_position);
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        for emitter in &self.emitters {
            for particle in &emitter.particles {
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
    }

    fn step(&mut self, _: &ExerciseData) {
        if self.mouse_clicked {
            let mut emitter = ParticleEmitter::new(self.mouse_position);
            emitter.forces.push(pt2(0.0, 0.05));
            self.emitters.push(emitter);
            self.mouse_clicked = false;
        }

        for emitter in self.emitters.iter_mut() {
            let particle = emitter.add_particle();
            particle.velocity = pt2(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..0.0),
            );

            emitter.update_with_forces();
        }

    }
}
