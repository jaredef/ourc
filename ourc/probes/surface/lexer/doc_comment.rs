/// doc on fn
fn f() -> i64 { 41 }
fn main() { //! not inner doc here is an error in rustc? no: inner doc mid-block
    println!("{}", f() + 1); }
