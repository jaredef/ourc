fn label(n: i32) -> &'static str {
    if n == 0 { "zero" } else if n == 1 { "one" } else if n == 2 { "two" } else { "many" }
}
fn main() { let mut i = 0; while i < 5 { println!("{} -> {}", i, label(i)); i = i + 1; } }
