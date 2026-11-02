// P-CLOSURE-DYNFIELD guard: Box<dyn Fn> local + capture, Box<dyn Fn> struct field
// (single + multi), and the Fn-bounded generic field `f: F`. Each closure value is
// our {code,env} fat-ptr cell; `(s.f)(x)` calls through it.
struct Boxed { f: Box<dyn Fn(i32) -> i32> }
struct Two { g: Box<dyn Fn(i32) -> i32>, h: i32 }
struct Gen<F: Fn(i32) -> i32> { f: F }
fn main() {
    let k = 7;
    let local: Box<dyn Fn(i32) -> i32> = Box::new(move |n| n + k);
    let b = Boxed { f: Box::new(|n| n * 3) };
    let t = Two { g: Box::new(|n| n + 1), h: 9 };
    let g = Gen { f: |x| x + 100 };
    println!("{}", local(10));      // 17
    println!("{}", (b.f)(4));       // 12
    println!("{}", (t.g)(4) + t.h); // 14
    println!("{}", (g.f)(23));      // 123
}
