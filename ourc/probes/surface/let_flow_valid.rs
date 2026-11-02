fn pick(n: i32) -> i32 {
    let r;
    if n > 2 {
        r = 10;
    } else if n > 0 {
        r = 5;
    } else {
        return -1;
    }
    r
}
fn by_match(n: i32) -> i32 {
    let r: i32;
    match n {
        0 => r = 7,
        1 => r = 8,
        _ => panic!("no"),
    }
    r
}
fn main() {
    let a;
    loop {
        a = 3;
        break;
    }
    let mut b;
    b = 1;
    b = b + a;
    let c: i32;
    let d = 4;
    c = d * 2;
    println!("{} {} {} {} {} {}", pick(3), pick(1), pick(0), by_match(1), b, c);
    let x;
    let ok = true;
    if ok { x = 1; } else { x = 2; }
    println!("{}", x);
}
