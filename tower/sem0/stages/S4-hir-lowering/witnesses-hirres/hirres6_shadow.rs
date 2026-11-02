const K: i32 = 7;
fn shadowed() -> i32 { let K = 3; K + K }
fn plain() -> i32 { K }
fn scoped() -> i32 {
    let a = K;
    { let K = 1; let _b = K; }
    a + K
}
