pub mod utils;

use std::future;

use nannou::app::Builder;
use nannou::event::Update;
use nannou::geom::Point2;
use nannou::prelude::pt2;
use nannou::{App, Draw};
use nannou::{Event, Frame};

pub trait ExerciseState: 'static {
    fn new(exercise: &ExerciseData, app: &App) -> Self;
    fn handle_event(&mut self, _event: Event, _exercise: &ExerciseData) {}
    fn show(&self, draw: &Draw, exercise: &ExerciseData);
    fn step(&mut self, exercise: &ExerciseData);
}

fn event<TState: ExerciseState>(_app: &App, data: &mut ExerciseDataWrapper<TState>, event: Event) {
    data.state.handle_event(event, &data.exercise);
}

fn update<TState: ExerciseState>(
    _app: &App,
    data: &mut ExerciseDataWrapper<TState>,
    _update: Update,
) {
    data.state.step(&data.exercise);
}

fn view<TState: ExerciseState>(app: &App, data: &ExerciseDataWrapper<TState>, frame: Frame) {
    data.draw.reset();
    data.state.show(&data.draw, &data.exercise);
    data.draw.to_frame(app, &frame).unwrap()
}

pub struct ExerciseDataWrapper<TState> {
    state: TState,
    exercise: ExerciseData,
    draw: Draw,
}
pub struct ExerciseRunner;

impl ExerciseRunner {
    pub fn run<TState: ExerciseState>(exercise: ExerciseData) {
        let width = exercise.width;
        let height = exercise.height;
        let scale = exercise.scale;
        Builder::new_async(move |app| {
            let _ = app
                .new_window()
                .size((width * scale) as u32, (height * scale) as u32)
                .view(view::<TState>)
                .build()
                .unwrap();
            let state = TState::new(&exercise, &app);

            let draw = Draw::new()
                .scale_x(scale)
                .scale_y(-scale)
                .x_y(-width / 2.0, -height / 2.0);
            Box::new(future::ready(ExerciseDataWrapper {
                state,
                exercise,
                draw,
            }))
        })
        .update(update)
        .event(event)
        .run();
    }
}

pub struct ExerciseData {
    width: f32,
    height: f32,
    half_width: f32,
    half_height: f32,
    scale: f32,
}

impl ExerciseData {
    pub fn new(width: u32, height: u32, scale: impl Into<f64>) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
            half_width: width as f32 / 2.0,
            half_height: height as f32 / 2.0,
            scale: Into::<f64>::into(scale) as f32,
        }
    }

    pub fn size(&self) -> Point2 {
        pt2(self.width, self.height)
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn half_width(&self) -> f32 {
        self.half_width
    }

    pub fn half_height(&self) -> f32 {
        self.half_height
    }

    pub fn half_position(&self) -> Point2 {
        pt2(self.half_width, self.half_height)
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

pub fn map_range(value: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let from_min = from_range.0;
    let from_max = from_range.1;
    let to_min = to_range.0;
    let to_max = to_range.1;

    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}
