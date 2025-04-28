use crate::utils::particle::Particle;
use crate::utils::particle_force::ParticleForce;
use nannou::geom::Point2;
use nannou::math::clamp;
use nannou::prelude::Pow;

pub struct Repeller {
    pub power: f32,
    pub position: Point2
}

impl ParticleForce for Repeller {
    fn apply_force_to(&self, p: &mut Particle) {
        let direction = self.position - p.position;
        let mut distance = direction.length();
        distance = clamp(distance, 5., 50.);
        let strength = -1. * self.power / distance.pow(2);
        let force = direction.normalize() * strength;
        p.apply_force_internal(force);
    }
}