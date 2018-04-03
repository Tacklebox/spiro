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
    match p1 {
        Point { x, y } if x.abs() < 1e-10 && y.abs() < 1e-10 => 0f64,
        Point { x, .. } if x == 0f64 => p1.y.signum() * PI / 2f64,
        Point { x, .. } if x > 0f64 => (p1.y / p1.x).atan(),
        Point { y, .. } if y >= 0f64 => (p1.y / p1.x).atan() + PI,
        Point { y, .. } if y < 0f64 => (p1.y / p1.x).atan() - PI,
        _ => panic!("Error converting point to angle!"),
    }
}
