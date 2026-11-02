fn main() {
    let v: Vec<i64> = vec![10, 20, 30];
    let a = *v.last().unwrap();
    let b = v.last().copied().unwrap();
    let mut s = a + b;
    let e: Vec<i64> = vec![];
    if e.last().is_none() { s += 1; }
    println!("{}", s);
}
