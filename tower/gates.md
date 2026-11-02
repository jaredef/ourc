# tower gates — rationale

The ordered gate list run under `build-tower.sh --gates` is a table in `build-tower.sh`; the *why* for
each gate lives here (relocated verbatim from the former inline comments). Each entry records what the
gate uniquely catches — several note the exact past incident that the gate exists to prevent recurring.

## sem0-stage-llvm (C2: llvm-frozen emit == cf; stage0 mint)

`$A/backend-llvm/sem0-stage-llvm.sh`

STAGE0 PROMOTION 2026-10-02 (keeper-authorized, Tier-2 C2 self-host-stage0 admission -- tower-naming-and-trust-
path.md). The LLVM-minted frozen is the self-host stage0: mint it (cf bootstrap seeds it + C2-guards emit ==
cf byte-identical), and if present, point SEM0_CC at it so every S-tier gate compiles the dev with it. C2 FAIL
aborts (a divergent LLVM frozen must never be the stage0). Skips cleanly to the cf frozen when llc/cc absent.

## S1 AST differential

`$B/S1-lex-parse-ast/s1-ast-diff.sh`

_(no inline rationale in the original)_

## S2 macro expand

`$B/S2-macro-procmacro/s2-expand-diff.sh`

_(no inline rationale in the original)_

## S3 resolve

`$B/S3-resolve-crate-driver/s3-resolve-diff.sh`

_(no inline rationale in the original)_

## S4 HIR lowering

`$B/S4-hir-lowering/s4-hir-diff.sh`

_(no inline rationale in the original)_

## S5 typeck

`$B/S5-typeck-trait-solver/s5-typeck-diff.sh`

_(no inline rationale in the original)_

## S6 generics/mono

`$B/S6-generics-mono/s6-mono-diff.sh`

_(no inline rationale in the original)_

## S7 checker tiers

`$B/S7-checker-tiers/s7-check-diff.sh`

_(no inline rationale in the original)_

## S9 run differential (sem)

`$B/S9-lower-codegen/s9-run-diff.sh`

_(no inline rationale in the original)_

## S9 whole-file self-compilation

`$B/S9-lower-codegen/s9-wholefile-diff.sh`

The only gate that runs gen2 -- emit_s9 SELF-COMPILED -- over a real crate. Every other gate exercises gen1 or
the sem witnesses, so a self-compilation regression is invisible to them: on 2026-08-24 a one-line lowering
change took this board 35/35 -> 11 ok / 24 crash and passed all 14 of the others.

## S9 behavioural differential

`$B/S9-lower-codegen/s9-behavior-diff.sh`

The only gate that compares BEHAVIOUR to the source's meaning rather than emissions to each other. Every other
S9 check is emission-vs-emission and is therefore structurally blind to an emit_s9 fidelity gap: on 2026-08-25
std::process::exit(N) was found to be a silent no-op in emit_s9 (exit 0 where rustc gives N) with all 16 gates
green, because SELF-EMIT compares two emissions that BOTH run emit_s9. Requires wf.gen1/wf.gen2, so it must run
after s9-wholefile-diff.sh. ~1 min.

## r2-corpus zero-regression

`$A/corpus/r2-corpus/corpus-gate.sh`

The PARITY-vs-rustc board over 904 real probes, gated at zero-regression. The project names this as the parity
oracle and states "zero regression per rung", but until 2026-08-25 nothing ran it: the i64-statics promotion
passed all 15 gates while carrying a live miscompile class (16 probes, all one meet). Every other gate here
checks self-consistency or a curated witness set; this is the only one that asks "does it match rustc, at
breadth". ~2 min.

## D0 over-acceptance (vs rustc)

`$B/D0-derived-substrate/overaccept-board.sh`

D0 over-acceptance ratchet: the ONLY gate here fed inputs the front end must REJECT. Every other corpus
holds valid programs only, which is how `is_idc` accepted every byte >= 128 for months while the tree board
reported 0 OVER-ACCEPTED. Known-open divergences are listed in D0-derived-substrate/known-open.txt; a
divergence not on that list fails the build, and so does a stale entry for a defect that has been closed.

## crate census (T0 regression)

`$A/census/census-gate.sh`

T0 whole-crate parse over all 72 rustc compiler crates (census R6), gated at zero-regression vs
baseline.txt. F1: a PARSE outcome, not a resolve proxy. The heaviest gate (~10 min, 1847 files) and
ORACLE-source-dependent: it SKIPS cleanly where the pinned rustc source is not cached (CI stays host-free),
and FAILS on any T0->non-T0 regression or new CRASH where the source is present.

## arena headroom (self-emit at 75%)

`$A/sem0/arena-headroom.sh`

_(no inline rationale in the original)_

## harness synth (tower-built == golden)

`$HERE/../apparatus/arcs/2026-09-20-entry-harness-in-chain/harness-equiv-gate.sh`

entry/test-harness synthesizer (apparatus/arcs/2026-09-20-entry-harness-in-chain): build harness.elf in-gate
via SEM0 + the spine, assert it == the committed golden on the fixture (oracle-free). Gates the harness's
behaviour without promoting it into the trust root.

## RUN-slice (SEM0-linked cross-crate == golden)

`$HERE/../apparatus/arcs/2026-09-22-t3-run-pipeline-slices/run-slice-gate/run-slice-gate.sh`

RUN-slice: the cross-crate RUN capability + the SEM0-lowered linker in the trust path. SEM0-lowers link.rs
(0-UNLOWERED), links two //@SUPERSET crates (RS6 recursive-AST) with the self-hosted linker, runs the linked
ELF, asserts == a committed golden (oracle-free). Anchors the linker (9 execution-found fixes) + the
//@METADATA///@SUPERSET emitter path, self-hosted end-to-end; regressions fail the tower loudly.

## multi-crate RUN (pf -> lexer == golden)

`$HERE/../apparatus/arcs/2026-09-24-meta-telos-completion/locales/Plane-I-integration/multicrate-pf-lexer/multicrate-gate.sh`

MULTI-CRATE RUN: two interdependent REAL rustc crates (rustc_parse_format -> rustc_lexer) + driver, linked by
the SEM0-lowered N-crate linker, RUN == committed golden (oracle-free) on the cf spine AND (when llc/cc present)
the LLVM meta-telos backend. Vendored crate sources in-repo. The step up from single real-crate leaves to a
real cross-crate-dependency composition; 0 dangling rustc_lexer:: calls asserts the linkage is resolved.

## SEM0-native backend (cf.rs via SEM0 == cf.hoist)

`$A/sem0/sem0-native-backend-gate.sh`

_(no inline rationale in the original)_

## rzir->LLVM bridge parity (board)

`$A/backend-llvm/llvm-parity.sh`

rzir -> LLVM IR bridge board (Route B, apparatus/arcs/2026-09-27-rzir-llvm-bridge): the SELF-HOSTED translator
(backend-llvm/rzir2ll.rs, SEM0-compiled) drives the parity battery through .rs -> SEM0 -> rzir -> rzir2ll ->
llc -> cc -> native, vs rustc -O; zero-regression vs baseline.txt. SKIPS host-free (llc/cc external -- a
capability board on the amortization path to LLVM-linked rustc, NOT a hex-rooted trust gate).

## ourc (19 crates, tower-built SEM0 == golden)

`$A/ourc/ourc-gate.sh`

(LLVM-mint C2 now runs at the TOP of this block as the stage0 mint -- see STAGE0 PROMOTION above. It licenses
LLVM-minting: cf (hex-rooted) stays the authority + C2 reference; the LLVM frozen is admitted only when it
reproduces cf's emit byte-identically, and it is then the self-host stage0 the S-tier gates compiled with.)
TOP OF THE SOURCE-LANGUAGE STAIRCASE (added 2026-10-16): build `ourc` -- the 19 crates we authored plus the
odrv driver -- with the tower's OWN minted SEM0, and assert the fixture it compiles behaves == the committed
golden. Until now ourc was built only by the arc's build-ourc.sh, which takes its frontend from the
ENVIRONMENT (in practice a rustc-built SEM0 in a scratch dir), so the tower could be green from the hand-hex
to SEM0 while nothing connected ourc to it. Host-gated on llc/cc like the bridge board.

## semc (r2-corpus, tower-built SEM0 vs rustc)

`$A/semc/semc-gate.sh`

Added 2026-10-23. `scripts/semc.sh` (SEM0 -> rzir -> r2l -> llc -> cc) was only ever run by hand with a
rustc-built SEM0 (`sem0.head`) from a scratch path, so nothing showed that the compiler semc ships is the one
the tower derives from the hand-hex. This runs semc with the tower's raw frozen (`sem0.llvm.frozen`, else
`sem0.frozen`) and the tower-minted `r2l.elf` over all 904 r2-corpus probes, compares stdout + exit code with
`rustc -O` (oracle only), and FAILS on any probe whose `semc/baseline.txt` status is MATCH and no longer is.
First board (from-hex SEM0): MATCH 887, ORACLE-REJECT 15 (the unsound-reject probes rustc refuses to compile),
SEM0-REJECT 2 (nestgen_vecresult / nestgen_vectuple, the known nestgen gap), DIFF 0 -- i.e. 887/889 of the
probes rustc accepts. Parallel, ~15s. Host-gated on llc/cc/rustc like the bridge board and ourc.

## provenance (DDC, hex-rooted)

`$A/backend-toolchain/provenance.sh`

_(no inline rationale in the original)_

