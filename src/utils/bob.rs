use nannou::geom::Point2;

pub struct Bob {
    pub position: Point2,
    pub velocity: Point2,
    pub acceleration: Point2,

    pub mass: f32,
}

impl Bob {
    pub fn new_resting(position: Point2, mass: f32) -> Self {
        Self {
            position,
            mass,
            velocity: Point2::ZERO,
            acceleration: Point2::ZERO,
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
}
