use nannou::color::BLACK;
use nannou::geom::{pt2, Point2, Vec3};
use nannou::math::Vec2Angle;
use nannou::Draw;
use nature_of_code::utils::body::Body;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    body: Body,
    mouse: Body,
}

impl ExerciseState for State {
    fn new(_: &ExerciseData) -> Self {
        State {
            body: Body::new_body(pt2(320., 120.), 10.),
            mouse: Body::new_simple(Point2::ZERO, 10.),
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        if let Some(new_pos) = event.get_position(&exercise) {
            self.mouse.position = new_pos;
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        let rotated = draw
            .translate(Vec3::new(self.body.position.x, self.body.position.y, 0.))
            .rotate(self.body.angle);

        rotated
            .rect()
            .wh(pt2(self.body.size.x, self.body.size.x / 2.));
    }

    fn step(&mut self, _: &ExerciseData) {
        let mut direction = self.mouse.position - self.body.position;
        direction = direction.normalize();
        self.body.acceleration = direction * 0.1;
        self.body.velocity = self.body.velocity.clamp_length(0., 1.);

        self.body.angle = self.body.velocity.angle();

        self.body.update();
    }
}
