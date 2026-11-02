fn main(){ let v:Option<i64>=Some(7); let r = if let Some(x)=v { x*2 } else { -1 }; let w:Option<i64>=None; let s= if let Some(x)=w { x } else { 99 }; println!("{}", r+s); }
