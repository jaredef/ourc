const X: u64 = !0 as u16 as u64;
const Y: u64 = !0 as u32 as u64;
const FIVE: i32 = 5;
const NAME: &str = "bob";
fn kind(n: i32) -> &'static str {
    match n {
        FIVE => "five",
        x if x > FIVE => "big",
        _ => "small",
    }
}
fn main() {
    let r = match 1u64 { X => 10, Y => 20, _ => 1 };
    let s = match 65535u64 { X => 10, Y => 20, _ => 1 };
    let t = match 4294967295u64 { X => 10, Y => 20, _ => 1 };
    let w = match "bob" { NAME => 1, _ => 0 };
    println!("{} {} {} {} {} {} {}", r, s, t, kind(5), kind(9), kind(2), w);
}
