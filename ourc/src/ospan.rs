#[derive(Copy, Clone)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}
pub fn sp(lo: u32, hi: u32) -> Span {
    Span { lo: lo, hi: hi }
}
pub fn sp_to(a: &Span, b: &Span) -> Span {
    Span { lo: a.lo, hi: b.hi }
}
pub fn sp_len(s: &Span) -> u32 {
    s.hi - s.lo
}
pub fn u32s(n: u32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut v = n;
    let mut s = String::from("");
    while v > 0 {
        let d = (v % 10) as u32;
        let c = (48 + d) as u8 as char;
        let mut t = String::from("");
        t.push(c);
        t.push_str(&s);
        s = t;
        v = v / 10;
    }
    s
}
pub fn sp_str(s: &Span) -> String {
    let mut r = u32s(s.lo);
    r.push_str("..");
    r.push_str(&u32s(s.hi));
    r
}
