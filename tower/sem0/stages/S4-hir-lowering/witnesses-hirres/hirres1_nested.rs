fn a() -> i32 { 1 }
fn b() -> i32 { 2 }
fn c() -> i32 { a() + b() + a() }
fn d() -> i32 { c() }
