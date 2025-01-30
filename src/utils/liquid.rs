use super::mover::Mover;
use nannou::geom::Point2;
use nannou::prelude::Pow;

pub struct Liquid {
    pub position: Point2,
    pub size: Point2,
    pub coefficient: f32
}

impl Liquid {
    pub fn contains(&self, mover: &Mover) -> bool {
        let pos = &self.position;
        let mpos = &mover.position;

        mpos.x > pos.x && mpos.x < pos.x + self.size.x
            && mpos.y > pos.y && mpos.y < pos.y + self.size.y
    }

    pub fn apply_drag_to(&self, mover: &mut Mover) {
        let drag_magnitude = self.coefficient * mover.velocity.length().pow(2);
        let drag_direction = mover.velocity.normalize() * -1.;
        let drag_force = drag_direction * drag_magnitude;
        mover.apply_force(drag_force);
    }
}
