use nannou::color::Rgba;
use nannou::geom::pt2;
use nannou::noise::{NoiseFn, Perlin};
use nannou::{event::Update, App, Draw, Frame};
use nature_of_code::{map_range, Exercise};

const EXERCISE: Exercise = Exercise::new(400, 400, 2);

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> State {
    EXERCISE.init_with_view(app, view);
    State {
        width: EXERCISE.width(),
        height: EXERCISE.height(),
        noise: Perlin::new()
    }
}

fn update(_app: &App, walker: &mut State, _update: Update) {
    walker.step();
}

fn view(app: &App, walker: &State, frame: Frame) {
    let draw = EXERCISE.draw(app);
    walker.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct State {
    width: u32,
    height: u32,
    noise: Perlin,
}

impl State {
    pub fn show(&self, draw: &Draw) {
        for x in 0..self.width {
            for y in 0..self.height {
                let noise = self.noise.get([x as f64 / 10.0, y as f64 / 10.0]);
                let mapped_noise = map_range(noise as f32, (-1., 1.), (0., 1.));

                draw.line()
                    .x(x as f32)
                    .y(y as f32)
                    .color(Rgba::new(mapped_noise, mapped_noise, mapped_noise, 1.))
                    .stroke_weight(1.0)
                    .points(pt2(0., 0.), pt2(0., 1.));
            }
        }
    }

    pub fn step(&mut self) {
        //
    }
}
