use nannou::geom::Point2;

#[derive(Default)]
pub struct Mover {
    pub position: Point2,
    pub velocity: Point2,
    pub acceleration: Point2,
    pub mass: f32,
}

impl Mover {
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Point2::ZERO;

        self.check_edges();
    }

    pub fn apply_force(&mut self, force: Point2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn check_edges(&mut self) {
        // if self.position.x < 0. || self.position.x > EXERCISE.width() as f32 {
        //     self.velocity.x *= -1.;
        //     self.acceleration.x *= -1.;
        // }

        // if self.position.y < 0. || self.position.y > EXERCISE.height() as f32 {
        //     self.velocity.y *= -1.;
        //     self.acceleration.y *= -1.;
        // }
    }
}
