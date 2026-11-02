// The ourc tower gate's fixture: real Rust, exercising the surfaces each crate owns -- the lexer (literals,
// escapes, chars), the parser (structs, enums, generics, patterns, for/while/loop), the typechecker (Option,
// Result, Vec<T>, {:?}), monomorphization, and codegen (arithmetic widths, index checks, the exit code).
#[derive(Debug)]
enum Shape { Dot, Line(i64), Rect { w: i64, h: i64 } }

fn area(s: &Shape) -> i64 {
    match s { Shape::Dot => 0, Shape::Line(n) => *n, Shape::Rect { w, h } => w * h }
}

fn largest<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
    let mut best = v[0];
    for i in 1..v.len() { if v[i] > best { best = v[i]; } }
    best
}

fn half(n: i64) -> Option<i64> { if n % 2 == 0 { Some(n / 2) } else { None } }

fn main() {
    let shapes = vec![Shape::Dot, Shape::Line(7), Shape::Rect { w: 3, h: 5 }];
    let mut total = 0;
    for s in &shapes { total += area(s); }
    println!("total {}", total);
    println!("{:?}", shapes[2]);
    println!("largest {}", largest(&vec![4, 19, 12]));
    match half(total) { Some(h) => println!("half {}", h), None => println!("odd") }
    let mut i = 0;
    let found = loop { if i * i > total { break i; } i += 1; };
    println!("first sq over {} is {}", total, found);
    let s = "tower\tourc\n";
    println!("len {} first {}", s.len(), 'o');
    std::process::exit((total % 7) as i32);
}
