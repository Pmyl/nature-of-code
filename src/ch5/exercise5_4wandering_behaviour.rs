use nannou::color::{BLACK, BLUE};
use nannou::geom::pt2;
use nannou::App;
use nannou::Draw;
use nannou::math::Vec2Angle;
use nannou::prelude::GRAY;
use rand::{thread_rng, Rng};
use nature_of_code::utils::point::PointExt;
use nature_of_code::utils::vehicle::Vehicle;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    vehicle: Vehicle,
    r: f32,
    theta: f32,
    distance: f32,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            vehicle: Vehicle {
                position: pt2(300., 100.),
                max_force: 0.01,
                max_speed: 1.0,
                velocity: pt2(0., 0.1),
                ..Default::default()
            },
            r: 30.0,
            theta: 0.,
            distance: 80.0
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        let normalized = self.vehicle.velocity.normalize();
        let center_circle = self.vehicle.position + normalized * self.distance;
        draw.line()
            .start(self.vehicle.position)
            .end(center_circle)
            .color(GRAY);

        draw.ellipse()
            .xy(center_circle)
            .radius(self.r)
            .color(GRAY);

        let theta_rotated = self.vehicle.velocity.angle() + self.theta;
        let x = self.r * theta_rotated.cos();
        let y = self.r * theta_rotated.sin();
        let target = center_circle + pt2(x, y);

        draw.line()
            .start(center_circle)
            .end(target)
            .color(BLACK);

        draw.ellipse()
            .xy(target)
            .radius(3.0)
            .color(BLUE);

        draw.polygon()
            .points(vec![pt2(0.0, -4.0), pt2(10.0, 0.0), pt2(0.0, 4.0)].center_polygon())
            .xy(self.vehicle.position)
            .rotate(self.vehicle.velocity.y.atan2(self.vehicle.velocity.x));
    }

    fn step(&mut self, exercise_data: &ExerciseData) {
        let center_circle = self.vehicle.position + self.vehicle.velocity.normalize() * self.distance;
        let theta_rotated = self.vehicle.velocity.angle() + self.theta;
        let x = self.r * theta_rotated.cos();
        let y = self.r * theta_rotated.sin();
        let target = center_circle + pt2(x, y);

        self.vehicle.seek(target);
        self.vehicle.update();

        self.theta += thread_rng().gen_range(-0.1..0.1);

        if self.vehicle.position.x > exercise_data.size().x {
            self.vehicle.position.x -= exercise_data.size().x
        }

        if self.vehicle.position.y > exercise_data.size().y {
            self.vehicle.position.y -= exercise_data.size().y
        }

        if self.vehicle.position.x < 0. {
            self.vehicle.position.x += exercise_data.size().x
        }

        if self.vehicle.position.y < 0. {
            self.vehicle.position.y += exercise_data.size().y
        }
    }
}
