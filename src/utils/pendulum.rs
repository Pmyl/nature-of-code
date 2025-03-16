use nannou::geom::{pt2, Point2};
use std::f32::consts::PI;

pub struct Pendulum {
    pub r: f32,
    pub angle: f32,
    pub angular_velocity: f32, 
    pub angular_acceleration: f32,
    pub bob: Point2,
    pub pivot: Point2,
    pub damping: f32
}

impl Pendulum {
    pub fn new(position: Point2, r: f32) -> Self {
        Pendulum {
            r,
            angle: PI / 4.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            bob: Point2::ZERO,
            pivot: position,
            damping: 0.995,
        }
    }
    pub fn update(&mut self) {
        let gravity = 0.4;
        self.angular_acceleration = -1.0 * gravity * self.angle.sin() / self.r;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.angular_velocity*= self.damping;
        
        self.bob = pt2(self.angle.sin() * self.r, self.angle.cos() * self.r) + self.pivot;
    }
}