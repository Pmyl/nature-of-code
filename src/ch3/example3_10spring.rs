use nannou::color::{BLACK, WHITE};
use nannou::geom::{pt2, Point2};
use nannou::Draw;
use nature_of_code::utils::bob::Bob;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::spring::Spring;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    mouse_position: Point2,
    mouse_pressed: bool,
    bob: Bob,
    spring: Spring,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData) -> Self {
        State {
            mouse_position: Point2::ZERO,
            mouse_pressed: false,
            bob: Bob::new_resting(pt2(exercise.half_width(), 100.0), 24.0),
            spring: Spring {
                anchor_position: pt2(exercise.half_width(), 10.0),
                rest_length: 100.0,
            },
        }
    }

    fn handle_event(&mut self, event: nannou::Event, exercise: &ExerciseData) {
        self.mouse_position = event.get_position(&exercise).unwrap_or(self.mouse_position);
        self.mouse_pressed = event.pressed_state_changed().unwrap_or(self.mouse_pressed);
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        draw.ellipse()
            .wh(Point2::ONE * self.bob.mass * 2.0)
            .xy(self.bob.position);

        draw.ellipse()
            .wh(Point2::ONE * 10.0)
            .xy(self.spring.anchor_position);

        draw.line()
            .color(WHITE)
            .start(self.spring.anchor_position)
            .end(self.bob.position);
    }

    fn step(&mut self, _: &ExerciseData) {
        Gravity(pt2(0.0, 0.2)).apply_to_bob(&mut self.bob);

        self.bob.update();
        self.bob.velocity *= 0.98;

        self.spring.connect(&mut self.bob);
        self.spring.constrain_length(&mut self.bob, 30.0, 200.0);

        if self.mouse_pressed {
            self.bob.position = self.mouse_position;
        }
    }
}
