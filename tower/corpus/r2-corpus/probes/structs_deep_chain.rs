struct A { v: i64 }
struct B { a: A }
struct C { b: B }
fn main() {
    let c = C { b: B { a: A { v: 7 } } };
    println!("{}", c.b.a.v);
}
