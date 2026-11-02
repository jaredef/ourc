use std::fmt::Debug;

trait Shape {
    fn area(&self) -> i32;
    fn name(&self) -> String;
    fn describe(&self) -> String { format!("{} has area {}", self.name(), self.area()) }
}

#[derive(Debug, Clone)]
struct Rect { w: i32, h: i32 }
#[derive(Debug, Clone)]
struct Square { s: i32 }

impl Shape for Rect {
    fn area(&self) -> i32 { self.w * self.h }
    fn name(&self) -> String { String::from("rect") }
}
impl Shape for Square {
    fn area(&self) -> i32 { self.s * self.s }
    fn name(&self) -> String { String::from("square") }
}

fn total_area<T: Shape>(shapes: &[T]) -> i32 {
    shapes.iter().map(|s| s.area()).sum()
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut m = items[0];
    for &x in items.iter() { if x > m { m = x; } }
    m
}

#[derive(Debug)]
enum Tree { Leaf(i32), Node(Box<Tree>, Box<Tree>) }
fn tree_sum(t: &Tree) -> i32 {
    match t { Tree::Leaf(v) => *v, Tree::Node(l, r) => tree_sum(l) + tree_sum(r) }
}

fn main() {
    let rects = vec![Rect { w: 2, h: 3 }, Rect { w: 4, h: 5 }];
    println!("{}", total_area(&rects));
    println!("{}", rects[0].describe());
    println!("{:?}", rects);

    let nums = [3, 7, 2, 9, 1];
    println!("{}", largest(&nums));

    let t = Tree::Node(Box::new(Tree::Leaf(3)), Box::new(Tree::Node(Box::new(Tree::Leaf(4)), Box::new(Tree::Leaf(5)))));
    println!("{}", tree_sum(&t));

    let words = vec!["hello", "world", "foo"];
    let long: Vec<&str> = words.iter().filter(|w| w.len() > 3).cloned().collect();
    println!("{:?} {}", long, long.join(","));

    let sq = Square { s: 6 };
    println!("{}", sq.describe());
}
