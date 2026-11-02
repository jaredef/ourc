// softfloat: Rust's Display/{:.N} uses round-half-to-EVEN (banker's rounding), not half-up. On an exact tie
// (dropped remainder == half an ULP) with an even quotient, round DOWN: 2.5->2, 4.5->4, 0.25->0.2.
fn main() {
    println!("{:.0} {:.0} {:.0} {:.0}", 2.5, 3.5, 4.5, 5.5);   // 2 4 4 6
    println!("{:.1} {:.0} {:.0}", 0.25, 2.4, 2.6);             // 0.2 2 3
    println!("{:.2}", 3.14159);                                // 3.14 (non-tie unchanged)
}
