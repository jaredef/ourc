#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Namespace { Type, Value, Macro }
#[derive(Clone, Copy, Debug)]
struct Res { ns: Namespace, id: u32 }
fn main() {
    let a = Res { ns: Namespace::Value, id: 7 };
    let b = a;
    println!("{}", (a.ns == b.ns) as i32);
    println!("{:?}", a.ns);
    println!("{:?}", b);
}
