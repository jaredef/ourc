fn main() { let fs: Vec<Box<dyn Fn(i64) -> i64>> = vec![Box::new(|x| x + 1), Box::new(|x| x * 2)]; let mut s = 0; for f in fs.iter() { s += f(10); } println!("{}", s); }
