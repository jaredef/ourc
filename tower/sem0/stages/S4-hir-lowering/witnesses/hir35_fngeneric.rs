fn identity<T>(x: T) -> T { x }
fn cloned<T: Clone>(x: T) -> T { x.clone() }
fn two<A, B: Clone>(a: A, b: B) -> i32 { 0 }
fn wh<T>(x: T) -> T where T: Clone { x.clone() }

trait Wrap {
    fn wrap<U: Clone>(&self, u: U) -> i32;
}

fn main() {}
