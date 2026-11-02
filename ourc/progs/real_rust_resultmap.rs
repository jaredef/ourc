// C-RESULTMAP: Result.map transforms Ok, passes Err through; Result.map_err transforms Err, passes Ok
// through -- the symmetric siblings of Option.map. Enables the common `parse().map_err(..)?` chain.
fn p(s:&str)->Result<i64,i64>{ let n=s.parse::<i64>().map_err(|_| 0)?; Ok(n+1) }
fn main(){ let a=p("4"); let b=p("x");
    let r:Result<i64,i64>=Ok(3); let m=r.map(|v| v*2);
    println!("{:?} {:?} {:?}", a, b, m); }
