fn g(o:Option<i32>)->i32{ let Some(x)=o else { return -1 }; x*10 }
fn main(){ println!("{}",g(Some(4))+g(None)); }
