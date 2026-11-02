fn main() { let a = [1, 2, 3, 4, 5]; let s: i64 = a.iter().filter(|&&x| x % 2 == 1).map(|x| x * x).sum(); println!("{}", s); }
