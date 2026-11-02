// P-BRD-PATRANGE-CONST: range-pattern bounds that are primitive constants (i32::MIN, char::MAX, u8::MAX).
fn bucket(n: i32) -> i32 { match n { i32::MIN..=-1 => 0, 0 => 1, 1..=i32::MAX => 2 } }
fn wide(c: char) -> i32 { match c { '\u{80}'..=char::MAX => 1, _ => 0 } }
fn top(b: u8) -> i32 { match b { 200..=u8::MAX => 1, _ => 0 } }
fn main() {
    println!("{} {} {}", bucket(-7), bucket(0), bucket(9));
    println!("{} {}", wide('a'), wide('é'));
    println!("{} {}", top(100), top(250));
}
