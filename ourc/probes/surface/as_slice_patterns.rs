fn f(v: &Vec<i64>) -> i64 { match v.as_slice() { [] => 0, [x] => *x, [a, b] if a + b == 7 => 77, [a, .., z] => a * 10 + z } }
fn main() { let mut m = vec![3, 1, 2]; m.as_mut_slice().sort(); println!("{} {} {} {} {} {:?}", f(&vec![]), f(&vec![4]), f(&vec![3, 4]), f(&vec![1, 2, 3]), vec![5, 6].as_slice().len(), m); }
