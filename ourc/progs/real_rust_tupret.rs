fn f()->(i64,i64){ (1,2) }
fn g()->(i64,bool){ (7,true) }
fn main() { let t=f(); println!("{:?} {:?} {} {}", t, g(), t.0+t.1, (g()).0); }
