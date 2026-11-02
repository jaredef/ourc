fn main() {
    let a = [10, 14, 18];
    let mut s = 0;
    for x in a { s += x; }
    println!("{}", s);
    let v = vec![1, 2, 39];
    let mut t = 0;
    for y in v { t += y; }
    println!("{}", t);
    let o = Some(7);
    match o { Some(_) => println!("some"), None => println!("none") }
}
