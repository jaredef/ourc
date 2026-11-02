fn main() { let o: Option<i64> = None; let v = if let Some(x) = o { x } else { 42 }; println!("{}", v); }
