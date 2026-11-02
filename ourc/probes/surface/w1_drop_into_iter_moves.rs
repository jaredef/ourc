struct F; impl Drop for F { fn drop(&mut self) { println!("d"); } } fn main(){ { let v = vec![F, F]; let w: Vec<F> = v.into_iter().collect(); println!("{}", w.len()); } println!("e"); }
