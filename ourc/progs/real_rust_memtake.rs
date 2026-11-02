// the default of a Vec place is an EMPTY VECTOR, not zero -- the type decides, so lowering decides
fn main(){ let mut v:Vec<i64>=Vec::new(); v.push(3); let t=std::mem::take(&mut v); println!("{} {} {}",v.len(),t.len(),t[0]); }
