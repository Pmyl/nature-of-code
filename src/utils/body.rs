use nannou::geom::{pt2, Point2};

#[derive(Default)]
pub struct Body {
    pub position: Point2,
    pub velocity: Point2,
    pub acceleration: Point2,
    pub mass: f32,
    pub size: Point2,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
}

impl Body {
    pub fn new_body(position: Point2, mass: f32) -> Self {
        Body {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            mass,
            size: pt2(8., 8.) * mass.sqrt(),
            angle: 0.,
            angular_velocity: 0.,
            angular_acceleration: 0.,
        }
    }
    pub fn new_simple(position: Point2, mass: f32) -> Self {
        Body {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            mass,
            size: pt2(16., 16.) * mass,
            angle: 0.,
            angular_velocity: 0.,
            angular_acceleration: 0.,
        }
    }
    pub fn new(position: Point2, mass: f32, size: Point2) -> Self {
        Body {
            position,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
            mass,
            size,
            angle: 0.,
            angular_velocity: 0.,
            angular_acceleration: 0.,
        }
    }
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Point2::ZERO;

        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.angular_acceleration = 0.;
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
