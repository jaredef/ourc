# tower/ourc -- ourc in the tower's chain

`ourc-gate.sh` builds `ourc` (the 19 crates in
`ourc/src/` plus the `odrv` driver) with the tower's OWN
minted SEM0, and asserts ourc's behaviour on `fixture.rs` against the committed `fixture.golden`
(stdout + exit code). Oracle-free at run time. Host-gated on `llc`/`cc`, like the LLVM bridge board.

Added 2026-10-16. Before it, ourc was built only by the arc's `build-ourc.sh`, whose frontend, translator
and `llc`/`cc` all come from the ENVIRONMENT -- in practice a rustc-built SEM0 in a scratch dir. The tower
could therefore be green from the hand-hex up to SEM0 while nothing connected ourc to it.

## What the first cold run found (2026-10-16) -- and what it actually was

The tower-minted ourc **segfaulted immediately**, on any input, before reading a file, while the
rustc-built frontend's ourc compiled and ran correctly. Bisected by swapping one component at a time: the
rzir->LLVM translator was not involved (tower's and the scratch one both worked) and it was not
LLVM-specific (the cf mint broke it too) -- it was the frontend. Comparing the two frontends per crate
named a crisp symptom:

```
metadata:  rustc-built = 2600 META lines     "minted" = 0
```

**The cause was this gate, not the compiler.** It took its frontend from `$SEM0_CC`, and the `.cc`
artifacts the stages mint are **wrappers**:

```sh
t="$(mktemp)"; { echo '//@S9'; cat; } > "$t"; "$SP/sem0.llvm.frozen" < "$t"
```

They prepend `//@S9` -- they exist to be "the compiler that compiles the dev source in S9 mode". So
`//@METADATA` was no longer the first line and was ignored, as was `//@SUPERSET`: every crate lowered with
no cross-crate facts and no crate-tagged symbols, bare names collided across 19 crates, the image still
linked, and it crashed.

Swept every directive against the **raw frozen** instead, and it honours all of them -- byte-identical
first lines to the rustc-built frontend on `//@S4HIR`, `//@S6MONO`, `//@S7CHECK`, `//@SUPERSET` and
`//@METADATA`. **There is no metadata self-host gap.** An earlier note here and in memory said there was;
that was this wrapper, and it is retracted.

The gate now resolves `$SP/sem0.llvm.frozen` (the C2-admitted mint) or `$SP/sem0.frozen`, never a `.cc`
wrapper, and asserts the `//@METADATA` directive is live BEFORE building -- a frontend that ignores it
builds an ourc that links and then crashes three steps later, so the check belongs where it names itself.

Worth keeping: the probe that found it was an additive sweep over marker length (5, 8, 9, 10, 11 bytes).
Only `//@S9` "worked", which looked like a chain-length threshold in the detection code -- an 11-term `&&`
chain of byte compares. A direct sweep of `&&`-chain length 2..16 through the minted frontend came back
**unbounded** (every length correct), which killed that reading and sent the search to the harness, where
the defect was. The axis that does not bend is the one that shrinks the engine.
