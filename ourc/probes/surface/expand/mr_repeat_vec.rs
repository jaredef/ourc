macro_rules! v { ($($x:expr),* $(,)?) => { { let mut t = Vec::new(); $(t.push($x);)* t } }; } fn main() { let a = v![1, 2, 3,]; println!("{}", a.len()); }
