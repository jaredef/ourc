struct H { f: Box<dyn Fn(i64) -> i64> }
fn main() {
    let h = H { f: Box::new(|x| x - 7) };
    println!("{}", (h.f)(50));
}
