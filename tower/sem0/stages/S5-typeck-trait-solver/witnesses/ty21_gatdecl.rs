trait Ct { type Item<X>; type Bare; }
struct S;
impl Ct for S { type Item<X> = X; type Bare = i32; }
fn main() {}
