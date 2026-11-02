fn classify(x: i64) -> i64 {
    if x < -1_000_000_000 { -1 }
    else if x < 0 { 0 }
    else if x < 1_000_000_000 { 1 }
    else { 2 }
}
fn main() {
    println!("{} {} {} {}", classify(-5_000_000_000), classify(-3), classify(500), classify(9_000_000_000));
}
