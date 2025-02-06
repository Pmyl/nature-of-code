use super::mover::Mover;
use nannou::prelude::Pow;

pub struct GravitationalAttraction;

impl GravitationalAttraction {
    pub fn apply_to(self, mover: &mut Mover, attractor: &Mover) {
        let mover_mass = mover.mass;
        let attractor_mass = attractor.mass;
        let distance = (mover.position - attractor.position).length().clamp(5.,25.);
        let direction = (attractor.position - mover.position).normalize();

        let force = mover_mass * attractor_mass * direction / distance.pow(2);
        mover.apply_force(force);
    }
}
