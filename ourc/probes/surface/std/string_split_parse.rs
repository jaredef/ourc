fn main() { let s = "3,4,5"; let v: Vec<i64> = s.split(",").map(|x| x.parse().unwrap()).collect(); println!("{}", v.iter().sum::<i64>()); }
