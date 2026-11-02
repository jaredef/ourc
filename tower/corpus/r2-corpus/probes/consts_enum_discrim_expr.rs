const K: isize = 3;
enum E { A = K, B = K + 4 }
fn main() { println!("{}", E::B as i64); }
