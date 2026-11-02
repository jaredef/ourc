fn main(){ let mut v=vec![1,2,3]; for x in v.iter_mut(){ *x*=10; } println!("{} {}",v[0],v[2]); }
