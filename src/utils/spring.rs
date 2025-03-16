use nannou::geom::Point2;

use super::bob::Bob;

pub struct Spring {
    pub anchor_position: Point2,
    pub rest_length: f32,
}

impl Spring {
    pub fn connect(&self, bob: &mut Bob) {
        let force = bob.position - self.anchor_position;
        let current_length = force.length();
        let stretch = current_length - self.rest_length;

        let force = force.normalize() * stretch;

        let k = 0.2;

        bob.apply_force(-1.0 * k * force);
    }

    pub fn constrain_length(&self, bob: &mut Bob, min: f32, max: f32) {
        let mut direction = bob.position - self.anchor_position;
        let current_length = direction.length();

        if current_length < min {
            direction = direction.normalize() * min;
        } else if current_length > max {
            direction = direction.normalize() * max;
        } else {
            return;
        }

        bob.position = self.anchor_position + direction;
        bob.velocity = Point2::ZERO;
    }
}
