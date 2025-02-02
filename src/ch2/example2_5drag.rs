use nannou::color::{BLACK, GREY};
use nannou::event::WindowEvent;
use nannou::geom::pt2;
use nannou::Event;
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::utils::gravity::Gravity;
use nature_of_code::utils::liquid::Liquid;
use nature_of_code::utils::mover::Mover;
use nature_of_code::Exercise;
use rand::{thread_rng, Rng};
use std::array;

const EXERCISE: Exercise = Exercise::new(640, 240, 2);

pub fn run() {
    nannou::app(model).update(update).event(event).run();
}

fn event(_app: &App, state: &mut State, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(wevent),
            ..
        } => match wevent {
            WindowEvent::MousePressed(_) => {
                state.mouse_pressed = true;
            }
            WindowEvent::MouseReleased(_) => {
                state.mouse_pressed = false;
            }
            _ => {}
        },
        _ => {}
    }
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        movers: array::from_fn::<_, 10, _>(|index: usize| Mover::new_simple(
            pt2(40. + index as f32 * 70., 20.),
            thread_rng().gen_range(0.5..3.)
        )).into_iter().collect::<Vec<_>>(),
        liquid: Liquid {
            position: pt2(0., EXERCISE.size().y / 2.),
            size: pt2(EXERCISE.size().x, EXERCISE.size().y / 2.),
            coefficient: 0.1
        },
        mouse_pressed: false,
    }
}

fn update(_app: &App, state: &mut State, _update: Update) {
    state.step();
}

fn view(app: &App, state: &State, frame: Frame) {
    let draw = EXERCISE.draw(app);
    state.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct State {
    movers: Vec<Mover>,
    liquid: Liquid,
    mouse_pressed: bool,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(BLACK);

        draw.rect()
            .wh(self.liquid.size)
            .xy(self.liquid.position + self.liquid.size / 2.)
            .color(GREY);

        for mover in self.movers.iter() {
            draw.ellipse()
                .wh(mover.size)
                .xy(mover.position);
        }
    }

    pub fn step(&mut self) {
        for mover in self.movers.iter_mut() {
            Self::step_mover(&self.liquid, mover);
        }
    }

    pub fn step_mover(liquid: &Liquid, mover: &mut Mover) {
        if liquid.contains(mover) {
            liquid.apply_drag_to(mover)
        }
        let gravity = Gravity(pt2(0., 0.1));
        gravity.apply_to(mover);

        mover.update();

        mover.bounce_edges(EXERCISE.size(), 0.9);
    }
}
