use nannou::color::{BLACK, BLUE, BROWN};
use nannou::geom::pt2;
use nannou::geom::Point2;
use nannou::App;
use nannou::Draw;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::liquid::Liquid;
use nature_of_code::utils::point::PointExt;
use nature_of_code::utils::vehicle::Vehicle;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    vehicle: Vehicle,
    water: Liquid,
    sand: Liquid,
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
            water: Liquid {
                position: pt2(200., 0.),
                size: pt2(100., 240.),
                coefficient: 0.,
            },
            sand: Liquid {
                position: pt2(400., 0.),
                size: pt2(240., 240.),
                coefficient: 0.,
            },
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        if let Some(pos) = event.get_position(exercise) {
            self.mouse_position = pos;
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);
        draw.rect()
            .wh(self.water.size)
            .xy(self.water.position + self.water.size / 2.)
            .color(BLUE);

        draw.rect()
            .wh(self.sand.size)
            .xy(self.sand.position + self.sand.size / 2.)
            .color(BROWN);

        let points = vec![pt2(0.0, -4.0), pt2(10.0, 0.0), pt2(0.0, 4.0)].center_polygon();

        draw.polygon()
            .points(points)
            .xy(self.vehicle.position)
            .rotate(self.vehicle.velocity.y.atan2(self.vehicle.velocity.x));
    }

    fn step(&mut self, _: &ExerciseData) {
        if self.water.contains_position(self.vehicle.position) {
            self.vehicle.max_speed = 0.5;
            self.vehicle.max_force = 0.01;
        } else if self.sand.contains_position(self.vehicle.position) {
            self.vehicle.max_speed = 2.0;
            self.vehicle.max_force = 0.03;
        } else {
            self.vehicle.max_speed = 2.0;
            self.vehicle.max_force = 0.1;
        }

        self.vehicle.seek(self.mouse_position);
        self.vehicle.update();
    }
}
