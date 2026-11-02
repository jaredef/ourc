extern "C" {
    pub fn a();
    pub static BAR: u8;
    pub fn b(x: u32) -> u32;
    pub static mut BAZ: i64;
}
extern "C" {
    pub fn c();
}
fn main() {}
