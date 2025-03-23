use super::{bob::Bob, body::Body, particle::Particle};
use nannou::geom::Point2;

pub struct Gravity(pub Point2);

impl Gravity {
    pub fn apply_to(self, body: &mut Body) {
        body.apply_force(self.0 * body.mass);
    }

    pub fn apply_to_bob(self, bob: &mut Bob) {
        bob.apply_force(self.0 * bob.mass);
    }

    pub fn apply_to_particle(&self, particle: &mut Particle) {
        particle.apply_force(self.0);
    }
}
