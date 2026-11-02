enum Opt { Some(i32), None }
fn get_or(o: Opt, d: i32) -> i32 { match o { Opt::Some(v) => v, Opt::None => d } }
fn pick<T>(c: bool, a: T, b: T) -> T { if c { a } else { b } }
fn main() { let x = pick(true, get_or(Opt::Some(7), 0), get_or(Opt::None, 99)); std::process::exit(x + pick(false, 1, 2)); }