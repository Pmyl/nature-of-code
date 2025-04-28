use crate::utils::particle_force::ParticleForce;
use nannou::geom::Point2;

pub struct Particle {
    pub position: Point2,
    pub acceleration: Point2,
    pub velocity: Point2,

    pub total_life_span: f32,
    pub remaining_life: f32,
}

impl Particle {
    pub fn new(position: Point2, life_span: f32) -> Self {
        Particle {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            remaining_life: life_span,
            total_life_span: life_span,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Point2::ZERO;

        self.remaining_life -= 1.0;
    }

    pub(super) fn apply_force_internal(&mut self, force: Point2) {
        self.acceleration += force;
    }

    pub fn apply_force(&mut self, force: &impl ParticleForce) {
        force.apply_force_to(self)
    }

    pub fn is_dead(&self) -> bool {
        self.remaining_life < 0.0
    }
}
