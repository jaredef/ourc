fn main(){ let v:Option<Vec<i64>>=None; let w=v.unwrap_or_default(); let x:Option<i64>=None; println!("{}", w.len() as i64 + x.unwrap_or_default()); }
