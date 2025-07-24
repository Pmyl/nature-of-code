use nannou::prelude::{pt2, BLACK};
use nannou::App;
use nannou::color::{Rgba, GREY};
use nannou::Draw;
use nannou::geom::Point2;
use rand::{thread_rng, Rng};
use nature_of_code::{utils::flow_field::FlowField, ExerciseData, ExerciseRunner, ExerciseState};

pub fn run() {
    ExerciseRunner::run::<State>(ExerciseData::new(640, 240, 2));
}

struct State {
    flow: FlowField,
}

impl ExerciseState for State {
    fn new(ex: &ExerciseData, _: &App) -> Self {
        State {
            flow: FlowField::new(
                20,
                ex.width() as usize,
                ex.height() as usize,
                |cols, rows| {
                    let mut vec = vec![pt2(1., 0.);cols * rows];

                    for i in 0..cols * rows {
                        let x = thread_rng().gen_range(-1.0..1.0);
                        let y = thread_rng().gen_range(-1.0..1.0);
                        vec[i] = pt2(x, y).normalize();
                    }

                    vec
                },
            ),
        }
    }

    fn show(&self, draw: &Draw, _: &ExerciseData) {
        draw.background().color(BLACK);

        for row in 0..self.flow.rows {
            for col in 0..self.flow.cols {
                let direction = self.flow.field[row * self.flow.cols + col];
                let position = pt2((col * self.flow.resolution) as f32, (row * self.flow.resolution) as f32);
                let center = position + self.flow.resolution as f32 / 2.;
                let start = center - direction * 10. / 2.;
                draw.rect().xy(position + self.flow.resolution as f32 / 2.)
                    .stroke(GREY)
                    .stroke_weight(1.)
                    .color(Rgba::new(0., 0., 0., 0.))
                    .wh(pt2(self.flow.resolution as f32, self.flow.resolution as f32));

                draw.arrow().start(start).end(start + direction * 10.);
            }
        }
    }

    fn step(&mut self, _: &ExerciseData) {
    }
}
