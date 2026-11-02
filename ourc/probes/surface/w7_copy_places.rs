#[derive(Copy, Clone)]
struct A { a: i32 }
struct H { p: A }
fn bump(mut p: A) -> i32 { p.a += 1; p.a }
fn main() {
    let x = A { a: 1 };
    let r = bump(x);
    let mut y = A { a: 5 };
    y = x;
    y.a = 50;
    let mut arr = [x, x];
    arr[0].a = 5;
    let mut z = arr[1];
    z.a = 7;
    let h = H { p: A { a: 2 } };
    let mut w = h.p;
    w.a = 9;
    println!("{} {} {} {} {} {} {} {}", x.a, r, y.a, arr[0].a, arr[1].a, z.a, h.p.a, w.a);
}
