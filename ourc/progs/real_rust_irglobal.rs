// The IR self-verification must read INSTRUCTIONS, not the bytes of a string constant. A global line
// starts at column zero with an @ and its payload is data: scanning it for calls made a literal
// containing "call @foo" look like a call to an undefined function.
fn main(){
    let s="call @foo";
    let t="the emitted program calls `@zz`, which was never defined";
    println!("{} {} {}", s, t.len(), s.len()); }
