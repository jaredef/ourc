fn t(x: Option<usize>) -> i32 { match x { Some(_x @ (0 | 1)) => 1, Some(0 | _) => 2, _ => 0 } }
fn main(){ let v: usize = 1; println!("{} {} {} {}", t(Some(0)), t(Some(42)), t(None), matches!(v, (0 | 1))); }
