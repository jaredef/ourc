struct S(i64, i64, i64);
fn main(){ let s=S(1,2,3); match s { S(..,b,c) => println!("F {} {}",b,c) } }
