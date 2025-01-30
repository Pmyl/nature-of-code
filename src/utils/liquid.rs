use nannou::geom::Point2;
use nannou::prelude::pt2;
use super::mover::Mover;

pub struct Liquid {
    pub position: Point2,
    pub size: Point2,
    pub coefficient: f32
}

impl Liquid {
    pub fn contains(self, mover: &Mover) -> bool {
        true
    }

    pub fn calculate_drag(self, mover: &Mover) -> Point2 {
        pt2(0., 0.)
    }
}
