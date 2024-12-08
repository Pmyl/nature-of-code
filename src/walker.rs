use nannou::{color::WHITE, event::Update, geom::pt2, rand::random_f32, App, Draw, Event, Frame};
use nannou::event::WindowEvent;
use nature_of_code::Exercise;

const EXERCISE: Exercise = Exercise::new(300, 300, 2);

pub fn run() {
    nannou::app(model).update(update).event(event).run();
}

fn event(_app: &App, walker: &mut Walker, event: Event) {
    match event {
        Event::WindowEvent { simple:Some(wevent), ..} => {
            match wevent {
                WindowEvent::MouseMoved(mouse) => walker.mouse_position = (
                    mouse.x + (EXERCISE.width() / 2 * EXERCISE.scale()) as f32,
                    mouse.y - (EXERCISE.height() / 2 * EXERCISE.scale()) as f32
                ),
                _ => {}
            }
        }
        _ => {}
    }
}

fn model(app: &App) -> Walker {
    EXERCISE.init_with_view(app, view);
    Walker::new(
        EXERCISE.width(),
        EXERCISE.height(),
        MouseTendencyWalkerStrategy {fallback: UniformWalkerStrategy},
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
    position: (f32, f32),
    step_strategy: Box<dyn WalkerStrategy>,
    mouse_position: (f32, f32),
}

impl Walker {
    pub fn new(width: u32, height: u32, strategy: impl WalkerStrategy + 'static) -> Self {
        Self {
            position: (width as f32 / 2., height as f32 / 2.),
            step_strategy: Box::new(strategy),
            mouse_position: (0., 0.)
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.line()
            .x(self.position.0)
            .y(self.position.1)
            .color(WHITE)
            .stroke_weight(1.0)
            .points(pt2(0., 0.), pt2(0., 1.));
    }

    pub fn step(&mut self) {
        let (x, y) = self.step_strategy.step(&self.mouse_position, &self.position);
        self.position.0 += x;
        self.position.1 += y;
    }
}

trait WalkerStrategy {
    fn step(&self, mouse_position: &(f32, f32), position: &(f32, f32)) -> (f32, f32);
}

struct UniformWalkerStrategy;
impl WalkerStrategy for UniformWalkerStrategy {
    fn step(&self, _: &(f32, f32), _: &(f32, f32)) -> (f32, f32) {
        let xstep = f32::floor((random_f32() * 3.) - 1.);
        let ystep = f32::floor((random_f32() * 3.) - 1.);

        (xstep, ystep)
    }
}

struct RightDownTendencyWalkerStrategy;
impl WalkerStrategy for RightDownTendencyWalkerStrategy {
    fn step(&self, _: &(f32, f32), _: &(f32, f32)) -> (f32, f32) {
        let xstepf = f32::floor((random_f32() * 3.) - 1.);
        let ystepf = f32::floor((random_f32() * 3.) - 1.);

        let xstep_leaning = (xstepf + 0.05).min(1.0);
        let ystep_leaning = (ystepf + 0.05).min(1.0);

        (xstep_leaning, ystep_leaning)
    }
}


struct RightTendencyWalkerStrategy;
impl WalkerStrategy for RightTendencyWalkerStrategy {
    fn step(&self, _: &(f32, f32), _: &(f32, f32)) -> (f32, f32) {
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

struct MouseTendencyWalkerStrategy<T> {
    fallback: T
}
impl<T:WalkerStrategy> WalkerStrategy for MouseTendencyWalkerStrategy<T> {
    fn step(&self, mouse_position: &(f32, f32), position: &(f32, f32)) -> (f32, f32) {
        let rand = random_f32();

        println!("{:?}, {:?}", position, mouse_position);
        if rand < 0.5 {
          self.fallback.step(&mouse_position, &position)
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


