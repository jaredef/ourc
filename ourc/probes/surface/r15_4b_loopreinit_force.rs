struct Tok {
    v: i32,
}
fn stop() -> ! {
    std::process::exit(7)
}
fn main() {
    let mut t = Tok { v: 1 };
    let mut sum = 0;
    let mut i = 0;
    loop {
        if i > 4 {
            break;
        }
        i += 1;
        let u = t;
        sum += u.v;
        if i < 9 {
            t = Tok { v: u.v + 1 };
            continue;
        }
        stop();
    }
    println!("{}", sum);
}
