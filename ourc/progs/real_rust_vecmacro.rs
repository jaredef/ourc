fn main() {
    let v = vec![10u32, 14, 18];
    let mut t = 0;
    for x in v { t += x; }
    println!("{}", t);
    let e: Vec<i64> = vec![];
    println!("{}", e.len());
    if cfg!(nosuchflag) { println!("yes"); } else { println!("no"); }
}
