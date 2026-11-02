struct S{v:i32}
fn main(){ let mut xs=vec![S{v:1},S{v:2}]; xs[0].v=99; println!("{} {}",xs[0].v,xs[1].v); }
