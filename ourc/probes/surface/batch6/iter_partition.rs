fn main() { let v = vec![1, 2, 3, 4, 5, 6]; let (ev, od): (Vec<i64>, Vec<i64>) = v.iter().partition(|&&x| x % 2 == 0); println!("{:?} {:?}", ev, od); }
