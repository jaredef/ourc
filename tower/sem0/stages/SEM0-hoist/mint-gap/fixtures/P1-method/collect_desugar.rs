fn main(){ let mut v:Vec<i32>=Vec::new(); let mut x=0; while x<5 { v.push(x*2); x+=1; } println!("{} {}",v.len(),v[3]); }
