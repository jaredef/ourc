use std::fmt;

const LIMIT: i32 = 100;

struct Point { x: i32, y: i32 }

enum Shape { Dot, Line(i32), Rect { w: i32, h: i32 } }

trait Area {
    fn area(&self) -> i32;
    fn name(&self) -> i32 { 0 }
}

impl Area for Point {
    fn area(&self) -> i32 { self.x * self.y }
}

fn classify(s: &Shape) -> i32 {
    match s {
        Shape::Dot => 0,
        Shape::Line(n) => *n,
        Shape::Rect { w, h } => w * h,
    }
}

fn sum(v: &[i32]) -> i32 {
    let mut total = 0;
    for x in v {
        total += *x;
    }
    total
}

fn main() {}
