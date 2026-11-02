fn main() {
    let mut flags = vec![false; 10];
    let mut i = 0;
    while i < 10 { flags[i] = i % 2 == 0; i += 1; }
    let mut c = 0;
    for f in &flags { if *f { c += 1; } }
    println!("{}", c);
}
