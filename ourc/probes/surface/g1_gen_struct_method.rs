// G1 G-P3 guard: a method of a GENERIC USER struct (`impl<T: Debug> W<T>`) is instanced per receiver instantiation (T from
// the receiver's GEN-INST: `W:T=int`), and `self.v: T` reads the instance's type -- the one erased `W$show` typed `self.v`
// as an unknown T and refused `{:?}` of it even for W<i32>
struct W<T> {
    v: T,
}
impl<T: std::fmt::Debug> W<T> {
    fn show(&self) -> String {
        format!("{:?}", self.v)
    }
}
fn main() {
    println!("{} {}", W { v: 5 }.show(), W { v: "s" }.show());
}
