trait HasId { const ID: i64; fn get(&self) -> i64 { Self::ID } }
struct S;
impl HasId for S { const ID: i64 = 77; }
fn main() { println!("{}", S.get()); }
