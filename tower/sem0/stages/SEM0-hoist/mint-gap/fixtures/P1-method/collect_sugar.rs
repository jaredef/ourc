fn main(){ let v:Vec<i32>=(0..5).map(|x|x*2).collect(); println!("{} {}",v.len(),v[3]); }
