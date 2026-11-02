// `s.chars()` is the same element-wise view of a str as `s.as_bytes()` at this tier (a char IS an integer
// here), and a `for` over either must use the str primitives, not the vector ones -- that fell through to
// the vector primitives and read a C string as a vector handle: accepted, then a crash at run time.
fn main(){
    let s="abc";
    let mut n=0; for c in s.chars(){ if c=='b' { n=n+1; } }
    let mut v:Vec<char>=Vec::new(); for c in s.chars(){ v.push(c); }
    let mut t=0; for x in s.as_bytes(){ t=t+(*x as i32); }
    let b=s.as_bytes(); let mut u=0; for x in b { u=u+(*x as i32); }
    let o=String::from("hello"); let mut m=0; for c in o.chars(){ let _=c; m=m+1; }
    println!("{} {} {} {} {} {} {}", n, v.len(), v[1], t, u, m, s.chars().count()); }
