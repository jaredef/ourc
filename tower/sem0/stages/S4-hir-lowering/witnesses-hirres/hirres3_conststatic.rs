const K: i32 = 7;
static S: i32 = 9;
fn f() -> i32 { 1 }
fn g() -> i32 {
    let x = 3;
    K + S + x + f()
}
