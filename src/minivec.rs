use std::f64::consts::PI;
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn magnitude(delta: Point) -> f64 {
    ((delta.x.powi(2)) + (delta.y.powi(2))).sqrt()
}

pub fn distance(p1: Point, p2: Point) -> Point {
    Point {
        x: (p1.x - p2.x),
        y: (p1.y - p2.y),
    }
}

pub fn angle(p1: Point) -> f64 {
    p1.y.atan2(p1.x)
}
