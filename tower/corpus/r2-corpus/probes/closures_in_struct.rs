struct Holder<F: Fn(i64) -> i64> { f: F }
fn main() {
    let h = Holder { f: |x| x + 100 };
    println!("{}", (h.f)(23));
}
