// a small commented module
pub enum Kind {
    A,   // first
    B,   /* second */
}
pub fn k(n: u32) -> Kind {
    // classify
    match n {
        0 => Kind::A,
        _ => Kind::B,
    }
}
