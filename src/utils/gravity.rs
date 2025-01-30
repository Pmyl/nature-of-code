use nannou::geom::Point2;
use super::mover::Mover;

pub struct Gravity(pub Point2);

impl Gravity {
    pub fn apply_to(self, mover: &mut Mover) {
        mover.apply_force(self.0 * mover.mass);
    }
}
