fn cl(a: char) -> i64 { match a { '#' => 1, 'a' => 2, _ => 0 } }
fn main() { println!("{} {} {}", cl('#'), cl('a'), cl('z')); }
