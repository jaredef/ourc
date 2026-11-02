enum E { A, B(i32), C }
fn f() -> E { E::A }
fn g() -> E { E::B(1) }
