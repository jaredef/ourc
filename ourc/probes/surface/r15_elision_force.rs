struct Store {
    v: i32,
}
impl Store {
    fn pick(&self, _f: fn()) -> &i32 {
        &self.v
    }
}
fn first(p: (&i32, bool)) -> &i32 {
    p.0
}
fn noop() {}
fn main() {
    let s = Store { v: 5 };
    let w = 9;
    println!("{} {}", s.pick(noop), first((&w, true)));
}
