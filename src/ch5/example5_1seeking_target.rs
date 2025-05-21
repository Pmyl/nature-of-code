use nannou::color::BLACK;
use nannou::geom::pt2;
use nannou::geom::Point2;
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::point::PointExt;
use nature_of_code::utils::vehicle::Vehicle;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    vehicle: Vehicle,
    mouse_position: Point2,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData, _: &App) -> Self {
        State {
            vehicle: Vehicle {
                position: pt2(300., 100.),
                max_force: 0.1,
                max_speed: 2.0,
                ..Default::default()
            },
            mouse_position: Point2::ZERO,
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        if let Some(pos) = event.get_position(exercise) {
            self.mouse_position = pos;
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        let points = vec![pt2(0.0, -4.0), pt2(10.0, 0.0), pt2(0.0, 4.0)].center_polygon();

        draw.polygon()
            .points(points)
            .xy(self.vehicle.position)
            .rotate(self.vehicle.velocity.y.atan2(self.vehicle.velocity.x));
    }

    fn step(&mut self, _: &ExerciseData) {
        self.vehicle.seek(self.mouse_position);
        self.vehicle.update();
    }
}
