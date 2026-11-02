fn counter<'a>(v: &'a Vec<i32>) -> impl FnMut() -> i32 + 'a {
    let mut i = 0;
    move || {
        i += 1;
        v[i - 1]
    }
}
fn main() {
    let v = vec![5, 6, 7];
    let mut c = counter(&v);
    println!("{} {}", c(), c());
}
