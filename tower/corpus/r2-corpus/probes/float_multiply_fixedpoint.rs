// P-TY-LITERAL-DEFAULT / softfloat: fixed-point f64 multiply must rescale by 1e9 (a*b is 1e18-scaled)
// and survive products > i64max/1e18 without overflow. Both operand-var and inline-literal forms.
fn main() {
    let x = 3.5;
    let y = x * 2.0;               // small product
    let r = 2.0;
    let area = 3.14159 * r * r;    // product > 9.2 (overflowed the naive (a*b)/1e9)
    let big = 100.0 * 100.0;       // 10000
    println!("{} {:.2} {}", y, area, big);
}
