use nannou::geom::{pt2, Point2};

#[derive(Default)]
pub struct Mover {
    pub position: Point2,
    pub velocity: Point2,
    pub acceleration: Point2,
    pub mass: f32,
    pub size: Point2,
}

impl Mover {
    pub fn new_simple(position: Point2, mass: f32) -> Self {
        Mover {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            mass,
            size: pt2(16., 16.) * mass,
        }
    }
    pub fn new(position: Point2, mass: f32, size: Point2) -> Self {
        Mover {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            mass,
            size,
        }
    }
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Point2::ZERO;
    }

    pub fn apply_force(&mut self, force: Point2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    pub fn bounce_edges(&mut self, size: Point2, bounciness: f32) {
        if self.position.y - self.size.y / 2. < 0. {
            self.velocity.y *= -bounciness;
            self.position.y = self.size.y / 2.;
        }

        if self.position.x - self.size.x / 2. < 0. {
            self.velocity.x *= -bounciness;
            self.position.x = self.size.x / 2.;
        }

        if self.position.y > size.y - self.size.y / 2. {
            self.velocity.y *= -bounciness;
            self.position.y = size.y - self.size.y / 2.;
        }

        if self.position.x  > size.x - self.size.x / 2. {
            self.velocity.x *= -bounciness;
            self.position.x = size.x - self.size.x / 2.;
        }
    }

    pub fn contact_edges(&mut self, size: Point2) -> bool {
        if self.position.y - self.size.y / 2. < 0. {
          return true;
        }

        if self.position.x - self.size.x / 2. < 0. {
          return true;
        }

        if self.position.y > size.y - self.size.y / 2. {
          return true;
        }

        if self.position.x  > size.x - self.size.x / 2. {
          return true;
        }

        false
    }
}
