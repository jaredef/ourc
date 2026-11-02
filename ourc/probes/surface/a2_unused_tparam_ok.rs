use std::marker::PhantomData;
struct P<T> { x: T }
struct Q<T> { m: PhantomData<T>, n: i32 }
enum E<A, B> { L(A), R(B) }
struct C<const N: usize> { v: i32 }
fn main() {
    let p = P { x: 4 };
    let q: Q<u8> = Q { m: PhantomData, n: 5 };
    let e: E<i32, bool> = E::L(6);
    let c: C<3> = C { v: 7 };
    let k = match e { E::L(v) => v, E::R(_) => 0 };
    println!("{} {} {} {}", p.x, q.n, k, c.v);
}
