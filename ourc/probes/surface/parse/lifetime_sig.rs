fn first<'a>(v: &'a [i64]) -> &'a i64 { &v[0] } fn main() { let v = [7, 8]; println!("{}", first(&v)); }
