struct N(u8);
impl Drop for N {
    fn drop(&mut self) { print!("d{} ", self.0); }
}
struct Foo { n0: N, n1: N }
macro_rules! run { ($($t: tt)*) => { $($t)* } }
macro_rules! sum { ($($t: tt)*) => { 0 $(+ $t)* } }
macro_rules! blk { ($($code: tt)*) => { {$($code)*} } }
fn early(c: bool) -> i32 {
    let _a = N(1);
    let _b = N(2);
    if c { return 5; }
    let _e = N(3);
    7
}
fn mk() -> Foo { print!("mk "); Foo { n0: N(10), n1: N(11) } }
fn ret_in_lit() -> Foo {
    let old = || Foo { n0: N(20), n1: N(21) };
    Foo { n1: { return Foo { n0: N(30), n1: N(31) } }, ..old() }
}
fn main() {
    #[cfg(target_pointer_width = "32")]
    assert_eq!(1, 2);
    println!("{} {}", early(true), early(false));
    let x = Foo { n1: { print!("f "); N(15) }, ..mk() };
    println!("| {} {}", x.n0.0, x.n1.0);
    let y = ret_in_lit();
    println!("| {} {}", y.n0.0, y.n1.0);
    println!("{} {}", run!(1 + 2), sum!(1 2 3));
    blk!(println!("blk"););
}
