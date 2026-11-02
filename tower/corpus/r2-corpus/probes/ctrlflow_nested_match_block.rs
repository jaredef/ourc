fn main() {
    let p = (1, 2);
    let r = match p.0 {
        1 => match p.1 { 2 => 42, _ => 0 },
        _ => -1,
    };
    println!("{}", r);
}
