trait Tr { fn m(&self); }
struct S;
impl Tr for S { fn m(&self) {} }
impl Tr for S { fn m(&self) {} }
fn main() {}
