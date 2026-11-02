fn main() { let o = Some((4, 3)); let p: Option<(i64, &str)> = None; println!("{:?} {:?} {:?}", o, Some((1, "x")), p); }
