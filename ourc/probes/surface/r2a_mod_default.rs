// R2a P5 guard: `Default::default` of a type declared in a module, called through the module path
// (`l::P::default()`, `crate::l::P::default()`) -- it was the call `l$P$default()` ("cannot find function")
mod l {
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct P {
        pub a: i32,
        pub b: String,
    }
}
fn main() {
    let p = l::P::default();
    let q = crate::l::P::default();
    println!("{:?} {} {}", p.clone(), p == l::P { a: 0, b: String::new() }, q.a);
}
