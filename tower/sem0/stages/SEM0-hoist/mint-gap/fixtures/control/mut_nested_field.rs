struct I{n:i32} struct O{i:I}
fn main(){ let mut o=O{i:I{n:5}}; o.i.n+=10; println!("{}",o.i.n); }
