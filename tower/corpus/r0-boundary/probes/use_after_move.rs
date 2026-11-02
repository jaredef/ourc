struct S{v:i32}
fn main(){ let a=S{v:1}; let b=a; println!("{}", a.v); let _=b; }
