# Remaining SEM0 corpus MISCOMP debt (10 probes, ~6 roots) — scoping after the Box/deref rung

Board: 879 OK / 10 MISCOMP (SEM0 tier). The Box/deref rung (commit 63652a35) was the one surgical fix; each
remaining pipeline is a distinct DEEP arc. Precise localizations below (all via [[rusteze-sem0-test-recipe]]).

## 1. macro-stmt-repeat — macro_rules_repetition_stmt  [S2 re-parse]
A user macro whose body is a STATEMENT with a trailing `;` (`=> { println!("v={}", $x); }`), called in statement
position, expands to NOTHING (collapses to the format-string value). PRECISE LOCALIZATION (minimized):
  - mG `=> { println!("v = {}", $x) }`  (NO trailing semi)  -> WORKS.
  - mH `=> {{ println!("v = {}", $x); }}` (explicit block)  -> WORKS.
  - mA `=> { println!("v = {}", $x); }`  (semi -> auto-wrap) -> FAILS (drv empty; println dropped to its string).
  - mJ `=> { println!("plain"); }` (semi, no {} in fmt)      -> FAILS  => not a format-string issue.
  - mI `=> { let q=$x+1; println!("{}",q); }` (2 stmts)      -> FAILS.
mac_expand_expr wraps a top-level-semi body via `format!("{{ {} }}", sbody)` then `parse_expr(0)`. The explicit
`{{}}` body (semi=false, parsed as-is) works; the auto-wrapped body (semi=true) parses to just the inner
expr's string. The wrapped and explicit strings LOOK identical (`{ println!(..); }`) yet parse differently ->
the bug is in the wrap+re-lex+parse_expr(0) interaction, NOT the block content (plain `{ println!(..); }` block
statements mD/mE/mF all work). FIX DIRECTION: mac_expand_expr @ ~2189 -- parse the wrapped multi-stmt body as a
BLOCK the same way the explicit-`{{}}` body is parsed (bump `{` + parse_block, or register the body already
braced) instead of the string-wrap+parse_expr that drops the statement. (Diagnostic note: eprintln! is a no-op
in the SEM0 self-runtime, and `{}` on a String var printed its address -- use __putc-loop dumps for S2 debugging.)

## 2. closure-mono-in-generics — generic_slice_struct_method, census_trait_generic_crate, census_layer3
See P-MONO-ELEM-scoping-2026-09-25.md. Map closure in a generic fn is lifted once generically (x:&T unresolved);
needs per-instantiation closure mono (or single-impl dispatch for the 1-impl case). $clospelem hint is a building
block. census_* additionally bundle derives/macros.

## 3. tuple/deref — layer9_byref_tuple_match_payload_deref
Crashes. `match (&ra, &rb) { (Ty::Infer(v), _) => ... }` -- a match on a TUPLE OF REFS with enum-payload binders;
composite with enum clone + Vec index. P-DEREF-family (the by-ref tuple payload deref).

## 4. canonical-unify — layer9_canonical_tuple_unify   (inference/tuple)
## 5. impl-trait-lifetime — impl_trait_return_lifetime_bound  (impl Trait return + lifetime)
## 6. arena/dyn — rustcfe_arena_alloc_ref, rustcfe_slice_dyn_trait_dispatch  (raw-ptr arena; Vec<Box<dyn>> dispatch)
## 7. closure-field — grow9_closurefield_18  (prints a garbage pointer; a closure stored in a struct field)

Each is its own focused arc. The macro-stmt (S2) and closure-field ones look most self-contained for a next rung;
closure-mono and the layer9/impl-trait/arena ones are larger.
