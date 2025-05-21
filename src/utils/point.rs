use nannou::geom::{pt2, Point2};

pub trait PointExt {
    fn center_polygon(&self) -> Vec<Point2>;
}

impl PointExt for Vec<Point2> {
    fn center_polygon(&self) -> Vec<Point2> {
        let n = self.len() as f32;
        let centroid = self.iter().fold(pt2(0.0, 0.0), |acc, p| acc + *p) / n;
        self.iter().map(|p| *p - centroid).collect()
    }
}
