#[derive(Clone, Debug)]
pub enum E {
    A,
    B(i32, u8),
    C { x: i32, y: bool },
}
