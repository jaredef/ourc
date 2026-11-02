// R2a P4 guard: eprint!/eprintln! go to stderr, print!/println! to stdout, in order (surface compares stdout).
fn f(v: i32) -> i32 {
    if v < 0 {
        eprintln!("neg {}", v);
        return 0;
    }
    v
}
fn main() {
    eprint!("a");
    eprintln!();
    print!("p");
    println!("{} {}", f(-2), f(3));
    for i in 0..3 {
        if i % 2 == 0 { eprintln!("e{}", i); } else { println!("o{}", i); }
    }
}
