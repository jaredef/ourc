# Phase profile instrument (rung 84)

The compiler cannot time itself: its source must stay inside ourc's own language, which has no clock. The
RUSTC-BUILT golden can -- `bash $J/mk.sh "" yes` composes the same k.rs that `rustc -O --edition 2021`
accepts -- so the instrument goes into src/odrv.rs, is built with rustc, and is then REVERTED before the
input is composed. Feeding an instrumented unit to an instrumented compiler refuses the input: eprintln and
Instant are not in our language, and the input IS the compiler.

  1. add `fn __rss()` (reads /proc/self/statm) and `eprintln!("PH <phase> {:?} rss={}", t0.elapsed(), __rss())`
     after each step of emit_ir_in
  2. bash $J/mk.sh "" yes && rustc -O --edition 2021 $J/k.rs -o $J/k.ph
  3. git checkout src/odrv.rs && bash $J/mk.sh "" yes      # a CLEAN unit as the input
  4. ./k.ph k.rs --emit-ir >/dev/null 2>&1 | grep '^PH'

Output on the 19-crate unit, rustc-built: lex 2.9 ms, parse 231 ms, mono_structs/infer_rets ~0,
monomorphize 18 ms, lcx_fill ~0 -- the front end is PARSE, and the whole of it is 0.25 s at 47 MB.
It prints the sequence TWICE, which is how the duplicate-pipeline finding below was noticed.
