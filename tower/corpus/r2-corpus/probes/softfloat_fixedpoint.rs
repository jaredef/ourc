// SOFTFLOAT: fixed-point f64 -- literal, int->f64 cast, f64 division, default + precision Display,
// and float->int truncation. All within the corpus magnitude range (operands < ~9.2 for division).
fn main() {
    let x = 3.14159f64;
    let n: i64 = 7;
    let q = n as f64 / 2.0;      // 3.5
    let f = 3.9_f64;
    println!("{:.2}|{:.0}|{}|{}|{}", x, x, q, f as i64, 2.05f64);
}
