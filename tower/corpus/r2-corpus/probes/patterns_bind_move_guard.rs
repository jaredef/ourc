fn main() {
    let opt = Some(15);
    let r = match opt { Some(n) if n > 10 => n - 10, Some(n) => n, None => 0 };
    println!("{}", r);
}
