use std::hint;
use std::hint::black_box;
use std::marker::PhantomData;
struct Tag<T> { id: u32, _p: PhantomData<T> }
struct Gen<T> { id: u32, v: T }
fn main() {
    let t: Tag<String> = Tag { id: 3, _p: PhantomData };
    let x = black_box(40) + 2;
    let y = hint::black_box(21) * 2;
    let p: PhantomData<String> = PhantomData;
    let _q = p;
    let g: Gen<String> = Gen { id: 4, v: String::from("x") };
    let h = Gen { id: 5, v: 2.5 };
    println!("{} {} {} {} {} {} {} {}", t.id, x, y, std::mem::size_of::<PhantomData<u64>>(), g.id, g.v, h.id, h.v);
}
