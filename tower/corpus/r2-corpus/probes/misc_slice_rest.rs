fn main() { let a = [10, 20, 30, 40]; let [head, rest @ ..] = a; println!("{}", head + rest.len() as i64); }
