// and the slice is READ as well as printed, which is what caught the first wrong answer for its type
fn main(){ let a=[5,6,7]; let s=&a[1..]; println!("{} {} {:?}", s.len(), s[0]+s[1], s); }
