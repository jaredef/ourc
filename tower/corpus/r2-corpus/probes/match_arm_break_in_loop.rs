fn f() -> i32 { let mut i = 0; loop { match i { 0 => { i += 1; } _ => break, } } i }
fn main(){ println!("{}", f()); }
