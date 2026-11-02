enum Opt { Some(i32), None }
fn unwrap_or(o: Opt, d: i32) -> i32 { match o { Opt::Some(v) => v, Opt::None => d } }
fn main() {
    let a = unwrap_or(Opt::Some(42), 0);
    let b = unwrap_or(Opt::None, 7);
    std::process::exit(a + b);   // 42 + 7 = 49
}
