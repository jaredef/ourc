fn f((): (), x: i64, (): ()) -> i64 { x * 3 }
fn main(){ println!("{}", f((), 5, ())); }
