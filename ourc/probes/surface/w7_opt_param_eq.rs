fn t(f: Option<i32>) -> i32 { if f == Some(7) { 0 } else if f == None { 2 } else { 9 } }
fn n(x: Option<i32>) -> bool { x != Some(3) }
fn r(x: Result<i32, i32>) -> bool { x == Ok(5) }
fn main() {
    let a = Some(4);
    let c: Option<char> = Some('x');
    println!("{} {} {} {} {} {} {} {}", t(Some(7)), t(Some(8)), t(None), n(Some(3)), n(Some(4)), r(Ok(5)), a == Some(3), c == Some('x'));
}
