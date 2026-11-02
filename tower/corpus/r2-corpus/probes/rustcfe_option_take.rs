// RUSTC-FE sub-arc 03 (rustc-data-structures) / DS: Option::take() on a bound Option local -- return the
// old Option AND set the original to None. Was: returned the payload but left the original Some (o.is_none
// stayed false). rustc's OnDrop::disable does self.0.take(); .take() is pervasive in the resolver/parser.
fn main() {
    let mut o = Some(5);
    let t = o.take();
    println!("{} {}", t.unwrap(), o.is_none() as i32);   // 5 1
    let mut n: Option<i32> = None;
    let u = n.take();
    println!("{} {}", u.is_none() as i32, n.is_none() as i32);   // 1 1
}
