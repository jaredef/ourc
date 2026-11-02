fn f(c: char) -> String { match c { x @ ('+' | '-') => format!("a{}", x), x @ ('*' | '/') => format!("m{}", x), _ => "o".to_string() } }
fn main() { println!("{} {} {}", f('-'), f('/'), f('x')); }
