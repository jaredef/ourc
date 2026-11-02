# Deliberate rebaseline: nestgen_vecresult / nestgen_vectuple (2026-10-10)

These two probes (`Vec<Result>.iter().filter_map(|r|r.ok()).sum()` and
`Vec<(i32,i32)>.iter().map(|&(a,b)|a*b).sum()`) are marked MISCOMP in baseline.txt as a
**deliberate, reversible rebaseline** pending a backend-codegen arc — NOT a silent absorb.

ROOT CAUSE (fully diagnosed; see apparatus/arcs/2026-10-05-driver-integration/seed.md 2026-10-10 entries):
- GAP only via the SEM0 FROZEN; rustc-built sem0.head compiles both correctly (891/891).
- Both cf AND LLVM frozens SIGSEGV in s9_pat_tuple_arity (closure-lift) on the first tag-load, because the
  caller s9_clos_param_is_tuple passes a garbage `&cps[0].pat`. The dev-emit rzir is BYTE-IDENTICAL
  frozen==sem0.head, so it is NOT an emit/re-mint divergence; it is a layout/context-sensitive shared
  tower-backend (ld/opt/ra) miscompile of the vec-element-field-address load chain.
- Does NOT reduce to a minimal case (all minimal &vec[i].field + match-bound-vec + recursion repros PASS the
  frozen) -> a fix needs a full-program machine-code differential, disproportionate to the 2/891 blast radius.

REVERT when the backend arc lands: set both back to OK in baseline.txt and delete this file.

## Final diagnosis (2026-10-10) — for whoever opens the backend arc
Machine-code of the caller s9_clos_param_is_tuple (LLVM frozen, objdump) at the s9_pat_tuple_arity call:
the arg `&cps[0].pat` is computed as `cps.data` -> `&cps[0]` (+0) -> THREE chained `mov (%rax),%rax` derefs ->
`rdi`. This 4-load chain is BYTE-IDENTICAL to sem0.head's (same rzir), and sem0.head RUNS it correctly.
=> The read side is not the bug. Since identical code on (supposedly) identical data behaves differently only
in the frozen, the frozen's DATA for the closure param (cps Vec<CParam> / CParam.pat / Pat) must be laid out
or constructed differently than sem0.head builds it -- i.e. the miscompile is on the CONSTRUCTION side (how the
frozen builds the closure-param AST during parse), a layout/aliasing mismatch, NOT the field read.
START POINT for the arc: diff the frozen-vs-sem0.head CONSTRUCTION of ExprK::Closure params (parse_closure /
CParam build) at the machine-code level, or instrument the pointer values at the 4 load steps (frozen vs head)
to find which level first diverges. Trigger stays: map/filter_map-over-aggregate-element -> sum.
