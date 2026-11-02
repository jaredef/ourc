// CRASH-2 regression guard: C-SEM0-ATTRUNSAFE-CALLTAIL-LOOP-DEREF (census R4).
fn f() -> R {
    #[cfg(unix)]
    unsafe { g(os) }
    { loop { if (*c).n == 0 { } } }
}
