fn f(s:&[i32])->i32{ match s { [] => 0, [a] => *a, [a,.., b] => a+b } }
fn main(){ let v=[1,2,3,4]; println!("{}", f(&v)); }
