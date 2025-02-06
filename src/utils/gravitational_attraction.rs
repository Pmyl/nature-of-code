use nannou::geom::Point2;
use super::body::Body;
use nannou::prelude::Pow;

pub struct GravitationalAttraction;

impl GravitationalAttraction {
    pub fn apply_to(self, body: &mut Body, attractor: &Body) {
        body.apply_force(self.calculate_force(body, attractor));
    }

    pub fn calculate_force(self, body: &Body, attractor: &Body) -> Point2 {
        let mover_mass = body.mass;
        let attractor_mass = attractor.mass;
        let distance = (body.position - attractor.position).length().clamp(5., 25.);
        let direction = (attractor.position - body.position).normalize();

        mover_mass * attractor_mass * direction / distance.pow(2)
    }
}
