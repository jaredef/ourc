fn class(n: i64) -> &'static str { match n { i64::MIN..=-1 => "neg", 0 => "zero", 1..=9 => "small", x if x % 2 == 0 => "big-even", _ => "big-odd" } }
fn main() { for n in [-5, 0, 7, 12, 13] { print!("{} ", class(n)); } println!(); }
