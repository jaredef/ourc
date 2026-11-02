// C-ARRELEM: an array's type name carries its ELEMENT now, so the Debug printer picks by element --
// this printed [1, 0] while the bare name only meant "array of something".
fn main(){ let a=[true,false]; let s=["x","y"]; let v=vec![true,false,true];
    println!("{:?} {:?} {:?}", a, s, v); }
