fn h() -> i32 { 3 }
fn k(n: i32) -> i32 {
    let mut t = 0;
    for _i in 0..n { t = t + h(); }
    if t > 0 { h() } else { k(t) }
}
