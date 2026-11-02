enum Cmd { Move(i32, i32), Scale(i32), Reset }
fn area(c: Cmd) -> i32 { match c { Cmd::Move(dx, dy) => dx * dy, Cmd::Scale(f) => f * f, Cmd::Reset => 1 } }
fn main() { std::process::exit(area(Cmd::Move(6, 7)) + area(Cmd::Scale(3)) + area(Cmd::Reset)); }  // 42 + 9 + 1 = 52
