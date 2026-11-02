fn first(v: &Vec<i64>) -> &i64 { &v[0] } fn main() { let v = vec![42]; println!("{}", first(&v)); }
