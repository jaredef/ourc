struct Inner { v: i64 }
struct Outer { a: Inner, b: Inner }
fn main() {
    let o = Outer { a: Inner { v: 10 }, b: Inner { v: 32 } };
    println!("{}", o.a.v + o.b.v);
}
