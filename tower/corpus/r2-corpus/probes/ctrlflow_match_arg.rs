fn dbl(x: i32) -> i32 { x * 2 }
fn main() {
    let n = 2;
    println!("{}", dbl(match n { 0 => 5, _ => 21 }));
}
