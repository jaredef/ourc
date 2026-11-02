// CRASH-1 regression guard: C-SEM0-INFN-DECL-MACRO-REPETITION (census R3).
// An in-fn-body macros-2.0 `macro` item with a repetition matcher SIGSEGV'd the SEM0-built parser.
fn f() { macro p($($a:tt)*) { 0 } }
