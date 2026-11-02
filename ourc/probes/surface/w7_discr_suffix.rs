#[derive(Debug)]
#[repr(i32)]
enum E {
    Min = -2147483648i32,
    Max = 2147483647i32,
}
#[repr(u8)]
enum B { X = 3u8, Y = 200u8 }
fn main() {
    println!("{} {} {} {}", E::Min as i32, Some(E::Max).unwrap() as i32, B::X as u8, B::Y as u8);
}
