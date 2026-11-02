struct Foo; struct Bar { x: i64 } struct Baz(i64);
enum FooBar { F(Foo), B(i64) }
impl Drop for Foo { fn drop(&mut self) { println!("foo"); } }
impl Drop for Bar { fn drop(&mut self) { println!("bar"); } }
impl Drop for Baz { fn drop(&mut self) { println!("baz"); } }
impl Drop for FooBar { fn drop(&mut self) { println!("foobar"); } }
fn main() {
    { let _x = Foo; } println!("1");
    { let _x = Bar { x: 21 }; } println!("2");
    { let _x = Baz(21); } println!("3");
    { let _x = FooBar::F(Foo); } println!("4");
    { let _x = FooBar::B(42); } println!("5");
    { let _ = Foo; } println!("6");
    { let _ = Bar { x: 21 }; } println!("7");
    { let _ = FooBar::F(Foo); } println!("8");
}
