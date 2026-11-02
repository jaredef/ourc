enum E<T> { A(T), B }
fn g(e: E<i64>) -> i64 { match e { E::<i64>::A(v) => v + 1, E::B::<i64> => 0 } }
fn main(){ println!("{} {}", g(E::A(4)), g(E::B)); }
