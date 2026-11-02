// A tiny facet-3 trigger: s1.rz (emit_s9) EMITS GARBLED rzir for this (its __obuf/heap is corrupted mid-emit --
// the emitted output has interleaved/cut lines + a scrambled partial second runtime block), while s1.rz emits the
// nearly-identical `w[0].len()` (acc_repro_return_vec_string) CLEANLY. So it is layout-sensitive s1.rz-runtime heap
// corruption = the L6 facet-3 whole-file lowering bug, reproduced in ~2 lines. L6-direct compiles this correctly.
fn mk() -> Vec<String> { let mut o: Vec<String> = Vec::new(); o.push(format!("None")); o }
fn main() { let w = mk(); println!("{}", w.len()); }  // rustc -> 1 ; emit_s9 -> garbled rzir -> segfault
