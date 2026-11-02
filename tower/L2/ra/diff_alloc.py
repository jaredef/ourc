#!/usr/bin/env python3
"""Differential oracle for rzir-ra: slot-everything vs allocated pipeline.

For each fuzz seed: src -> ccc -> rzir-ld -> [rzir-ra] -> cf -> ELF -> exit code.
ρ* must be 1 on every terminating program: identical exit codes (or identical
skip/timeout class) between the slot pipeline and the allocated pipeline.

Persistent in-repo harness (the R5 process note: ephemeral-overlay copies vanish).
Checkpoints every 25 seeds so running-clean is distinguishable from stuck.

Speedups (all oracle-preserving):
- exec timeout 1.5s (100x margin over real corpus runtimes);
- known-hangs.txt caches seeds whose SLOT side timed out: on re-runs only the alloc
  side executes for them -- a miscompile that breaks OUT of the infinite loop still
  reports as DIVERGE (alloc terminates where slot is known not to);
- seeds run in parallel across cores (results deterministic; log order is not).
Usage: diff_alloc.py [N=400] [start=0]     env: RZIR_RA= alternate allocator
"""
import os, sys, subprocess, multiprocessing

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.expanduser("~/Developer/rusteze/apparatus/fuzz"))
import fuzz, fuzz_ccc

RA = os.environ.get("RZIR_RA") or os.path.join(HERE, "rzir-ra.py")
RA_CMD = ["python3", RA] if RA.endswith(".py") else [RA]   # M5: a compiled allocator runs directly
OPT = os.environ.get("RZIR_OPT")                           # rzir-opt R1: optional opt stage on the
OPT_CMD = OPT and (["python3", OPT] if OPT.endswith(".py") else [OPT])   # alloc side (opt|ra vs slot)
SKIPS = ("UB", "CC_OVERFLOW", "CC_FAIL", "CF_CRASH", "LD_FAIL")
HANGFILE = os.path.join(HERE, "known-hangs.txt")

def run_pipeline(cc, cf, src, d, i, alloc):
    rz = fuzz.sh([cc], input=src.encode(), timeout=10)
    if rz.returncode == 7: return "CC_OVERFLOW"
    if not rz.stdout: return "CC_FAIL"
    if fuzz._ub(rz.stdout.decode("latin1")): return "UB"
    ld = subprocess.run(["python3", fuzz.RZIRLD], input=rz.stdout, capture_output=True, timeout=30)
    if ld.returncode != 0: return "LD_FAIL"
    ir = ld.stdout
    if alloc:
        if OPT_CMD:
            op = subprocess.run(OPT_CMD, input=ir, capture_output=True, timeout=30)
            if op.returncode != 0: return "OPT_FAIL"
            ir = op.stdout
        ra = subprocess.run(RA_CMD, input=ir, capture_output=True, timeout=30)
        if ra.returncode != 0: return "RA_FAIL"
        ir = ra.stdout
    p = subprocess.run([cf], input=ir, capture_output=True, timeout=10)
    if p.returncode == 8: return "CC_OVERFLOW"
    if p.returncode != 0 or not p.stdout: return "CF_CRASH"
    elf = os.path.join(d, f"p{i}")
    with open(elf, "wb") as fh: fh.write(p.stdout)
    os.chmod(elf, 0o755)
    import time
    for _ in range(5):
        try: return subprocess.run([elf], capture_output=True, timeout=1.5).returncode
        except subprocess.TimeoutExpired: return "TIMEOUT"
        except OSError: time.sleep(0.02)
    return "CRASH"

def gen_key():
    # the cache is only valid for the generator that produced it: seed->program moved at R13
    import hashlib
    return hashlib.sha256(open(fuzz_ccc.__file__, "rb").read()).hexdigest()[:16]

def load_hangs():
    try: lines = open(HANGFILE).read().split()
    except OSError: return set()
    if not lines or lines[0] != gen_key(): return set()   # stale generator -> ignore cache
    return set(int(x) for x in lines[1:])

# worker globals (inherited via fork)
G_ctx = None

def test_seed(seed):
    """returns (seed, kind, detail): kind in 'ok'|'skip'|'hang'|'diverge'"""
    d, ccc, cf, knownhang = G_ctx
    src = fuzz_ccc.G(seed).program()
    try:
        if seed in knownhang:
            # slot side is known non-terminating; only a TERMINATING alloc side is a signal
            a = run_pipeline(ccc, cf, src, d, 1000000 + seed, alloc=True)
            if a == "TIMEOUT" or a in SKIPS: return (seed, "hang", None)
            return (seed, "diverge", ("TIMEOUT(known)", a, src))
        s = run_pipeline(ccc, cf, src, d, seed, alloc=False)
        a = run_pipeline(ccc, cf, src, d, 1000000 + seed, alloc=True)
    except subprocess.TimeoutExpired:
        return (seed, "hang", "stage")
    if s in SKIPS and a in SKIPS: return (seed, "skip", None)
    if s == "TIMEOUT" and a == "TIMEOUT": return (seed, "hang", "new")
    if s != a: return (seed, "diverge", (s, a, src))
    return (seed, "ok", None)

def main():
    global G_ctx
    N = int(sys.argv[1]) if len(sys.argv) > 1 else 400
    start = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    d, ccc, m0cc, cf = fuzz_ccc.build()
    knownhang = load_hangs()
    G_ctx = (d, ccc, cf, knownhang)
    nworkers = max(2, (os.cpu_count() or 4) - 2)
    diverge = []; skip = 0; hang = 0; tested = 0; done = 0; newhangs = []
    with multiprocessing.Pool(nworkers) as pool:
        for (seed, kind, detail) in pool.imap_unordered(test_seed, range(start, start + N)):
            done += 1
            if kind == "skip": skip += 1
            elif kind == "hang":
                hang += 1
                if detail == "new": newhangs.append(seed)
            elif kind == "diverge":
                s, a, src = detail
                diverge.append(seed)
                fn = os.path.join(fuzz.HERE, "findings", f"alloc-diverge-seed{seed}.c")
                open(fn, "w").write(f"/* seed {seed}: slot={s} alloc={a} DIVERGE */\n" + src)
                print(f"  ** DIVERGE seed={seed}: slot={s} alloc={a} -> {fn}", flush=True)
            else: tested += 1
            if done % 25 == 0:
                print(f"[checkpoint] {done}/{N}: tested {tested}, skip {skip}, hang {hang}, diverge {len(diverge)}", flush=True)
    if newhangs:
        allhangs = sorted(knownhang | set(newhangs))
        with open(HANGFILE, "w") as fh:
            fh.write(gen_key() + "\n" + "".join(f"{s}\n" for s in allhangs))
    print(f"alloc differential: {N} seeds ({start}..{start+N-1}): tested {tested}, skip {skip}, "
          f"hang {hang} ({len(newhangs)} newly cached), DIVERGENCES {len(diverge)} {sorted(diverge)}", flush=True)

main()
