struct Counter { n: i32, step: i32 }
fn bump(c: &mut Counter) { c.n = c.n + c.step; }
fn main() {
    let mut c = Counter { n: 0, step: 5 };
    bump(&mut c);
    bump(&mut c);
    bump(&mut c);
    std::process::exit(c.n);   // 15
}
