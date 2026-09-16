//! 金向量样例即九 kind 全覆盖与配平节点。

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

pub trait Area {
    fn area(&self) -> f64;
    fn name(&self) -> String {
        let s = format!("area={}", self.area() * 2.0);
        s
    }
}

impl Area for Point {
    fn area(&self) -> f64 {
        let m = vec![1, 2, 3].iter().fold(0, |acc, v| acc + v);
        self.x * self.y + (m as f64)
    }
}

const UNIT: f64 = 1.0;
static NAMES: &[&str] = &["a", "b"];

pub type Table = HashMap<String, Point>;

pub mod geometry {
    pub fn dist(p: Point, q: Point) -> f64 {
        let dx = p.x - q.x;
        (dx * dx + 4.0).sqrt()
    }

    struct Inner {
        v: u8,
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{} {}", p.x, geometry::dist(p, p));
}
