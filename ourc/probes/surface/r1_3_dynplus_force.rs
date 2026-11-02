fn make() -> Box<dyn FnMut() -> i32 + Send + 'static> {
    let mut n = 0;
    Box::new(move || {
        n += 1;
        n
    })
}
fn unit() -> Box<dyn Fn() + 'static> {
    Box::new(|| ())
}
fn main() {
    let mut f = make();
    let g = unit();
    g();
    println!("{} {}", f(), f());
}
