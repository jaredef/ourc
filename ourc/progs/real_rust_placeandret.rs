// the same call used as a VALUE twice inside one expression: each call writes its place back in order
fn g(a:&mut i64)->i64{ *a+=1; *a }
fn main(){ let mut p=0; let n=g(&mut p)+g(&mut p); println!("{} {}",p,n); }
