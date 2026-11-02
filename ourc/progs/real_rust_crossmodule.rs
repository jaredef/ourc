// A struct built in one module and read in another printed its FIRST field three times. C-TYRESOLVE
// fixed it in rung 71: the reader could
// not find the struct under the name it had, so every field index resolved to 0 -- silently, in both of
// the two places that rule lives. C-FIELDUNKNOWN now refuses instead. The REAL fix is to resolve a
// function's declared return type across module boundaries the way bare type names already are; until
// The resolution belongs where a type NAME is validated, not where a field is looked up.
mod a { pub struct S { pub x:i32, pub y:i32, pub z:i32 } }
mod b { use crate::a::*; pub fn mk()->S{ S{x:11, y:22, z:33} } }
fn main(){ let s=b::mk(); println!("{} {} {}", s.x, s.y, s.z); }
