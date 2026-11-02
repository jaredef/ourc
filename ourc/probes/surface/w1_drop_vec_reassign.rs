struct F; impl Drop for F { fn drop(&mut self) { println!("d"); } } fn main(){ { let mut v: Vec<F> = vec![F]; v = vec![]; } println!("e"); }
