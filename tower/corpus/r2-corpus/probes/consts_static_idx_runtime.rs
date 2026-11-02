static T: [i64; 3] = [100, 200, 300];
fn f(i: usize) -> i64 { T[i] }
fn main() { println!("{}", f(2)); }
