fn main(){let mut v=vec![1,2,3];let w=std::mem::take(&mut v);println!("{} {}",w.len(),v.len());}
