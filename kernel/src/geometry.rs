pub struct Point {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Line {
    pub id: usize,
    pub start: Point,
    pub end: Point,
}

pub struct Plane {
    pub id: usize,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

pub struct Axis {
    pub id: usize,
    pub point: Point,
    pub normal: Plane,
}