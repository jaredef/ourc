// C-ZEROAGG: a struct's default is a struct of per-field defaults -- the zero rule asking itself one
// level down. Answering an integer zero for an aggregate handed back a NULL HANDLE.
#[derive(Default)]
struct Inner { n:i64 }
#[derive(Default)]
struct Outer { i:Inner, s:String, v:Vec<i64> }
fn main(){ let mut o = Outer::default(); o.v.push(4);
    println!("{} [{}] {} {}", o.i.n, o.s, o.v.len(), o.v[0]); }
