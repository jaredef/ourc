fn main() {
    let mut v = Some(5);
    let n = match v { Some(ref mut p) => *p + 1, None => 0 };
    println!("{}", n);
}
