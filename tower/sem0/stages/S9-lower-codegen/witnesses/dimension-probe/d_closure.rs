fn main() { let add = |a: i32, b: i32| a+b; let v = vec![1,2,3]; let t: i32 = v.iter().map(|x| x*2).sum(); println!("{} {}", add(4,5), t); }
