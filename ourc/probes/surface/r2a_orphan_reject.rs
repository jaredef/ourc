// R2a P6 ORPHAN guard (E0117): a foreign trait for a foreign type is refused in any crate
impl std::fmt::Display for Vec<i32> { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "x") } } fn main(){}
