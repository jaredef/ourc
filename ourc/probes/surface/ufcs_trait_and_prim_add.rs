trait Foo { fn get(&self) -> i32; fn add(&self, k: i32) -> i32; fn dflt(&self) -> i32 { self.get() + 100 } }
impl Foo for i32 { fn get(&self) -> i32 { *self * 2 } fn add(&self, k: i32) -> i32 { *self + k } }
fn main(){ let x: i32 = 4; println!("{}", Foo::dflt(&x) + x.add(1)); }
