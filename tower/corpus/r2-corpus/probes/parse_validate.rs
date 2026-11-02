// P-STRING-RESIDUAL: str::parse::<i32>() VALIDATES -> Err on a non-integer string
// (was: disc always Ok). __str_valid_int prelude helper (sign + all-digits).
fn main() {
    println!("{}", "abc".parse::<i32>().is_err());   // true
    println!("{}", "42".parse::<i32>().is_err());    // false
    println!("{}", "-5".parse::<i32>().is_err());    // false
    let a: i32 = "42".parse().unwrap();
    println!("{}", a);                               // 42
    let b: i32 = "xy".parse().unwrap_or(-1);
    println!("{}", b);                               // -1
}
