use nannou::geom::{pt2, Point2};

#[derive(Default)]
pub struct Vehicle {
    pub position: Point2,
    pub velocity: Point2,
    pub acceleration: Point2,
    pub max_speed: f32,
    pub max_force: f32,
    pub desired_direction: Point2,
}

impl Vehicle {
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = if self.velocity.length() > self.max_speed {
            self.velocity.normalize() * self.max_speed
        } else {
            self.velocity
        };
        self.position += self.velocity;
        self.acceleration = pt2(0., 0.);
    }

    pub fn seek(&mut self, target: Point2) {
        let mut desired = target - self.position;
        desired = desired.normalize() * self.max_speed;
        self.desired_direction = desired;

        let mut steer = desired - self.velocity;
        steer = steer.normalize() * self.max_force;

        self.apply_force(steer);
    }

    pub fn flee(&mut self, target: Point2) {
        let mut desired = target - self.position;
        desired = desired.normalize() * self.max_speed;

        let mut steer = desired - self.velocity;
        steer = steer.normalize() * self.max_force;

        self.apply_force(steer * -1.);
    }

    pub fn apply_force(&mut self, force: Point2) {
        self.acceleration += force;
    }
}
