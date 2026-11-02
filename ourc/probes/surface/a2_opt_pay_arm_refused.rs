fn main() { let o = Some(1); let x = match o { Some(v) => v, None => "" }; println!("{}", x); }
