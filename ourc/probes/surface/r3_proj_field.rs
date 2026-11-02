trait Resources {
    type Buffer: Copy;
}
struct BufferHandle<R: Resources> {
    raw: <R as Resources>::Buffer,
    n: i32,
}
enum Res {}
impl Resources for Res {
    type Buffer = u32;
}
trait Named {
    type Out;
}
struct Holder<N: Named> {
    o: <N as Named>::Out,
}
struct A;
impl Named for A {
    type Out = String;
}
fn main() {
    let b: BufferHandle<Res> = BufferHandle { raw: 7, n: 2 };
    assert_eq!(b.raw, 7);
    let h: Holder<A> = Holder { o: String::from("ok") };
    assert_eq!(h.o, "ok");
    println!("{} {} {} {}", b.raw, b.n, h.o, h.o.len());
}
