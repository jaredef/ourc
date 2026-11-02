#[derive(Clone, Copy)]
pub enum E {
    A,
    B(i32, u8),
    C { x: i32, y: bool },
}
