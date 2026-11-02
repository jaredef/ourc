#[derive(Debug)]
enum Foo {
    A(usize),
    C,
}
fn main() {
    let foo_a = Foo::A(22);
    println!("{:?}", foo_a);
    print!("{:?} ", foo_a);
    if let Foo::A(v) = foo_a {
        println!("{}", v);
    }
    println!("{:?}", Foo::C);
}
