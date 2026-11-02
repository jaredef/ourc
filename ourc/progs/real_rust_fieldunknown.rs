// The two controls for C-FIELDUNKNOWN: a struct built and read in the same module, and one built and read
// inside another module, must both stay exact. Only the CROSS-module case is refused (see
// probes/crossmodule-struct-return.rs), and these pin that the refusal is narrow.
mod a { pub struct S { pub x:i32, pub y:i32, pub z:i32 } }
mod b { use crate::a::*; pub fn go()->i32{ let s=S{x:11, y:22, z:33}; s.x+s.y*100+s.z*10000 } }
use a::*;
fn main(){ let s=S{x:1, y:2, z:3}; println!("{} {} {} {}", s.x, s.y, s.z, b::go()); }
