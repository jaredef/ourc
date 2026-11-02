trait SomeTrait { type SomeType; fn v(&self) -> isize; }
struct X;
impl SomeTrait for X { type SomeType = isize; fn v(&self) -> isize { 11 } }
fn main() {
    let x = X;
    let r: &dyn SomeTrait<SomeType = isize> = &x;
    println!("{}", r.v());
}
