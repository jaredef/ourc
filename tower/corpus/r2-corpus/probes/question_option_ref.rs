// tractable: `?` on Option<&T> (find/get/first) binds the ref payload -> `*x` no-op.
// Was: ? (and a `let x = ..?`) dropped the ref-ness -> *x derefed a value.
fn first_even(v: &[i64]) -> Option<i64> {
    let x = v.iter().find(|&&n| n % 2 == 0)?;
    Some(*x * 10)
}
fn main() {
    let v = vec![1i64, 3, 4, 5];
    println!("{}", first_even(&v).unwrap_or(-1));  // 40
    let e: Vec<i64> = vec![1, 3, 5];
    println!("{}", first_even(&e).unwrap_or(-1));  // -1
    // unwrap path too
    let w = vec![2, 4, 6];
    let y = w.iter().find(|&&n| n > 3).unwrap();
    println!("{}", *y);                            // 4
}
