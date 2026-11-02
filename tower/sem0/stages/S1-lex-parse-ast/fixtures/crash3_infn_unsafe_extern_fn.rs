// CRASH-3 regression guard: C-SEM0-INFN-UNSAFE-EXTERN-FN (census R5).
fn f() { unsafe extern "C" fn cb(x: usize) { } }
