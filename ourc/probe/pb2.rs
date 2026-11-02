fn u32s(n:u32)->String{ if n==0 {return String::from("0");} let mut v=n; let mut s=String::from(""); while v>0 {let d=(v%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;v=v/10;} s }
enum Tk2 { Num(i64), Plus }
struct Tok2 { k: Tk2, lo: u32, hi: u32 }
fn main(){ let mut v:Vec<Tok2>=Vec::new(); v.push(Tok2{k:Tk2::Num(5), lo:0, hi:1}); v.push(Tok2{k:Tk2::Plus, lo:2, hi:3});
  println!("SAME-CRATE {} {} {} {}", u32s(v[0].lo), u32s(v[0].hi), u32s(v[1].lo), u32s(v[1].hi)); }
