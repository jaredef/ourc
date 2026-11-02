fn main() { let v: Vec<i64> = (1..=5).collect(); let m = v.iter().max().unwrap(); let p = v.iter().position(|&x| x == 3); println!("{} {:?}", m, p); }
