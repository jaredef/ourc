fn first_even(v: &[i64]) -> Option<i64> { let x = v.iter().find(|x| **x % 2 == 0)?; Some(*x * 10) } fn main() { println!("{:?} {:?}", first_even(&[1, 4]), first_even(&[1, 3])); }
