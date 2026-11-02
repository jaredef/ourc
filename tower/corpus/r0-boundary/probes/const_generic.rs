fn sumn<const N:usize>(a:[i32;N])->i32{ let mut t=0; for x in a { t+=x; } t }
fn main(){ println!("{}", sumn([1,2,3,4])); }
