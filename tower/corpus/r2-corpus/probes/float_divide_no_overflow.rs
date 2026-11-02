// softfloat: fixed-point f64 division must not overflow i64 for dividend value > ~9.2. The naive (a*1e9)/b
// left a 1e18 intermediate; split into (a/b)*1e9 + ((a%b)*1e9)/b so only the divisor magnitude bounds it.
fn main() {
    let a = 10.0 / 4.0;               // 2.5 -- overflowed before (1e10*1e9)
    let c = 100.0;
    let f = c * 9.0 / 5.0 + 32.0;     // 212 -- classic C->F, big intermediate
    println!("{} {:.4} {}", a, 22.0 / 7.0, f);
}
