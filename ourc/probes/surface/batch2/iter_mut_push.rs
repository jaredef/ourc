fn main() { let mut v = vec![1, 2, 3]; for x in v.iter_mut() { *x *= 10; } v.push(40); println!("{:?} {}", v, v.iter().sum::<i64>()); }
