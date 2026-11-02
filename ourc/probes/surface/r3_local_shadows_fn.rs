struct Pt<A> { x: A }
fn f<A>(x: Pt<A>) -> Pt<A> { x }
fn main() {
    let r = f(Pt { x: 2 });
    let v = vec![1, 2, 3];
    let k: Vec<i32> = v.into_iter().filter(|y| *y > 1).collect();
    println!("{} {:?}", r.x, k);
}
