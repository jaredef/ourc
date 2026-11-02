fn a() -> i32 { let x = async { 1 }; 0 }
fn b() -> i32 { let y = async move { 2 }; 0 }
async fn c() -> i32 { 5 }
async fn d() { }
fn main() {}
