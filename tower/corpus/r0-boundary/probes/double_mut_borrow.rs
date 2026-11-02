fn main(){ let mut x=5; let a=&mut x; let b=&mut x; *a+=1; *b+=1; println!("{}", x); }
