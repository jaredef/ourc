mod bar {
    pub fn foo() -> bool { true }
    pub fn baz() -> i32 { 7 }
}
fn main() {
    let foo = || false;
    use bar::foo;
    assert_eq!(foo(), false);
    use bar::baz;
    let a = baz();
    {
        let baz = |x: i32| x + 1;
        println!("{} {}", baz(a), foo());
    }
}
