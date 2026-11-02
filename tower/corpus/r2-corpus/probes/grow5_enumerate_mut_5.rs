fn main() {
    let mut v = vec![10, 20, 30];
    for (i, x) in v.iter_mut().enumerate() {
        *x += i as i32;
    }
    println!("{}", v[0] + v[1] + v[2]);
}
