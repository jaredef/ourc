#[derive(Copy, Clone)]
pub struct HirId {
    pub v: u32,
}
#[derive(Copy, Clone)]
pub struct LocalId {
    pub v: u32,
}
#[derive(Copy, Clone)]
pub struct Temp {
    pub v: u32,
}
pub fn hid(v: u32) -> HirId {
    HirId { v: v }
}
pub fn lid(v: u32) -> LocalId {
    LocalId { v: v }
}
pub fn tid(v: u32) -> Temp {
    Temp { v: v }
}
pub fn hidx(h: &HirId) -> u32 {
    h.v
}
pub fn lidx(l: &LocalId) -> u32 {
    l.v
}
pub fn tidx(t: &Temp) -> u32 {
    t.v
}
// IndexVec: a Vec addressed by a typed index (the crate's defining shape)
pub struct TempVec {
    pub raw: Vec<i64>,
}
pub fn tv_new() -> TempVec {
    TempVec { raw: Vec::new() }
}
pub fn tv_push(v: &mut TempVec, x: i64) -> Temp {
    let i = v.raw.len() as u32;
    v.raw.push(x);
    tid(i)
}
pub fn tv_get(v: &TempVec, t: &Temp) -> i64 {
    v.raw[t.v as usize]
}
pub fn tv_set(v: &mut TempVec, t: &Temp, x: i64) {
    let i = t.v as usize;
    v.raw[i] = x;
}
pub fn tv_len(v: &TempVec) -> u32 {
    v.raw.len() as u32
}
