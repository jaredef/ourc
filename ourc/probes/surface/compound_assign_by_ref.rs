fn main(){
let mut a=3i8; a+=&2i8; println!("a {}",a);
let mut b=7i16; b-=&4; println!("b {}",b);
let mut d=6f64; d/=&2f64; println!("d {}",d);
let mut e=7i64; e%=&4i64; println!("e {}",e);
let mut f=0b1010u8; f|=&0b1100u8; println!("f {}",f);
let mut g=0b1010u64; g<<=&2u32; println!("g {}",g);
let mut h=0b1010u64; h>>=&2i16; println!("h {}",h);
let y=4; let mut k=1; k+=&y; println!("k {}",k);
let z=2.5f64; let mut m=1.0; m+=&z; println!("m {}",m);
}
