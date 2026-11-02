// RUSTC-FE sub-arc 01 (rustc-lexer) / LEX: Vec::get(index) -> Option<&T>. Was UNHANDLED for a Vec (only
// HashMap.get existed), so `*v.get(i).unwrap()` / `if let Some(x)=v.get(i)` segfaulted. rustc parsers
// hold slice cursors and index via .get() pervasively.
fn main() {
    let v: Vec<i32> = vec![10, 20, 30];
    println!("{}", *v.get(1).unwrap());
    match v.get(9) { Some(x) => println!("{}", *x), None => println!("none") }
    let mut s = 0; let mut i = 0; while i < 3 { s += *v.get(i).unwrap(); i += 1; } println!("{}", s);
}
