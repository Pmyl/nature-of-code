use nannou::{
    event::WindowEvent,
    geom::{pt2, Point2},
    Event,
};

use crate::ExerciseData;

pub trait MouseEvent {
    fn get_position(&self, exercise: &ExerciseData) -> Option<Point2>;
    fn pressed_state_changed(&self) -> Option<bool>;
}

impl MouseEvent for Event {
    fn get_position(&self, exercise: &ExerciseData) -> Option<Point2> {
        match self {
            Event::WindowEvent {
                simple: Some(wevent),
                ..
            } => match wevent {
                WindowEvent::MouseMoved(mouse) => Some(pt2(
                    mouse.x / exercise.scale() + exercise.half_width(),
                    -mouse.y / exercise.scale() + exercise.half_height(),
                )),
                _ => None,
            },
            _ => None,
        }
    }

    fn pressed_state_changed(&self) -> Option<bool> {
        match self {
            Event::WindowEvent {
                simple: Some(wevent),
                ..
            } => match wevent {
                WindowEvent::MousePressed(_) => Some(true),
                WindowEvent::MouseReleased(_) => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
}
