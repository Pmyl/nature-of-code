pub mod utils;

use std::future;

use nannou::app::Builder;
use nannou::event::Update;
use nannou::geom::Point2;
use nannou::prelude::pt2;
use nannou::{App, Draw};
use nannou::{Event, Frame};

pub trait ExerciseState: 'static {
    fn new(exercise: &ExerciseData) -> Self;
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
    let draw = app
        .draw()
        .scale_x(data.exercise.scale as f32)
        .scale_y(-(data.exercise.scale as f32))
        .x_y(
            -(data.exercise.width as f32) / (data.exercise.scale as f32),
            -(data.exercise.height as f32) / (data.exercise.scale as f32),
        );
    data.state.show(&draw, &data.exercise);
    draw.to_frame(app, &frame).unwrap()
}

pub struct ExerciseDataWrapper<TState> {
    state: TState,
    exercise: ExerciseData,
}
pub struct ExerciseRunner;

impl ExerciseRunner {
    pub fn run<TState: ExerciseState>(exercise: ExerciseData) {
        let state = TState::new(&exercise);
        let width = exercise.width;
        let height = exercise.height;
        let scale = exercise.scale;
        Builder::new_async(move |app| {
            let _ = app
                .new_window()
                .size(width * scale, height * scale)
                .view(view::<TState>)
                .build()
                .unwrap();
            Box::new(future::ready(ExerciseDataWrapper { state, exercise }))
        })
        .update(update)
        .event(event)
        .run();
    }
}

pub struct ExerciseData {
    width: u32,
    height: u32,
    scale: u32,
}

impl ExerciseData {
    pub const fn new(width: u32, height: u32, scale: u32) -> Self {
        Self {
            width,
            height,
            scale,
        }
    }

    pub fn size(&self) -> Point2 {
        pt2(self.width as f32, self.height as f32)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn half_width(&self) -> f32 {
        self.width as f32 / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height as f32 / 2.0
    }

    pub fn half_position(&self) -> Point2 {
        pt2(self.width as f32 / 2.0, self.height as f32 / 2.0)
    }

    pub fn scale(&self) -> u32 {
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
