// P-DEBUG: `{:?}` is emitted from the STATIC type -- the field and variant sets are known at compile
// time, so the printer is a flat sequence of prints with one tag test per enum variant. Debug QUOTES a
// char and a str where Display does not. String escapes are read by the lexer.
#[derive(Debug)]
struct Point { x: i32, y: i32 }
fn id(v: i32) -> i32 { v }
fn main() {
    println!("{:?} {:?} {:?} {:?}", id(1), true, 'a', "hi");
    println!("{:?}", vec![1, 2, 3]);
    println!("{:?}", vec!["a", "b"]);
    println!("{:?}", vec![vec![1], vec![2, 3]]);
    let a: Option<i32> = Some(id(1));
    let b: Option<i32> = None;
    println!("{:?} {:?}", a, b);
    println!("{:?}", Point { x: 1, y: 2 });
    println!("a\nb\tc\"d");
}
