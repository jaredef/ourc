struct Recbox<T> { x: Box<T> }
struct Wrap<T> { v: T }
fn reclift<T>(t: T) -> Recbox<T> { return Recbox { x: Box::new(t) }; }
fn wrap<T>(t: T) -> Wrap<T> { Wrap { v: t } }
fn main() {
    let foo: isize = 17;
    let rbfoo: Recbox<isize> = reclift::<isize>(foo);
    assert_eq!(*rbfoo.x, foo);
    let w = wrap(String::from("hi"));
    assert_eq!(w.v, "hi");
    let r = reclift(String::from("yo"));
    println!("{} {} {} {}", *rbfoo.x, w.v, w.v.len(), *r.x);
}
