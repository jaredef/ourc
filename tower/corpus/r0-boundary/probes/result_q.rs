fn parse(ok:bool)->Result<i32,i32>{ if ok { Ok(7) } else { Err(1) } }
fn go()->Result<i32,i32>{ let a=parse(true)?; Ok(a*2) }
fn main(){ println!("{}", go().unwrap()); }
