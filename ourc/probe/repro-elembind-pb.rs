fn u32s(n:u32)->String{ if n==0 {return String::from("0");} let mut v=n; let mut s=String::from(""); while v>0 {let d=(v%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;v=v/10;} s }
fn main(){
    let prog=pa::mk();
    // BEFORE any move: read funs
    println!("PRE nfun={} f0={} np0={}", u32s(prog.funs.len() as u32), prog.funs[0].name, u32s(prog.funs[0].params.len() as u32));
    // BEFORE the move: bound-ref to an element of the FIELD vec
    let mut b=0; while b<prog.funs.len(){ let f=&prog.funs[b]; println!("PREB{} name={} np={}", b, f.name, u32s(f.params.len() as u32)); b=b+1; }
    let ast=prog.main;                                  // PARTIAL MOVE of one field
    println!("POST ast={}", u32s(pa::ex_val(&ast) as u32));
    // AFTER the move: read the OTHER field
    println!("POST nfun={} f0={} np0={}", u32s(prog.funs.len() as u32), prog.funs[0].name, u32s(prog.funs[0].params.len() as u32));
    let mut j=0; while j<prog.funs.len(){ let f=&prog.funs[j]; println!("POSTB{} name={} np={}", j, f.name, u32s(f.params.len() as u32)); j=j+1; }
}
