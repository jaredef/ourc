// P-FLOAT R1: {:e} scientific notation on integers (no float -- mantissa + exponent).
fn main() {
    println!("{:e}", 1000);   // 1e3
    println!("{:e}", 1500);   // 1.5e3
    println!("{:e}", 42);     // 4.2e1
    println!("{:e}", 1005);   // 1.005e3
    println!("{:e}", 0);      // 0e0
    println!("{:e}", 7);      // 7e0
}
