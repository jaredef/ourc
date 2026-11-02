#[derive(Clone)] struct S{a:i32}
fn main(){ let s=S{a:1}; let t=s.clone(); println!("{}",t.a); }
