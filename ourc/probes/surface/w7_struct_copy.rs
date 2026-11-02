#[derive(Copy, Clone)]
struct A { a: isize }
#[derive(Copy, Clone, Debug)]
struct P { u: i32, w: i32 }
#[derive(Clone)]
struct V { v: Vec<i32> }
fn f(arg: &mut A) { arg.a = 100; }
fn main() {
    let mut x = A { a: 10 };
    f(&mut x);
    x.a = 20;
    let mut y = x;
    f(&mut y);
    let p = P { u: 1, w: 2 };
    let mut q = p;
    q.u = 9;
    let w = V { v: vec![1] };
    let mut z = w.clone();
    z.v.push(2);
    println!("{} {} {:?} {:?} {} {}", x.a, y.a, p, q, w.v.len(), z.v.len());
}
