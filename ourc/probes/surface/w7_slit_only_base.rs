#[derive(Debug, PartialEq, Clone)]
struct Foo<T> { bar: T, baz: T }
#[derive(Debug, PartialEq, Clone)]
struct G { bar: i32, baz: i32 }
fn main() {
    let a = Foo { bar: 0, baz: 1 };
    let b = a.clone();
    let c = Foo { ..a };
    let g = G { bar: 3, baz: 4 };
    let h = G { ..g };
    let s = Foo { bar: "one".to_string(), baz: "two".to_string() };
    let t = Foo { ..s };
    println!("{} {} {} {} {} {}", c.bar, c.baz, c == b, h.bar, h.baz, t.baz);
}
