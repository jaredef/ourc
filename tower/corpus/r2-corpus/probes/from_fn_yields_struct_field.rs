struct T { kind: u32, len: u32 }
fn gen() -> impl Iterator<Item = T> {
    let mut i: u32 = 0;
    std::iter::from_fn(move || { i += 1; if i <= 3 { Some(T { kind: 0, len: i }) } else { None } })
}
fn main(){ let mut n: i64 = 0; for t in gen() { n += t.len as i64; } println!("{}", n); }  // rustc 1+2+3=6
