fn main() {
    let t = (1, "x");
    match t {
        (1, s) => println!("one {}", s),
        (n, _) => println!("{}", n),
    }
    let (a, ref b) = (3, 4);
    let mut u = (5, 6);
    match u {
        (ref x, _) if *x > 4 => println!("big {}", x),
        _ => {}
    }
    u.0 = 7;
    println!("{} {} {:?}", a, b, u);
}
