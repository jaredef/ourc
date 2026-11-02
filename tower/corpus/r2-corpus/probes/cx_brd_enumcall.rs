// cx_brd_enumcall: `f().iter().enumerate()` over a CALL receiver types its elements from the return type (rung364)
fn names() -> Vec<String> { vec!["a".to_string(), "bc".to_string()] }
fn main() { for (i, s) in names().iter().enumerate() { println!("{}: {} {}", i, s, s.len()); } }
