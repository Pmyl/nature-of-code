use std::f32::consts::PI;

use nannou::color::{Rgba, GREY};
use nannou::geom::Point2;
use nannou::prelude::{pt2, BLACK};
use nannou::App;
use nannou::Draw;
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
                    let resolution = 20.0;
                    let width = ex.width();
                    let height = ex.height();
                    let center = pt2(width / 2., height / 2.);

                    let mut vec = vec![Point2::ZERO; cols * rows];

                    for col in 0..cols {
                        for row in 0..rows {
                            let i = row * cols + col;
                            let point = pt2(col as f32 * resolution, row as f32 * resolution);
                            let r = (center - point).length();
                            let point_adjusted = point - center;
                            // ROTATE
                            let angle = point_adjusted.y.atan2(point_adjusted.x) + PI / 2.;
                            let direction =
                                pt2(angle.cos() * r, angle.sin() * r).normalize_or_zero();
                            vec[i] = direction;
                        }
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
                let position = pt2(
                    (col * self.flow.resolution) as f32,
                    (row * self.flow.resolution) as f32,
                );
                let center = position + self.flow.resolution as f32 / 2.;
                let start = center - direction * 10. / 2.;
                draw.rect()
                    .xy(position + self.flow.resolution as f32 / 2.)
                    .stroke(GREY)
                    .stroke_weight(1.)
                    .color(Rgba::new(0., 0., 0., 0.))
                    .wh(pt2(
                        self.flow.resolution as f32,
                        self.flow.resolution as f32,
                    ));

                draw.arrow().start(start).end(start + direction * 10.);
            }
        }
    }

    fn step(&mut self, _: &ExerciseData) {}
}
