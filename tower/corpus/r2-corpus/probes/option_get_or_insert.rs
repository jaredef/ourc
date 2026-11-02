// tractable: Option::get_or_insert(v) -> &mut payload; sets Some(v) in place if None.
fn main() {
    let mut a: Option<i32> = None;
    let v = a.get_or_insert(8);
    println!("{}", *v);              // 8
    let mut b: Option<i32> = Some(5);
    println!("{}", *b.get_or_insert(9)); // 5 (already Some)
    let mut c: Option<i32> = None;
    { let r = c.get_or_insert(3); *r += 10; }
    println!("{}", c.unwrap());      // 13 (write-through)
}
