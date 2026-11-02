fn main() { let mut v = Vec::new(); v.push("hi"); v.push("yo"); let mut n = Vec::new(); for i in 0..3 { n.push(i * 2); } println!("{} {} {}", v[0], v[1], n[2]); }
