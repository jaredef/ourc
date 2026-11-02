fn main() { let p = (1, 2); let r = match p { (0, _) => 0, (1, y) => y * 10, _ => 9 }; println!("{}", r); }
