// P-BOX: a trait object is a {tag, payload} record, and with P-NOM the payload's concrete type has a
// NAME -- so a PRIMITIVE that implements the trait can carry the tag too. This is rustc's own
// associated-types-eq-obj.rs shape: `impl Foo for char`, reached through `&dyn Foo`.
pub struct Bar;
pub trait Foo { type A; fn boo(&self) -> isize; }
impl Foo for char { type A = Bar; fn boo(&self) -> isize { 7 } }
fn baz(x: &dyn Foo<A = Bar>) -> isize { x.boo() }
fn main() {
    let a = 'a';
    println!("{}", baz(&a));
}
