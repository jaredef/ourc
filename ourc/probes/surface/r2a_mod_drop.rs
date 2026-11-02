// R2a P5 guard: `impl Drop` of a type declared in a module runs at scope end (inner scope first, then reverse order) --
// the drop glue cut the qualified type `l$D` to `l` (gbase) and found no `l$drop`, so NO drop ran (a silent WRONG)
mod l {
    pub struct D(pub &'static str);
    impl Drop for D {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }
    pub fn mk(s: &'static str) -> D {
        D(s)
    }
}
fn main() {
    let _a = l::D("a");
    {
        let _b = l::mk("b");
        let _c = l::D("c");
    }
    println!("end");
}
