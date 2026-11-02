fn main(){ let o:Option<Option<i32>>=Some(Some(9)); if let Some(inner)=o { if let Some(v)=inner { println!("{}",v); } } }
