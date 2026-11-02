// G1 FMT-AGG-DEBUG guard: `format!("{:?}", aggregate)` (a tuple, a Vec, a derived struct) builds the Debug text by capturing
// the printing builder's output (it was refused: "debug format of an aggregate inside format is not implemented")
#[derive(Debug)] struct P { a: i32, b: String }
fn main(){ let s = format!("{:?}", (1, "a")); let v = format!("{:?}|{:?}", vec![1,2], P { a: 3, b: String::from("x") }); println!("{} {} {:?}", s, v, (2, "b")); }
