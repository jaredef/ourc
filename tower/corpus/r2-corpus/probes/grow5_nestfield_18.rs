struct C { v: i32 }
struct B { c: C }
struct A { b: B }
fn main() {
    let mut a = A { b: B { c: C { v: 1 } } };
    a.b.c.v = 42;
    println!("{}", a.b.c.v);
}
