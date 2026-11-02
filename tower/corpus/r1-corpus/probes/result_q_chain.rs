fn parse2(a:i32,b:i32)->Result<i32,i32>{ if b==0 { return Err(-1); } Ok(a/b) }
fn run()->Result<i32,i32>{ let x=parse2(20,4)?; let y=parse2(x,0)?; Ok(y) }
fn main(){ match run() { Ok(v)=>println!("{}",v), Err(e)=>println!("err{}",e) } }
