use nannou::geom::Point2;
use super::body::Body;

pub struct Gravity(pub Point2);

impl Gravity {
    pub fn apply_to(self, mover: &mut Body) {
        mover.apply_force(self.0 * mover.mass);
    }
}
