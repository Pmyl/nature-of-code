use nannou::color::{BLACK, GREY};
use nannou::geom::pt2;
use nannou::App;
use nannou::Draw;
use nannou::Event;
use nature_of_code::utils::body::Body;
use nature_of_code::utils::event::MouseEvent;
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::liquid::Liquid;
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};
use std::array;

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    movers: Vec<Body>,
    liquid: Liquid,
    mouse_pressed: bool,
}

impl ExerciseState for State {
    fn new(exercise: &ExerciseData, _: &App) -> Self {
        State {
            movers: array::from_fn::<_, 10, _>(|index: usize| {
                Body::new_simple(
                    pt2(40. + index as f32 * 70., 20.),
                    thread_rng().gen_range(0.5..3.),
                )
            })
            .into_iter()
            .collect::<Vec<_>>(),
            liquid: Liquid {
                position: pt2(0., exercise.size().y / 2.),
                size: pt2(exercise.size().x, exercise.size().y / 2.),
                coefficient: 0.1,
            },
            mouse_pressed: false,
        }
    }

    fn handle_event(&mut self, event: Event, _exercise: &ExerciseData) {
        if let Some(pressed) = event.pressed_state_changed() {
            self.mouse_pressed = pressed;
        }
    }

    fn show(&self, draw: &Draw, _exercise: &ExerciseData) {
        draw.background().color(BLACK);

        draw.rect()
            .wh(self.liquid.size)
            .xy(self.liquid.position + self.liquid.size / 2.)
            .color(GREY);

        for mover in self.movers.iter() {
            draw.ellipse().wh(mover.size).xy(mover.position);
        }
    }

    fn step(&mut self, exercise: &ExerciseData) {
        for mover in self.movers.iter_mut() {
            step_mover(&self.liquid, mover, &exercise);
        }
    }
}

fn step_mover(liquid: &Liquid, mover: &mut Body, exercise: &ExerciseData) {
    if liquid.contains(mover) {
        liquid.apply_drag_to(mover)
    }
    let gravity = Gravity(pt2(0., 0.1));
    gravity.apply_to(mover);

    mover.update();
    mover.bounce_edges(exercise.size(), 0.9);
}
