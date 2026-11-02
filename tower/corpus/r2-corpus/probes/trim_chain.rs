// P-STRING-RESIDUAL: chained methods on a trim() sub-descriptor -- `.parse()`, `.len()`,
// `.parse().is_err()`. Was: trim() returned, so `.parse()` fell to a generic `call parse`.
fn main() {
    let s = "   42   ";
    let n: i32 = s.trim().parse().unwrap();
    println!("{}", n + 1);                       // 43
    println!("{}", "  hello  ".trim().len());    // 5
    println!("{}", " xx ".trim().parse::<i32>().is_err()); // true
}
