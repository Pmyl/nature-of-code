use nannou::color::{Rgba, BLACK};
use nannou::geom::{pt2, Point2};
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::particle::Particle;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    gravity: Gravity,
    particles: Vec<Particle>,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            gravity: Gravity(pt2(0.0, 0.05)),
            particles: vec![],
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        for particle in &self.particles {
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

    fn step(&mut self, exercise: &ExerciseData) {
        let mut particle = Particle::new(pt2(exercise.half_width(), 50.0), 125.0);
        particle.velocity = pt2(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..0.0),
        );
        self.particles.push(particle);

        for i in (0..self.particles.len()).rev() {
            let particle = self.particles.get_mut(i).unwrap();
            particle.update();
            self.gravity.apply_to_particle(particle);

            if particle.is_dead() {
                self.particles.remove(i);
            }
        }
    }
}
