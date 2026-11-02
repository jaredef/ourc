fn main() { let o = Some(2); let r = match o { Some(1 | 2) => 1, Some(_) => 2, None => 3 }; println!("{}", r); }
