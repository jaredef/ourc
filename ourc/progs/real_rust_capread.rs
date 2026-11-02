// the control: a closure that only READS its captures is untouched -- and a closure's own LOCAL may still
// be mutated, which is a different thing entirely
fn main(){ let n=4; let f=|x:i64| x+n; let g=|x:i64| { let mut a=x; a=a+1; a };
    let v=vec![1,2,3]; let w:Vec<i64>=v.iter().map(|x| x*n).collect();
    println!("{} {} {:?}", f(1), g(4), w); }
