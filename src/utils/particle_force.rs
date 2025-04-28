use nannou::geom::Point2;
use crate::utils::particle::Particle;
pub trait ParticleForce {
    fn apply_force_to(&self, p: &mut Particle);
}

impl ParticleForce for Point2 {
    fn apply_force_to(&self, p: &mut Particle) {
        p.apply_force_internal(*self);
    }
}

