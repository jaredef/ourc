fn main(){ let a:u32=0xF0; let b:u32=0x0F; println!("{} {} {} {} {}", a|b, a&b, a^b, a<<2, a>>2); }
