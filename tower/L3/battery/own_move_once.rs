struct Buf { len: i32 }
fn consume(b: Buf) -> i32 { b.len }
fn main() { let a = Buf { len: 42 }; std::process::exit(consume(a)); }   // 42
