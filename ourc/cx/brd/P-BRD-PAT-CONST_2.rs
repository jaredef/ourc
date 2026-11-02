struct L;
impl L { const MAX: i32 = 100; }
fn main() { for n in [100i32, 5] { println!("{}", match n { L::MAX => "max", _ => "mid" }); } }
