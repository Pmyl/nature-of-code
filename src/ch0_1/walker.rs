use nannou::event::WindowEvent;
use nannou::noise::{NoiseFn, OpenSimplex, Seedable};
use nannou::prelude::Pow;
use nannou::{color::WHITE, event::Update, geom::pt2, rand::random_f32, App, Draw, Event, Frame};
use nature_of_code::{map_range, Exercise};
use rand::{thread_rng, Rng};

const EXERCISE: Exercise = Exercise::new(300, 300, 2);

pub fn run() {
    nannou::app(model).update(update).event(event).run();
}

fn event(_app: &App, walker: &mut Walker, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(wevent),
            ..
        } => match wevent {
            WindowEvent::MouseMoved(mouse) => {
                walker.context.mouse_position = (
                    mouse.x / EXERCISE.scale() as f32 + (EXERCISE.width() / 2) as f32,
                    -mouse.y / EXERCISE.scale() as f32 + (EXERCISE.height() / 2) as f32,
                )
            }
            _ => {}
        },
        _ => {}
    }
}

fn model(app: &App) -> Walker {
    EXERCISE.init_with_view(app, view);
    Walker::new(
        EXERCISE.width(),
        EXERCISE.height(),
        NoiseWalkerStrategy {
            noise: OpenSimplex::new().set_seed(987654),
        },
    )
}

fn update(_app: &App, walker: &mut Walker, _update: Update) {
    walker.step();
}

fn view(app: &App, walker: &Walker, frame: Frame) {
    let draw = EXERCISE.draw(app);

    walker.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct Walker {
    step_strategy: Box<dyn WalkerStrategy>,
    context: Context,
}

struct Context {
    frames: usize,
    position: (f32, f32),
    mouse_position: (f32, f32),
}

impl Walker {
    pub fn new(width: u32, height: u32, strategy: impl WalkerStrategy + 'static) -> Self {
        Self {
            step_strategy: Box::new(strategy),
            context: Context {
                frames: 0,
                position: (width as f32 / 2., height as f32 / 2.),
                mouse_position: (0., 0.),
            },
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.line()
            .x(self.context.position.0)
            .y(self.context.position.1)
            .color(WHITE)
            .stroke_weight(1.0)
            .points(pt2(0., 0.), pt2(0., 1.));
    }

    pub fn step(&mut self) {
        let (x, y) = self.step_strategy.step(&self.context);
        self.context.position.0 += x;
        self.context.position.1 += y;
        self.context.frames += 1;
    }
}

trait WalkerStrategy {
    fn step(&self, context: &Context) -> (f32, f32);
}

#[allow(dead_code)]
struct UniformWalkerStrategy;
impl WalkerStrategy for UniformWalkerStrategy {
    fn step(&self, _: &Context) -> (f32, f32) {
        let xstep = f32::floor((random_f32() * 3.) - 1.);
        let ystep = f32::floor((random_f32() * 3.) - 1.);

        (xstep, ystep)
    }
}

#[allow(dead_code)]
struct RightDownTendencyWalkerStrategy;
impl WalkerStrategy for RightDownTendencyWalkerStrategy {
    fn step(&self, _: &Context) -> (f32, f32) {
        let xstepf = f32::floor((random_f32() * 3.) - 1.);
        let ystepf = f32::floor((random_f32() * 3.) - 1.);

        let xstep_leaning = (xstepf + 0.05).min(1.0);
        let ystep_leaning = (ystepf + 0.05).min(1.0);

        (xstep_leaning, ystep_leaning)
    }
}

#[allow(dead_code)]
struct RightTendencyWalkerStrategy;
impl WalkerStrategy for RightTendencyWalkerStrategy {
    fn step(&self, _: &Context) -> (f32, f32) {
        let rand = random_f32();

        if rand < 0.4 {
            (1., 0.)
        } else if rand < 0.6 {
            (0., 1.)
        } else if rand < 0.8 {
            (0., -1.)
        } else {
            (-1., 0.)
        }
    }
}

#[allow(dead_code)]
struct MouseTendencyWalkerStrategy<T> {
    fallback: T,
}
impl<T: WalkerStrategy> WalkerStrategy for MouseTendencyWalkerStrategy<T> {
    fn step(&self, context: &Context) -> (f32, f32) {
        let rand = random_f32();
        let mouse_position = context.mouse_position;
        let position = context.position;

        if rand < 0.8 {
            self.fallback.step(&context)
        } else {
            let xmovement = if position.0 < mouse_position.0 {
                1.
            } else if position.0 > mouse_position.0 {
                -1.
            } else {
                0.
            };

            let ymovement = if position.1 < mouse_position.1 {
                1.
            } else if position.1 > mouse_position.1 {
                -1.
            } else {
                0.
            };

            (xmovement, ymovement)
        }
    }
}

#[allow(dead_code)]
struct LevyFlightWalkerStrategy;
impl WalkerStrategy for LevyFlightWalkerStrategy {
    fn step(&self, _: &Context) -> (f32, f32) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < 0.01 {
            let xstep = rng.gen_range(-100.0..=100.0);
            let ystep = rng.gen_range(-100.0..=100.0);

            (xstep, ystep)
        } else {
            let xstep = rng.gen_range(-1.0..=1.0);
            let ystep = rng.gen_range(-1.0..=1.0);

            (xstep, ystep)
        }
    }
}

#[allow(dead_code)]
struct AcceptRejectSquaredWalkerStrategy;
impl WalkerStrategy for AcceptRejectSquaredWalkerStrategy {
    fn step(&self, _: &Context) -> (f32, f32) {
        let step = 10.0;

        fn accept_reject(step: f32) -> f32 {
            let mut rng = thread_rng();

            loop {
                let direction_step = rng.gen_range(-step..=step);
                let qualifying_random = rng.gen_range(-(step.pow(2.0))..=step.pow(2.0));
                let p = direction_step.pow(2.0);

                if qualifying_random < p {
                    return direction_step;
                }
            }
        }

        let xstep = accept_reject(step);
        let ystep = accept_reject(step);

        (xstep, ystep)
    }
}

#[allow(dead_code)]
struct NoiseWalkerStrategy {
    noise: OpenSimplex,
}
impl WalkerStrategy for NoiseWalkerStrategy {
    fn step(&self, context: &Context) -> (f32, f32) {
        let xstep = self.noise.get([context.frames as f64 / 100.0, 0.0]);
        let ystep = self
            .noise
            .get([context.frames as f64 / 100.0 + 10000.0, 0.0]);
        let step = 1.0;

        (
            map_range(xstep as f32, (-1.0, 1.0), (-step, step)),
            map_range(ystep as f32, (-1.0, 1.0), (-step, step)),
        )
    }
}
