fn main() { let a = [1, 2, 3]; for x in a.iter() { print!("{} ", x); } let n = a.iter().count(); let v: Vec<i64> = a.iter().map(|x| x * 2).collect(); println!("{} {:?} {}", n, v, a.len()); }
