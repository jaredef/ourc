struct Pair<A, B> { a: A, b: B }
struct Tri<X, Y, Z> { x: X, y: Y, z: Z }
fn main() {
    let p = Pair { a: 1, b: 'x' };
    let q = Pair { a: String::from("s"), b: 2.5 };
    let t = Tri { x: true, y: 'y', z: String::from("zz") };
    println!("{} {} {} {} {} {} {}", p.a, p.b, q.a, q.b, t.x, t.y, t.z.len());
}
