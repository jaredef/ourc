#!/usr/bin/env python3
"""rzir-ra -- register allocator over rzir (behavior-preserving).

R3: linear-scan allocation over the 2-register lineloop-disjoint pool {r12, r13}, emitting
`@r12`/`@r13` on the def of each assigned value. cf currently IGNORES the annotation (its
d_assign finds "= " and skips the LHS), so behavior is identity and the differential oracle
gates it. The cf-honor lands in R4. A built-in self-check asserts the allocation is sound
(no two overlapping-interval values share a register).

Candidate class (cf-tractable for R4): single-def (not a ccc-reused vN), interval does not
cross a call, and every non-def use is operand-A of its consumer (avoids reg-source op
encoding). Linear-scan assigns the 2-register pool; overflow stays in slots (the graceful
floor). Emits identity for non-candidates.
"""
import sys, re, os

VAL  = re.compile(r'\bv(\d+)\b')
DEF  = re.compile(r'^(\s*)v(\d+):\s*(\w+)\s*=\s*(\w+)')     # indent, vN, type, OP
CALL = ('call ', 'calli ', 'syscall ', 'param')   # R15: param lines READ the marshalling regs
                                                  # at fn entry -- a value crossing one is CS-only
# chain-arm64-linux-codegen R3: pool selection is target-dependent. Three internal categories
# mirror the x86_64 lane: `cs` (callee-saved; only legal home for across-call values), `safe`
# (caller-saved non-arg scratch; free at call sites), `marsh` (arg-marshalling caller-saved;
# overwritten by call/syscall/param). aarch64 maps: x19/x20 = cs, x16/x17 (IP0/IP1) = safe,
# x0..x7 = marsh (AAPCS64 arg regs). Default `--target=x86_64-linux` preserves the pre-R3
# pool byte-identically (backward-compat / H2).
TARGETS = {
    'x86_64-linux': {
        'cs':    ['r12', 'r13'],
        'safe':  ['r10', 'r11'],
        'marsh': ['rsi', 'rdi', 'r8', 'r9'],
    },
    'arm64-linux': {
        'cs':    ['x19', 'x20'],
        'safe':  ['x16', 'x17'],
        'marsh': ['x0', 'x1', 'x2', 'x3', 'x4', 'x5', 'x6', 'x7'],
    },
    # darwin-reg-honoring D0: the Darwin-lane pool (arc.md design). cf-darwin's emitted code
    # reserves x9-x12 (frame/scratch), x16 (svc nr), x17 (wide-slot/lr scratch, T19/T21/T25),
    # x18 (platform-reserved -- NEVER allocate). Across-call pool x19/x20 (r12/r13 analog);
    # local pool x13,x14,x15,x21,x22,x23 mapped onto the x86 safe/marsh split in analog order
    # (r10/r11 -> x13/x14 as `safe`; rsi/rdi/r8/r9 -> x15/x21/x22/x23 as `marsh`). None of the
    # pool regs are AAPCS64 arg regs, so the split is structural only (keeps the allocator's
    # queue discipline byte-comparable to the x86 lane).
    # darwin-ra-quality Q1: pool widened 8->13. x24-x28 are free in EMITTED
    # code (cf's own x24-x28 uses are compile-time T21/T25 counters); they
    # join the LOCAL (marsh) pool -- no save discipline needed because the
    # allocator never gives a marsh/safe register to an across-call value,
    # so callee clobbers can't be observed. Across-call stays x19/x20 (D2
    # save discipline). x18 remains FORBIDDEN (platform register).
    'arm64-darwin': {
        'cs':    ['x19', 'x20'],
        'safe':  ['x13', 'x14'],
        'marsh': ['x15', 'x21', 'x22', 'x23', 'x24', 'x25', 'x26', 'x27', 'x28'],
    },
}
DEFAULT_TARGET = 'x86_64-linux'
# darwin-reg-honoring D0: chain binaries are stdin->stdout filters, and the Darwin chain
# invokes rzir-ra.py with no argv -- pool selection rides an env var (the GBASE-style seam
# for the oracle). Absent -> DEFAULT_TARGET (x86 pool, byte-identical: the do-no-harm gate).
ENV_POOLS = {'darwin': 'arm64-darwin'}

def alloc_function(lines, pools, widen_def=frozenset(), weighted=False):
    POOL_CS = pools['cs']; POOL_SAFE = pools['safe']; POOL_MARSH = pools['marsh']
    """return {def_line_index -> regname} for values assigned a register."""
    defs = {}; last_use = {}; def_lines = {}; def_ops = {}
    seen = set(); reused = set(); ucnt = {}
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        dv = int(m.group(2)) if m else None
        if m:
            if dv in seen: reused.add(dv)
            seen.add(dv)
            if dv not in defs: defs[dv] = i
            def_lines.setdefault(dv, []).append(i)
            def_ops.setdefault(dv, set()).add(m.group(4))
            # R22 fix: a mutable's TRAILING DEAD DEF still writes its register, so every def
            # extends the interval (end = max(last use, last def)). Without this, another value
            # can take the register in the gap and be clobbered by the dead store (seed 133).
            if dv in reused: last_use[dv] = max(last_use.get(dv, i), i)
        rhs = ln.split('=', 1)[1] if '=' in ln else ln
        for v in (int(x) for x in VAL.findall(rhs)):
            last_use[v] = i
            ucnt[v] = ucnt.get(v, 0) + 1   # Q2: spill-cost use count (every RHS occurrence)
    # R11: back-edge-aware intervals. A value defined BEFORE a loop and last-used INSIDE it is
    # live across the back edge -- the next iteration re-reads it after later-line code ran. Its
    # interval must extend to the loop end or a later value could take its register mid-body.
    # (ccc's emission shape -- statement-local temps, mutable vars as reused vN -- masked this;
    # honoring brz uses invites exactly this pattern, so the model is closed now.)
    # R14 fix: the loop region's popper is `jmp` (cf's back-jump, which pops the LP stack), NOT
    # `pool`. In ccc-emitted whiles, `pool` closes brz-IFS inside the body (loop .. cont/jmp/
    # endloop is the loop shape) -- pairing loop with the first pool truncated the region at the
    # first inner if, leaving the loop tail unprotected. Latent while mutables were excluded
    # (statement-local temps rarely span loops); mutable allocation made it live (fn eq_pos).
    # The back edge re-executes only lines up to the jmp, so [loop, jmp] is the exact region.
    regions = []; lstack = []
    labels = {}
    for i, ln in enumerate(lines):
        if re.match(r'\s*loop\b', ln): lstack.append(i)
        elif re.match(r'\s*jmp\b', ln) and lstack: regions.append((lstack.pop(), i))
        else:
            m3 = re.match(r'\s*label\s+(\w+)', ln)
            if m3: labels[m3.group(1)] = i; continue
            m3 = re.match(r'\s*goto\s+(\w+)', ln)
            if m3 and m3.group(1) in labels:      # R13: backward goto = a back edge; same
                regions.append((labels[m3.group(1)], i))   # clobber hazard as loop/pool
    changed = True
    while changed:
        changed = False
        for v, d in defs.items():
            lu = last_use.get(v, d)
            for (s, e) in regions:
                if d < s and s <= lu < e:
                    last_use[v] = e; changed = True
    # darwin-ra-quality Q2: spill weights. weight(v) = use_count(v) * 4^loop_depth,
    # loop_depth = the MAX region-nesting depth over v's def lines and use lines
    # (regions = the R11/R13 back-edge list above). Caps keep the arithmetic in
    # small ints for the .c twin (T22): depth <= 5, use count <= 4095.
    wdep = {}
    if weighted:
        ldep = [0] * len(lines)
        for (s, e) in regions:
            for i in range(s, e + 1): ldep[i] += 1
        for i, ln in enumerate(lines):
            d = min(ldep[i], 5)
            m = DEF.match(ln)
            if m:
                dv = int(m.group(2))
                if d > wdep.get(dv, 0): wdep[dv] = d
            rhs = ln.split('=', 1)[1] if '=' in ln else ln
            for v in (int(x) for x in VAL.findall(rhs)):
                if d > wdep.get(v, 0): wdep[v] = d
    def weight(v):
        return min(ucnt.get(v, 0), 4095) << (2 * wdep.get(v, 0))
    # R21: a call AT the last-use line is NOT across -- the value's only involvement there is
    # being an arg, read before the call clobbers. (If lu was region-extended, lines[lu] is a
    # jmp/goto, never a call, so the exclusion cannot fire on extended intervals.) Calls
    # STRICTLY inside the interval still force callee-saved.
    across = set()
    for v, d in defs.items():
        lu = last_use.get(v, d)
        if any(any(c in lines[i] for c in CALL) for i in range(d+1, lu)): across.add(v)
    ARITH = {'add','sub','mul','and','or','xor',
             'shl','shr','shu'}   # R8: cf honors the cl-count path (mov ecx,r12d/r13d; reg store)
    CMP = {'lt','le','gt','ge','eq','ne',
           'ltu','gtu','leu','geu'}   # R10: cf's cmp_body honors registers in A, B, and the store
    DIV = {'div','mod','duv','udm'}   # R19: the div family honors A (fwd-aware), divisor B, and the store
    MEM = {'load','get','getb'}       # R22: memory reads -- addr/idx are 32-bit reg reads (the whole
                                      # address space sits under 4GB; pre_movrc_a was already a 32-bit
                                      # mov), results route through bop_store. load8/store8/ref (true
                                      # 8-byte) stay excluded.
    HONORED = ARITH | CMP | DIV | MEM
    CALLOPS = {'call', 'syscall'}     # R20: arg marshalling honors register sources (calli excluded:
                                      # its function-pointer operand is a slot read; >6-arg calls
                                      # excluded: the spill path reads slots)
    BRZ = re.compile(r'\s*(brz|brc)\b')   # R11 brz; R3(opt) brc: cf's fused compare-branch
                                           # honors registers in both operand positions
    RET = re.compile(r'\s*ret\b')   # R12: cf's d_ret honors register results (mov eax/edi, rXd)
    STO = re.compile(r'\s*(store|storeb|put|putb)\b')   # R22: all operand positions honored
    arguse = set()   # R20: values consumed as call/syscall args -- restricted to r10-r13
                     # (the marshalling sequence overwrites rdi/rsi/r8/r9 left-to-right)
    def uses_ok(v):   # every non-def use must sit in a cf-honored consumer
        for i in range(defs[v]+1, last_use.get(v, defs[v])+1):
            ln = lines[i]
            rhs = ln.split('=',1)[1] if '=' in ln else ln
            if not re.search(r'\bv%d\b' % v, rhs): continue
            m2 = DEF.match(ln)
            if m2 and m2.group(4) in HONORED: continue
            if m2 and m2.group(4) in CALLOPS and len(VAL.findall(rhs)) <= 6:
                arguse.add(v); continue
            if BRZ.match(ln) or RET.match(ln) or STO.match(ln): continue
            return False   # consumed elsewhere -> unsafe
        return True
    # R14 (the §5 payoff): MUTABLE values are candidates. Textbook SSA-restore (rename each
    # redefinition) needs phi functions at merges, which cf lacks -- but the single-location
    # semantics that made reused vN excludable also makes them allocatable WITHOUT SSA: give the
    # value ONE register and have every def re-store it, every use read it. Mutation = register
    # re-store, exactly cf's slot model one substrate up. Requires: every def op honored
    # (const now is -- cf's a_const emits mov rXd,imm32) and every use honored; EVERY def line
    # is annotated (cf resets LOC[N] at each unannotated def). Interval = first def..last use
    # with the R13 back-edge extension, which this rung makes load-bearing on real ccc output.
    HONORED_DEF = HONORED | widen_def | {'const',
                             'ref'}   # R23: ref DEFS are honored (the address fits 32 bits) but ref
                                      # stays an UNHONORED CONSUMER: `ref vX` aliases X, so any
                                      # address-taken value keeps its slot -- baduse IS the aliasing
                                      # discipline.
    cands = [v for v in defs
             if last_use.get(v, defs[v]) > defs[v]
             and def_ops.get(v, set()) <= HONORED_DEF and uses_ok(v)]
    # R9: linear scan over two pools. Across-call values may ONLY take callee-saved r12/r13;
    # non-across values prefer the caller-saved r10/r11 (free: no prologue cost) and fall back
    # to r12/r13 when both are idle.
    cands.sort(key=lambda v: defs[v])
    active = []            # (last_use, reg, v)
    # R20: three queues. Arg-consumed values may only take r10/r11 (safe) or r12/r13 (cs);
    # plain values prefer the marshalling regs so the safe ones stay free for arg values.
    free_cs = list(POOL_CS); free_safe = list(POOL_SAFE)
    free_marsh = list(POOL_MARSH); assign = {}
    SAFE_SET = set(POOL_SAFE); CS_SET = set(POOL_CS)
    def release(r):
        (free_cs if r in CS_SET else free_safe if r in SAFE_SET else free_marsh).append(r)
    for v in cands:
        d = defs[v]; lu = last_use[v]
        active = [(e, r, w) for (e, r, w) in active if e >= d or release(r)]  # expire
        active.sort()
        if v in across:
            r = free_cs.pop(0) if free_cs else None
        elif v in arguse:
            r = free_safe.pop(0) if free_safe else (free_cs.pop(0) if free_cs else None)
        else:
            r = (free_marsh.pop(0) if free_marsh else
                 free_safe.pop(0) if free_safe else
                 free_cs.pop(0) if free_cs else None)
        # Q2 (weighted, arm64-darwin only): pool exhausted -> evict the CHEAPEST
        # active whose register is legal for v, iff strictly cheaper than v.
        # The evictee just loses its annotation (keeps its slot for its whole
        # life -- no code motion). Deterministic victim key:
        # (weight, end, regname, vN); ties keep the incumbent.
        if r is None and weighted:
            if v in across:      allowed = CS_SET
            elif v in arguse:    allowed = SAFE_SET | CS_SET
            else:                allowed = SAFE_SET | CS_SET | set(POOL_MARSH)
            cand2 = [(weight(w), e, rr, w) for (e, rr, w) in active if rr in allowed]
            if cand2:
                wt, e, rr, w = min(cand2)
                if wt < weight(v):
                    del assign[w]
                    active = [(e2, r2, w2) for (e2, r2, w2) in active if w2 != w]
                    r = rr
        if r:
            assign[v] = r; active.append((lu, r, v))
        # else: no register -> stays in slot
    # SELF-CHECK: no two assigned values with overlapping intervals share a register
    byreg = {}
    for v, r in assign.items():
        for (w, rr) in byreg.get(r, []):
            if defs[v] <= last_use[w] and defs[w] <= last_use[v]:
                sys.stderr.write("rzir-ra: UNSOUND overlap v%d,v%d on %s\n" % (v, w, r)); sys.exit(3)
        byreg.setdefault(r, []).append((v, last_use[v]))
    # R14: annotate EVERY def line of an assigned value (cf resets LOC[N] per unannotated def)
    return {i: assign[v] for v in assign for i in def_lines[v]}, len(cands), len(assign)

def emit(lines, out, idx_to_reg):
    for i, ln in enumerate(lines):
        if i in idx_to_reg:
            ln = re.sub(r'(=)', '@%s \\1' % idx_to_reg[i], ln, count=1)  # insert @reg before '='
        out.append(ln)

def main():
    target = DEFAULT_TARGET
    envpool = os.environ.get('RZIR_RA_POOL')
    if envpool is not None:
        target = ENV_POOLS.get(envpool, envpool)
    for a in sys.argv[1:]:
        if a.startswith("--target="):
            target = a.split("=", 1)[1]
        else:
            sys.stderr.write("rzir-ra: unrecognized arg: %s\n" % a); sys.exit(2)
    if target not in TARGETS:
        sys.stderr.write("rzir-ra: unrecognized --target=%s; recognized: %s\n"
                         % (target, ", ".join(sorted(TARGETS))))
        sys.exit(2)
    pools = TARGETS[target]
    # darwin-reg-honoring D9: cf-darwin honors call/syscall/argc RESULTS
    # (x0_store LOC dispatch), so their defs join HONORED_DEF -- for the
    # Darwin pool ONLY (the x86 lane's cf has no such consumer; the default
    # target stays byte-identical -- the do-no-harm gate).
    widen_def = frozenset(('call', 'syscall', 'argc')) if target == 'arm64-darwin' else frozenset()
    # darwin-ra-quality Q2: spill-cost-aware eviction, arm64-darwin pool ONLY
    # (DESIGN CHOICE of record: WEIGHTED off for the default/x86 target keeps
    # the do-no-harm gate byte-identical by construction; the x86 lane can opt
    # in via its own measured rung if it ever wants to).
    weighted = (target == 'arm64-darwin')
    text = sys.stdin.read(); lines = text.splitlines()
    funcs = []; pre = []; cur = None
    for ln in lines:
        if re.match(r'(ir main|fn )', ln):
            cur = [ln]; funcs.append(cur)
        elif cur is not None: cur.append(ln)
        else: pre.append(ln)
    out = list(pre); tcand = tass = 0
    for f in funcs:
        idx, nc, na = alloc_function(f, pools, widen_def, weighted); tcand += nc; tass += na
        emit(f, out, idx)
    pool_desc = ",".join(pools['cs'] + pools['safe'] + pools['marsh'])
    sys.stderr.write("rzir-ra: %d funcs, %d candidates, %d assigned (%s pool: %s)\n"
                     % (len(funcs), tcand, tass, target, pool_desc))
    sys.stdout.write("\n".join(out) + "\n")
main()
