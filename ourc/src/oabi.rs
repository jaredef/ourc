#[derive(Copy, Clone)]
pub struct Layout {
    pub size: u32,
    pub align: u32,
}
pub fn layout_int() -> Layout {
    Layout { size: 8, align: 8 }
}
pub fn layout_bool() -> Layout {
    Layout { size: 1, align: 1 }
}
pub fn align_to(x: u32, a: u32) -> u32 {
    let m = a - 1;
    (x + m) & (!m)
}
pub fn ll_ty(l: &Layout) -> String {
    if l.size == 8 {
        String::from("i64")
    } else {
        String::from("i1")
    }
}
pub fn frame_size(nslots: u32) -> u32 {
    align_to(nslots * 8, 16)
}
// struct layout: n i64 fields, each 8-byte aligned -> field k at offset k*8, size = n*8
pub fn struct_size(nfields: u32) -> u32 {
    align_to(nfields * 8, 8)
}
pub fn field_offset(k: u32) -> u32 {
    k * 8
}
