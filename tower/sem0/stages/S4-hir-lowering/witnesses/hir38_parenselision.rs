fn is_idc(c: u8) -> bool {
    (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || (c >= 48 && c <= 57) || c == 95 || c >= 128
}
fn keep(a: i32, b: i32, c: i32) -> i32 { (a + b) * c }
fn logic(a: bool, b: bool, c: bool) -> bool { (a || b) && c }
fn deref(x: &i32) -> i32 { (*x) + 1 }

fn main() {}
