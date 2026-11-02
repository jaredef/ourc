fn make() -> impl Iterator<Item = i64> {
    let mut i = 0i64;
    std::iter::from_fn(move || {
        i += 1;
        if i <= 3 { Some(i * 10) } else { None }
    })
}
fn main() {
    let mut sum = 0i64;
    for x in make() { sum += x; }
    println!("{}", sum);
}
