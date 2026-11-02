fn max_by<T: Copy, F: Fn(T) -> i64>(items: &[T], f: F) -> T {
    let mut best = items[0];
    let mut bv = f(items[0]);
    let mut i = 1;
    while i < items.len() {
        let v = f(items[i]);
        if v > bv { bv = v; best = items[i]; }
        i += 1;
    }
    best
}
fn main() {
    let xs = [3, -7, 5, -2, 4];
    let m = max_by(&xs, |x| if x < 0 { -x } else { x });
    println!("{}", m);
}
