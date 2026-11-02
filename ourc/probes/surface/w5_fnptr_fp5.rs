fn t() -> bool { true } fn main(){ let f: fn() -> bool = t; println!("{}", f()); }
