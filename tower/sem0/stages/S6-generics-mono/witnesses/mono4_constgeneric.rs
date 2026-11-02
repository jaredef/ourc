fn arr<const N: usize>() -> usize { N }
fn f() { arr::<3>(); arr::<5>(); }
fn main() { f(); }
