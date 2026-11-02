# SEM0 stage0 — freeze/promotion log

## 2026-10-02 — LLVM-minted frozen PROMOTED as the self-host stage0 (keeper-authorized)
The binary that compiles SEM0's own dev source (`s1_parser.rs`) in the S-tier gates is now the **LLVM-minted
frozen** (`sem0.llvm.cc` from `backend-llvm/sem0-stage-llvm.sh`), not the cf-staircase binary.

WHY. The cf mint (`sem0-stage.sh`) is a ~27-stage staircase (SEM0.FROZEN.rs -> STAGE2 -> 2B..2W -> 3 -> 4 -> 5)
built to dodge the L6-seed emit-capacity wall at each hop as the dev grows. The LLVM-built frozen has no such
wall -> it mints the full current dev in ONE hop.

TRUST (Tier-2 C2 self-host-stage0 admission; two-tier law amended this day in the project rules +
apparatus/docs/tower-naming-and-trust-path.md). Admitted BY RESULT, not provenance:
- The hex-rooted cf bootstrap (`sem0-stage.sh`) still runs FIRST -- it seeds the `rzir2ll` translator and IS the
  C2 reference.
- `sem0-stage-llvm.sh` GUARDS C2 (llvm-frozen emit of the dev == cf-bootstrap emit, byte-identical) + C4
  (self-fixpoint). A C2 FAIL aborts the promotion -- a divergent LLVM frozen can never be the stage0.
- `provenance.sh` (DDC, hex-rooted) still re-derives the Tier-1 chain from the hand-hex independently.
- So the LLVM frozen changes only HOW the compiler binary was codegen'd, never WHAT rzir it emits (hex-decided).
The cf staircase remains the from-hex fallback + trust reference. The L0-L6 substrate below SEM0 stays cf/hex only.

VERIFIED by a full cold `SP=<fresh> bash tower/build-tower.sh --gates`: `stage0 = LLVM-minted frozen
(C2-admitted)`, all S1-S9 + r2-corpus(891 OK) + D0 + census + arena-headroom + harness-synth + RUN-slice +
SEM0-native + rzir->LLVM bridge parity ok, and `provenance (DDC, hex-rooted) ok`. == build-tower: PASS ==.
The promoted dev also carries this session's RUN-plane work (LLVM board 891/904 = zero real r2-corpus MISCOMPs).
