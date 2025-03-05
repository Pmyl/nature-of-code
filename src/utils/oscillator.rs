use nannou::geom::Point2;

pub struct Oscillator {
    pub angle: Point2,
    pub angular_velocity: Point2,
}

impl Oscillator {
    pub fn new(angle: Point2, angular_velocity: Point2) -> Self {
        Self {
            angle,
            angular_velocity,
        }
    }

    pub fn update(&mut self) {
        self.angle += self.angular_velocity;
    }
}
