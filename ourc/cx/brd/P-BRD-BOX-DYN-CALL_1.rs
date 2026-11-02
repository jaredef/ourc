fn add(n: i64) -> Box<dyn Fn(i64) -> i64> { Box::new(move |x| x + n) }
fn main() { let f = add(10); println!("{}", f(1)); let v: Vec<Box<dyn Fn(i64) -> i64>> = vec![add(10)]; println!("{}", (v[0])(5)); }
