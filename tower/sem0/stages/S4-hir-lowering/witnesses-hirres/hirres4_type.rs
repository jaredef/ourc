struct Pt { x: i32 }
enum E { A, B }
fn mk() -> Pt { Pt { x: 1 } }
fn use_e(_e: E) -> i32 { 0 }
