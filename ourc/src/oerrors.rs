#[derive(Copy, Clone)]
pub enum Level {
    Error,
    Warning,
    Note,
}
pub struct Diag {
    pub level: Level,
    pub msg: String,
    pub lo: u32,
    pub hi: u32,
}
pub fn err(msg: String, lo: u32, hi: u32) -> Diag {
    Diag {
        level: Level::Error,
        msg: msg,
        lo: lo,
        hi: hi,
    }
}
pub fn lvl_name(l: &Level) -> String {
    match l {
        Level::Error => String::from("error"),
        Level::Warning => String::from("warning"),
        Level::Note => String::from("note"),
    }
}
pub fn is_error(d: &Diag) -> bool {
    match d.level {
        Level::Error => true,
        _ => false,
    }
}
pub fn render(d: &Diag) -> String {
    let mut s = lvl_name(&d.level);
    s.push_str("[");
    s.push_str(&ospan::sp_str(&ospan::sp(d.lo, d.hi)));
    s.push_str("]: ");
    s.push_str(&d.msg);
    s
}
