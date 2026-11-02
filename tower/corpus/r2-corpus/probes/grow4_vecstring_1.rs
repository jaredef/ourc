fn main(){ let mut v:Vec<String>=Vec::new(); v.push("foo".to_string()); v.push("bar".to_string()); v.push("baz".to_string()); let mut t=0; for s in &v { t+=s.len(); } println!("{} {}", v.len(), t); }
