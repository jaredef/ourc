trait Sub: Clone { fn m(&self) -> i32; }
trait Gen<X: Clone> { fn g(&self, x: X) -> i32; }
trait Wh<X> where X: Clone { fn w(&self) -> i32; }

fn main() {}
