// wrapping_mul / wrapping_add / wrapping_sub are the plain operators at this tier, and that is exact:
// a u64 and an i64 running the same wrapping chain produce identical BITS. Printed here as hex nibbles
// so the comparison is on the bits rather than on signed-vs-unsigned decimal.
fn hexit(v:i64)->String{
    let d="0123456789abcdef"; let db=d.as_bytes();
    let mut r=String::from(""); let mut i=60;
    while i>=0 { r.push(db[((v>>i)&15) as usize] as char); if i==0 { i=-1; } else { i=i-4; } }
    r }
fn main(){
    let mut h:i64=5381; let s="abcdefghijklmnopqrstuvwxyz0123456789"; let b=s.as_bytes();
    let mut k=0; while k<b.len(){ h=h.wrapping_mul(33).wrapping_add(b[k] as i64); k=k+1; }
    let mut g:i64=100; g=g.wrapping_sub(7).wrapping_mul(3);
    println!("{} {}", hexit(h), g); }
