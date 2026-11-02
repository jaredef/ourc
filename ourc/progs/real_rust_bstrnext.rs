// Reading a str's first character with chars().next() is ordinary Rust, and a byte view had no next. Our
// iterators are eager, so next is the first element wrapped in Option -- a prelude function, like the
// other Option-returning consumers.
fn main(){
    let src="abc"; let mut out=String::from(""); let i=1;
    out.push(src[i..(i+1)].chars().next().unwrap());
    let empty=match "".chars().next() { Some(_c)=>1, None=>0 };
    let first=match "zq".chars().next() { Some(c)=>c as i32, None=>0 };
    println!("{} {} {}", out, empty, first); }
