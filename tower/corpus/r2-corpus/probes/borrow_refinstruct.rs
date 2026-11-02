struct R<'a>{v:&'a i64}
fn main(){let n=17;let r=R{v:&n};println!("{}",*r.v);}
