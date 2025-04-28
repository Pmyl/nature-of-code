use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::wgpu::Texture;
use nannou::{image, App, Draw};
use nature_of_code::utils::particle_emitter::ParticleEmitter;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand_distr::{Distribution, Normal};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    emitter: ParticleEmitter,
    texture: Texture,
    normal_vx: Normal<f32>,
    normal_vy: Normal<f32>,
}

impl ExerciseState for State {
    fn new(exercise_data: &ExerciseData, app: &App) -> Self {
        State {
            emitter: ParticleEmitter::new(pt2(exercise_data.half_width(), 180.)),
            texture: Texture::from_image(
                app,
                &image::load_from_memory(include_bytes!("./assets/fuzzy_circle.png")).unwrap(),
            ),
            normal_vx: Normal::new(0., 0.3).unwrap(),
            normal_vy: Normal::new(-1., 0.3).unwrap(),
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        for particle in &self.emitter.particles {
            draw.texture(&self.texture).xy(particle.position);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        let wind_force = pt2(0.05, 0.);

        if let Some(particle) = self.emitter.add_particle() {
            particle.velocity = pt2(
                self.normal_vx.sample(&mut rand::thread_rng()),
                self.normal_vy.sample(&mut rand::thread_rng()),
            );
        }

        self.emitter.apply_force(&wind_force);
        self.emitter.update();
    }
}
