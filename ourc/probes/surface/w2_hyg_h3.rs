macro_rules! m { ($e:expr) => {{ let f = |y: i64| y * 2; f($e) }} } fn main(){ let y = 5; println!("{}", m!(y)); }
