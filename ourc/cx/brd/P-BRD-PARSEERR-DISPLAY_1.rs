use std::num::ParseIntError;
fn p(s: &str) -> Result<i64, ParseIntError> { s.parse::<i64>() }
fn main() { let e = p("x").unwrap_err(); let f = p("").unwrap_err(); println!("{}", format!("[{}] [{}]", e, f)); }
