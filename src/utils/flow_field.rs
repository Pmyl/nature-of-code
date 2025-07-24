use nannou::prelude::Point2;

pub struct FlowField {
    pub cols: usize,
    pub rows: usize,
    pub field: Vec<Point2>,
    pub resolution: usize
}

impl FlowField {
    pub fn new(resolution: usize, width: usize, height: usize, factory: impl FlowFieldFactory) -> FlowField {
        let cols = width / resolution;
        let rows =  height / resolution;
        FlowField {
            cols,
            rows,
            field: factory.create(cols, rows),
            resolution,
        }
    }
}

pub trait FlowFieldFactory {
    fn create(&self, cols: usize, rows: usize) -> Vec<Point2>;
}

impl<F> FlowFieldFactory for F where F: Fn(usize, usize) -> Vec<Point2> {
    fn create(&self, cols: usize, rows: usize) -> Vec<Point2> {
        self(cols, rows)
    }
}