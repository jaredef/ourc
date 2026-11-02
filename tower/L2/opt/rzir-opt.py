#!/usr/bin/env python3
"""rzir-opt -- optimizer over rzir (behavior-preserving).

M0 (R1): the IDENTITY pass + the census. The analysis computes everything M1-M3 will act on
-- the const table, fold fixpoint, zero-idiom copies, the dead-value cascade, cmp/brz pairs
-- and emits the input UNCHANGED, so the pass is provably identity this rung and the census
prices the ladder (completeness-as-threshold: rungs priced by conjuncts retired).

Value classes (spec section 1): pure single-def values may fold/propagate/die; mutables
(reused vN) are the fold barrier -- nothing flows THROUGH them; address-taken values
(ref operands) are untouchable; memory/effect lines are never moved or deleted.
"""
import sys, re, os

VAL = re.compile(r'\bv(\d+)\b')
DEF = re.compile(r'^(\s*)v(\d+):\s*(\w+)\s*=\s*(\w+)')
ARITH = {'add','sub','mul','and','or','xor','shl','shr','shu'}
CMPOP = {'lt','le','gt','ge','eq','ne','ltu','gtu','leu','geu'}
PURE = ARITH | CMPOP | {'const','ref'}
M = (1 << 32) - 1

def fold1(op, a, b):
    """i32 wraparound semantics = cf's. Returns None when the fold is unsafe/undefined."""
    if a >> 32 or b >> 32: return None  # 64-bit address const (Darwin GBASE > 2^32): the i32
                                        # fold would truncate it -- leave as a runtime 64-bit add.
    ua, ub = a & M, b & M
    sa = ua - (1 << 32) if ua >> 31 else ua
    sb = ub - (1 << 32) if ub >> 31 else ub
    if op == 'add': return (ua + ub) & M
    if op == 'sub': return (ua - ub) & M
    if op == 'mul': return (ua * ub) & M
    if op == 'and': return ua & ub
    if op == 'or':  return ua | ub
    if op == 'xor': return ua ^ ub
    if op in ('shl','shr','shu'):
        if not 0 <= ub <= 31: return None   # hardware masks; do not fold out-of-range counts
        if op == 'shl': return (ua << ub) & M
        if op == 'shu': return ua >> ub
        return (sa >> ub) & M
    if op == 'lt': return int(sa < sb)
    if op == 'le': return int(sa <= sb)
    if op == 'gt': return int(sa > sb)
    if op == 'ge': return int(sa >= sb)
    if op == 'eq': return int(sa == sb)
    if op == 'ne': return int(sa != sb)
    if op == 'ltu': return int(ua < ub)
    if op == 'gtu': return int(ua > ub)
    if op == 'leu': return int(ua <= ub)
    if op == 'geu': return int(ua >= ub)
    return None

def census_function(lines, stats):
    defs = {}; def_ops = {}; seen = set(); reused = set(); addr_taken = set()
    uses = {}          # v -> list of (line, consumer op or head)
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        dv = int(m.group(2)) if m else None
        if m:
            if dv in seen: reused.add(dv)
            seen.add(dv)
            defs.setdefault(dv, i); def_ops.setdefault(dv, m.group(4))
        rhs = ln.split('=', 1)[1] if '=' in ln else ln
        head = m.group(4) if m else (ln.split() or [''])[0]
        if head == 'ref':
            for v in (int(x) for x in VAL.findall(rhs)): addr_taken.add(v)
        for v in (int(x) for x in VAL.findall(rhs)):
            if v != dv: uses.setdefault(v, []).append((i, head))
    pure1 = {v for v in defs
             if v not in reused and v not in addr_taken and def_ops[v] in PURE}
    # const table + fold fixpoint (simulated)
    val = {}
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        if m and int(m.group(2)) in pure1 and m.group(4) == 'const':
            try: val[int(m.group(2))] = int(ln.split('=',1)[1].split()[1])  # no & M: 64-bit addr consts must survive ingest (T22); fold1 refuses them
            except (ValueError, IndexError): pass
    stats['const_defs'] += sum(1 for v in defs if def_ops[v] == 'const')
    # dup consts: same-K const defs beyond the first, per function (the value-numbering class)
    kcount = {}
    for v in defs:
        if def_ops[v] == 'const' and v not in reused and v in val: kcount[val[v]] = kcount.get(val[v], 0) + 1
    stats['dup_consts'] += sum(c - 1 for c in kcount.values() if c > 1)
    folded = set(); changed = True
    while changed:
        changed = False
        for i, ln in enumerate(lines):
            m = DEF.match(ln)
            if not m: continue
            dv = int(m.group(2)); op = m.group(4)
            if dv not in pure1 or dv in val or dv in folded or op not in (ARITH | CMPOP): continue
            ops = [int(x) for x in VAL.findall(ln.split('=',1)[1])]
            if len(ops) == 2 and all(o in val for o in ops):
                r = fold1(op, val[ops[0]], val[ops[1]])
                if r is not None:
                    val[dv] = r; folded.add(dv); changed = True
    stats['foldable'] += len(folded)
    # zero-idiom copies: vK = add vX v0 with val[v0]==0 (either order), vK pure single-def
    copies = 0
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        if not m: continue
        dv = int(m.group(2))
        if dv not in pure1 or m.group(4) != 'add' or dv in folded: continue
        ops = [int(x) for x in VAL.findall(ln.split('=',1)[1])]
        if len(ops) == 2 and any(val.get(o) == 0 for o in ops): copies += 1
    stats['zero_copies'] += copies
    # dead-value cascade after simulated fold+copy-prop: pure values whose every use is on a
    # line that itself dies (a folded def) or that vanish outright
    dead = set(); changed = True
    while changed:
        changed = False
        for v in list(pure1):
            if v in dead: continue
            remaining = []
            for (li, head) in uses.get(v, []):
                m2 = DEF.match(lines[li])
                consumer = int(m2.group(2)) if m2 else None
                # a use vanishes if its consuming line itself folds away or dies
                if consumer is not None and consumer in (folded | dead): continue
                remaining.append((li, head))
            if not remaining:
                dead.add(v); changed = True
    stats['dead_after'] += len(dead)
    # cmp/brz fusable pairs: vC cmp-class pure, sole use = the immediately following brz vC
    fus = 0
    for i, ln in enumerate(lines[:-1]):
        m = DEF.match(ln)
        if not m: continue
        dv = int(m.group(2))
        if dv in pure1 and m.group(4) in CMPOP and len(uses.get(dv, [])) == 1:
            u = uses[dv][0]
            if u[0] == i + 1 and re.match(r'\s*brz\b', lines[i+1]): fus += 1
    stats['cmp_brz'] += fus
    stats['values'] += len(defs)
    stats['lines'] += len(lines)

def analyze(lines):
    """value classes for one function: defs, def_ops, reused, addr_taken, uses, pure1."""
    defs = {}; def_ops = {}; seen = set(); reused = set(); addr_taken = set(); uses = {}
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        dv = int(m.group(2)) if m else None
        if m:
            if dv in seen: reused.add(dv)
            seen.add(dv)
            defs.setdefault(dv, i); def_ops.setdefault(dv, m.group(4))
        rhs = ln.split('=', 1)[1] if '=' in ln else ln
        head = m.group(4) if m else (ln.split() or [''])[0]
        if head == 'ref':
            for v in (int(x) for x in VAL.findall(rhs)): addr_taken.add(v)
        for v in (int(x) for x in VAL.findall(rhs)):
            if v != dv: uses.setdefault(v, []).append(i)
    pure1 = {v for v in defs
             if v not in reused and v not in addr_taken and def_ops[v] in PURE}
    return defs, def_ops, reused, addr_taken, uses, pure1

def opt_function(lines, stats):
    """M1: fold + const value-numbering + zero-idiom copy-prop + dead-value cascade,
    iterated to a fixpoint. Returns the rewritten line list."""
    changed_any = True
    while changed_any:
        changed_any = False
        defs, def_ops, reused, addr_taken, uses, pure1 = analyze(lines)
        val = {}
        for v in pure1:
            if def_ops[v] == 'const':
                try: val[v] = int(lines[defs[v]].split('=',1)[1].split()[1])  # no & M (T22)
                except (ValueError, IndexError): pass
        subst = {}   # dropped vN -> canonical survivor vN
        drop = set() # line indices to delete
        # FOLD: all-const arith/cmp -> rewrite the line to const R (stays; dedup may drop it next round)
        for i, ln in enumerate(lines):
            m = DEF.match(ln)
            if not m: continue
            dv = int(m.group(2)); op = m.group(4)
            if dv not in pure1 or op not in (ARITH | CMPOP): continue
            ops = [int(x) for x in VAL.findall(ln.split('=',1)[1])]
            if len(ops) == 2 and all(o in val for o in ops):
                r = fold1(op, val[ops[0]], val[ops[1]])
                if r is not None:
                    # cf's const parser is digit-only: emit the UNSIGNED 32-bit form (its
                    # decimal accumulator wraps mod 2^32, so the value round-trips exactly)
                    lines[i] = "%sv%d: %s = const %d" % (m.group(1), dv, m.group(3), r & M)
                    stats['r_fold'] += 1; changed_any = True
        if changed_any: continue   # re-analyze so new consts join val[]
        # CONST POOLING (value-numbering with dominance): textual order is NOT dominance --
        # a survivor inside a brz-skipped arm or zero-trip region leaves its slot
        # unmaterialized on paths that still reach a substituted use (the seed-0 falsifier).
        # Consts have no operands, so ONE def per K is HOISTED to the function's unskippable
        # prefix (after the leading param/argc block), which dominates every use; all other
        # same-K defs drop and their uses substitute the pooled name.
        # MEASURED OFF BY DEFAULT (R2): pooling regressed ccc +4.6KB through opt|ra --
        # hoisted consts hold whole-function intervals, grab all 8 registers first in linear
        # scan, and starve every other value. Rematerialization (mov rXd,imm32 at each use
        # site) is CHEAPER than sharing under register pressure. Re-enable via OPT_POOL=1 to
        # re-measure if cf ever gains op-imm forms or rzir-ra gains spill-aware priority.
        firstK = {}; hoist = []
        if os.environ.get('OPT_POOL'):
            for i, ln in enumerate(lines):
                m = DEF.match(ln)
                if not m: continue
                dv = int(m.group(2))
                if dv in pure1 and dv in val and def_ops[dv] == 'const':
                    k = val[dv]
                    if k in firstK:
                        if firstK[k] != dv: subst[dv] = firstK[k]; drop.add(i)
                    else:
                        firstK[k] = dv; hoist.append(i)
        # insertion point: after the header line and the leading param/argc defs
        ins = 1
        while ins < len(lines):
            m = DEF.match(lines[ins])
            if m and m.group(4) in ('param', 'argc'): ins += 1
            else: break
        moving = [i for i in hoist if i >= ins]
        if subst or moving:
            moved = [lines[i] for i in moving]
            head = [ln for i, ln in enumerate(lines) if i < ins and i not in drop]
            tail = [ln for i, ln in enumerate(lines)
                    if i >= ins and i not in drop and i not in moving]
            final = head + moved + tail
            if subst:
                def canon0(v):
                    while v in subst: v = subst[v]
                    return v
                final = [VAL.sub(lambda mm: 'v%d' % canon0(int(mm.group(1))), ln) for ln in final]
            if final != lines:                     # already-pooled defs re-collect each round;
                stats['r_dedup'] += len(subst)     # only a real rewrite restarts the fixpoint
                lines = final; changed_any = True
                continue
            subst = {}; drop = set()               # no-op round: fall through to copy-prop/DVE
        # ZERO-IDIOM COPY-PROP: vK = add vX v0 (val 0, either order), vK AND vX pure single-def
        for i, ln in enumerate(lines):
            m = DEF.match(ln)
            if not m: continue
            dv = int(m.group(2))
            if dv not in pure1 or def_ops[dv] != 'add' or i in drop or dv in subst: continue
            ops = [int(x) for x in VAL.findall(ln.split('=',1)[1])]
            if len(ops) != 2: continue
            zi = [o for o in ops if val.get(o) == 0]
            xs = [o for o in ops if val.get(o) != 0]
            if len(zi) == 1 and len(xs) == 1 and (xs[0] in pure1 or val.get(xs[0]) is not None):
                subst[dv] = xs[0]; drop.add(i); stats['r_copy'] += 1
        # resolve substitution chains to the canonical survivor
        def canon(v):
            while v in subst: v = subst[v]
            return v
        subst = {k: canon(k) for k in subst}
        # DEAD-VALUE CASCADE: pure values with no remaining uses (after drops) lose their defs
        while True:
            defs2, def_ops2, _, addr2, uses2, pure2 = analyze(
                [ln for i, ln in enumerate(lines) if i not in drop])
            # map back: recompute uses on the ORIGINAL indices instead (simpler: count uses
            # excluding dropped lines and dropped-def self-lines)
            live_use = {}
            for i, ln in enumerate(lines):
                if i in drop: continue
                m = DEF.match(ln)
                dv = int(m.group(2)) if m else None
                rhs = ln.split('=',1)[1] if '=' in ln else ln
                for v in (int(x) for x in VAL.findall(rhs)):
                    if v != dv: live_use[canon(v)] = live_use.get(canon(v), 0) + 1
            grew = False
            for v in pure1:
                if v in subst: continue
                if live_use.get(v, 0) == 0 and defs.get(v) is not None and defs[v] not in drop:
                    drop.add(defs[v]); stats['r_dead'] += 1; grew = True
            if not grew: break
        # M3 (R3): cmp/brz FUSION -- vC cmp-class pure single-def, sole use = the immediately
        # following `brz vC` => one `brc OP vA vB` line. Deletes the setcc round-trip without
        # creating any long-lived value (immune to the R2 pooling failure mode).
        fused = False
        # fuse only on QUIESCENT rounds: fusing mid-round discarded pending subst/drop and
        # re-counted the same copies every iteration (the 176 -> 1,600 counter artifact)
        i = 0 if not (os.environ.get('OPT_NO_FUSE') or subst or drop) else len(lines)
        while i < len(lines) - 1:
            m = DEF.match(lines[i])
            if m:
                dv = int(m.group(2)); op = m.group(4)
                if (dv in pure1 and op in CMPOP and len(uses.get(dv, [])) == 1
                        and uses[dv][0] == i + 1
                        and re.match(r'\s*brz\s+v%d\s*$' % dv, lines[i+1])):
                    ops = [int(x) for x in VAL.findall(lines[i].split('=',1)[1])]
                    if len(ops) == 2:
                        indent = re.match(r'\s*', lines[i+1]).group(0)
                        lines[i:i+2] = ['%sbrc %s v%d v%d' % (indent, op, ops[0], ops[1])]
                        stats['r_fuse'] += 1; fused = True
                        continue
            i += 1
        if fused: changed_any = True; continue
        if not subst and not drop: break
        # EMIT the rewrite: drop lines, substitute operands (dropped defs are gone, so any
        # remaining occurrence of a key is an operand -- whole-line substitution is safe)
        out = []
        for i, ln in enumerate(lines):
            if i in drop: continue
            if subst:
                ln = VAL.sub(lambda mm: 'v%d' % subst.get(int(mm.group(1)), int(mm.group(1))), ln)
            out.append(ln)
        stats['r_dedup'] += sum(1 for k in subst)
        lines = out; changed_any = True
    return lines

# ---------------------------------------------------------------------------
# O2 pass-pipeline harness (rzir-opt-o2 arc, rung O2a).
#
# RZIR_OPT_LEVEL absent or "1"  -> EXACTLY the historical behavior above.
# RZIR_OPT_LEVEL=2              -> the registered O2 passes run over the
#   function-parsed unit FIRST, then the existing fold/dedup/copy-prop/dead/
#   fuse passes re-run to their fixpoint (inlining exposes new folds).
# The pipeline is EMPTY this rung, so level 2 == level 1 byte-for-byte: the
# do-no-harm gate (gates/opt-do-no-harm.sh in the o2 arc) holds all three
# modes identical to git-HEAD until a pass actually lands.
#
# Contract (specs/pipeline-spec.md in the o2 arc): a 'function' pass receives
# (lines, tables, fresh, stats) where tables = analyze(lines) extended with
# per-line classification, and returns the rewritten line list; a 'unit' pass
# (INLINE only) receives (pre, funcs, fresh, stats) and returns the new funcs
# list.  All fresh vN come from the shared Fresh allocator -- never invented
# ad hoc (inlining renames; T33/T35: tables keyed by vN die at rename points).
# ---------------------------------------------------------------------------
EFFECT = {'store','syscall','call','calli','put','putc','puti','puts','ret'}
CONTROL = {'brz','brc','jmp','goto','loop','endloop','brk','cnt','label','fn','ir'}

def classify(ln):
    """('def', vN, op) | ('effect', head) | ('control', head) | ('other', head).
    A def whose op is an effect (call/calli/syscall) classifies as 'effect' --
    it is a barrier even though it defines a value."""
    m = DEF.match(ln)
    if m:
        op = m.group(4)
        return ('effect', op) if op in EFFECT else ('def', int(m.group(2)), op)
    head = (ln.split() or [''])[0]
    if head in EFFECT: return ('effect', head)
    if head in CONTROL: return ('control', head)
    return ('other', head)

class Fresh:
    """Monotonic fresh-vN allocator seeded past the unit-wide max vN.
    Unit-wide (not per-function) because INLINE splices callee bodies across
    function boundaries; a per-function counter could collide after a splice."""
    def __init__(self, funcs):
        mx = 0
        for f in funcs:
            for ln in f:
                for v in VAL.findall(ln): mx = max(mx, int(v))
        self.n = mx + 1
    def take(self):
        v = self.n; self.n += 1; return v

def tables(lines):
    """analysis tables for one function: the analyze() sextuple plus the
    per-line classification list. Recompute after EVERY rewriting pass."""
    defs, def_ops, reused, addr_taken, uses, pure1 = analyze(lines)
    return {'defs': defs, 'def_ops': def_ops, 'reused': reused,
            'addr_taken': addr_taken, 'uses': uses, 'pure1': pure1,
            'kinds': [classify(ln) for ln in lines]}

def inline_pass(pre, funcs, fresh, stats):
    """O2b INLINE (specs/inline-spec.md). Copy-based param binding; qualifying
    callees: leaf + non-recursive + single-tail-return + (<= CEIL body lines OR
    exactly one program-wide call site); growth budget. Deterministic: callees
    sorted, sites in file/line order, fresh vN from the shared allocator."""
    CEIL = int(os.environ.get('RZIR_INLINE_CEIL', '32'))
    total = len(pre) + sum(len(f) for f in funcs)
    budget = int(os.environ.get('RZIR_INLINE_BUDGET', str(total // 2)))
    sites = {}          # callee name -> program-wide call-site count
    for f in funcs:
        for ln in f:
            m = DEF.match(ln)
            if m and m.group(4) == 'call':
                nm = ln.split('=', 1)[1].split()[1]
                sites[nm] = sites.get(nm, 0) + 1
    info = {}           # qualifying-callee body info
    for f in funcs:
        hd = f[0].split()
        if hd[0] != 'fn' or not f or f[-1].strip() != 'end': continue
        name = hd[1]
        body = f[1:-1]
        p = 0
        while p < len(body):
            m = DEF.match(body[p])
            if m and m.group(4) == 'param': p += 1
            else: break
        params = [int(DEF.match(body[k]).group(2)) for k in range(p)]
        ok = True; depth = 0
        for ln in body[p:]:
            m = DEF.match(ln)
            op = m.group(4) if m else (ln.split() or [''])[0]
            if op in ('call', 'calli', 'goto', 'label', 'param'): ok = False; break
            if op == 'loop': depth += 1
            elif op == 'endloop': depth -= 1
            elif op in ('brk', 'cnt') and depth == 0: ok = False; break
        if not ok or depth != 0: continue
        t = len(body)
        while t > p and (body[t-1].split() or [''])[0] == 'ret': t -= 1
        if t == len(body): continue                 # no trailing ret run
        mret = re.match(r'\s*ret\s+v(\d+)\s*$', body[t])
        if not mret: continue
        if any((ln.split() or [''])[0] == 'ret' for ln in body[p:t]): continue
        # anonymous-slot guard: a vN referenced in the callee but never
        # DEFINED there is a local-array base (`ref vB`, interiors invisible
        # in rzir) -- splicing would re-home the base to a fresh scalar slot
        # while the interior range stays behind: disqualify the callee.
        dvs = set(params)
        for ln in body:
            m2 = DEF.match(ln)
            if m2: dvs.add(int(m2.group(2)))
        if any(int(x) not in dvs for ln in body for x in VAL.findall(ln)): continue
        info[name] = {'params': params, 'core': body[p:t],
                      'retv': int(mret.group(1)), 'size': t - p}
    # MEASURED OFF (O2b): the spec's single-site-over-ceiling rule (inline a
    # >CEIL callee when it has exactly one program-wide site) regressed
    # bench-full 0.88 s -> 2.57 s: splicing sieve's loop nest into main let
    # the splice's long-interval locals take the whole 8-reg pool across the
    # outer loop and pushed the hot inner-loop temps into slots.  Linear scan
    # has no spill priority, so big loop-heavy splices are net losses.
    # Re-enable via RZIR_INLINE_SINGLE=1 to re-measure if rzir-ra gains
    # spill-aware priority (the R2/OPT_POOL precedent).
    single_ok = bool(os.environ.get('RZIR_INLINE_SINGLE'))
    cands = sorted((sites[n] * info[n]['size'], n) for n in sorted(info)
                   if sites.get(n, 0) > 0
                   and (info[n]['size'] <= CEIL
                        or (single_ok and sites[n] == 1)))
    chosen = set(); cum = 0
    for w, name in cands:
        d = info[name]
        add = sites[name] * (2 + len(d['params']) + d['size'])
        # single-site callees duplicate nothing (the body exists once either
        # way) -- pure call-overhead deletion, exempt from the growth budget
        if sites[name] == 1:
            cum += add; chosen.add(name); continue
        if cum + add > budget: break
        cum += add; chosen.add(name)
    SLOTCAP = int(os.environ.get('RZIR_INLINE_SLOTCAP', '8190'))
    out_funcs = []
    for f in funcs:
        caller = f[0].split()[1] if f[0].startswith('fn ') else 'main'
        # per-caller slot budget (T19: cf-darwin emitted frame = 8192 slots,
        # usable ~8190).  Fresh splice names are later compacted to
        # orig_max+1.. by renumber(); guard the compacted ceiling here.
        orig_max = 0
        for ln in f:
            for x in VAL.findall(ln): orig_max = max(orig_max, int(x))
        fresh_used = 0
        out = [f[0]]
        for ln in f[1:]:
            m = DEF.match(ln)
            if m and m.group(4) == 'call':
                toks = ln.split('=', 1)[1].split()   # ['call', F, args...]
                F = toks[1]; args = toks[2:]
                d = info.get(F)
                if (F in chosen and F != caller and d
                        and len(args) == len(d['params'])
                        and all(re.fullmatch(r'v\d+', a) for a in args)):
                    vs = set(d['params']) | {d['retv']}
                    for cl in d['core']:
                        vs.update(int(x) for x in VAL.findall(cl))
                    if orig_max + fresh_used + 1 + len(vs) > SLOTCAP:
                        stats['r_inl_skipcap'] += 1
                        out.append(ln); continue
                    fresh_used += 1 + len(vs)
                    vR = int(m.group(2))
                    z = fresh.take()
                    ren = {v: fresh.take() for v in sorted(vs)}
                    sp = ['  v%d: i32 = const 0' % z]
                    for k, pv in enumerate(d['params']):
                        sp.append('  v%d: i32 = add %s v%d' % (ren[pv], args[k], z))
                    for cl in d['core']:
                        sp.append('  ' + VAL.sub(
                            lambda mm: 'v%d' % ren[int(mm.group(1))], cl).strip())
                    sp.append('  v%d: i32 = add v%d v%d' % (vR, ren[d['retv']], z))
                    out.extend(sp)
                    stats['r_inl_sites'] += 1
                    stats['r_inl_lines'] += len(sp) - 1
                    continue
            out.append(ln)
        out_funcs.append(out)
    stats['r_inl_callees'] += len(chosen)
    stats['r_inl_budget'] += cum
    return out_funcs

LCSE_LEADERS = {'brz','brc','pool','loop','endloop','jmp','cont','brk',
                'goto','label','ret'}   # the line AFTER any of these starts a
                                        # new block; call/syscall/store do NOT
                                        # end blocks (pure register-class
                                        # values are unreachable via memory)

def lcse_pass(lines, T, fresh, stats):
    """O2c LCSE (specs/licm-lcse-spec.md section 1): local value numbering per
    basic block. Candidates: const + pure arith/cmp defs, dv not reused/
    addr-taken; loads/refs excluded (memory CSE deferred). Consts key by
    VALUE so downstream arith keys converge. Kill rules: any redefinition
    gets a fresh value number; block leaders reset both tables."""
    reused, addr_taken = T['reused'], T['addr_taken']
    ELIG = ARITH | CMPOP | {'const'} - {'ref'}
    vn_of = {}; expr_vn = {}; nvn = [0]
    def fresh_vn():
        nvn[0] += 1; return nvn[0]
    def vn(v):
        if v not in vn_of: vn_of[v] = fresh_vn()
        return vn_of[v]
    subst = {}; drop = set()
    for i, ln in enumerate(lines):
        m = DEF.match(ln)
        if m:
            dv = int(m.group(2)); op = m.group(4)
            key = None
            if (op in ELIG and dv not in reused and dv not in addr_taken):
                rhs = ln.split('=', 1)[1].split()
                if op == 'const' and len(rhs) == 2:
                    key = ('const', rhs[1])
                elif op != 'const' and not any(
                        v in reused or v in addr_taken
                        for v in VAL.findall(ln.split('=', 1)[1]) for v in [int(v)]):
                    # mutables are the fold barrier: ref/store mutation has no
                    # def line, so vn() would stay stale across it (rzrs shape)
                    ops = [int(x) for x in VAL.findall(ln.split('=', 1)[1])]
                    if len(ops) == 2:
                        key = (op, vn(ops[0]), vn(ops[1]))
            if key is not None and key in expr_vn:
                sv = expr_vn[key]
                subst[dv] = sv; drop.add(i); vn_of[dv] = vn_of[sv]
                stats['r_lcse'] += 1
            else:
                vn_of[dv] = fresh_vn()          # kill rule 1: redef = fresh number
                if key is not None: expr_vn[key] = dv
            head = op
        else:
            head = (ln.split() or [''])[0]
        if head in LCSE_LEADERS:                # kill rule 2: block boundary
            vn_of = {}; expr_vn = {}
    if not subst: return lines
    def canon(v):
        while v in subst: v = subst[v]
        return v
    return [VAL.sub(lambda mm: 'v%d' % canon(int(mm.group(1))), ln)
            for i, ln in enumerate(lines) if i not in drop]

def licm_pass(lines, T, fresh, stats):
    """O2d LICM (specs/licm-lcse-spec.md section 2). Hoists pure trap-free
    defs (const/arith/cmp; div/mod/ref/memory excluded) whose operands are
    loop-invariant, single-def, not addr-taken, all uses inside the region
    (rule 5, belt-and-suspenders), to immediately before the `loop` line.
    Nested loops via iterate-to-fixpoint (innermost-first per round; a def
    hoisted to an inner preheader re-qualifies against the outer region next
    round).
    MEASURED (O2d): the full rule set regressed bench-full 0.92 -> 2.47 s --
    hoisted small consts hold loop-long intervals, grab the 8-reg pool and
    push hot inner-loop temps into slots (the R2 pooling failure mode at loop
    scope). DEFAULT is therefore the spec section-2.5 restriction: hoist only
    multi-limb (>=2^32) consts -- the 3-4 instruction GBASE materializations,
    unfoldable and un-imm-able by construction -- plus non-const arith/cmp.
    Restricted: bench-full 0.91 s / 712 B. RZIR_LICM_FULL=1 re-enables the
    full rule to re-measure if rzir-ra gains spill-aware priority."""
    ELIG = ARITH | CMPOP | {'const'}
    restrict = not os.environ.get('RZIR_LICM_FULL')
    while True:
        defsall = {}; uses = {}; addr_taken = set()
        for i, ln in enumerate(lines):
            m = DEF.match(ln)
            dv = int(m.group(2)) if m else None
            if m: defsall.setdefault(dv, []).append(i)
            rhs = ln.split('=', 1)[1] if '=' in ln else ln
            head = m.group(4) if m else (ln.split() or [''])[0]
            if head == 'ref':
                for v in (int(x) for x in VAL.findall(rhs)): addr_taken.add(v)
            for v in (int(x) for x in VAL.findall(rhs)):
                if v != dv: uses.setdefault(v, []).append(i)
        reused = {v for v, ds in defsall.items() if len(ds) > 1}
        stack = []; regions = []
        for i, ln in enumerate(lines):
            head = (ln.split() or [''])[0]
            if head == 'loop': stack.append(i)
            elif head == 'endloop' and stack: regions.append((stack.pop(), i))
        plan = []; taken = set()
        for (s, e) in regions:                    # pop order = innermost-first
            hoisted = set(); changed = True
            while changed:
                changed = False
                for i in range(s + 1, e):
                    if i in hoisted or i in taken: continue
                    m = DEF.match(lines[i])
                    if not m: continue
                    dv = int(m.group(2)); op = m.group(4)
                    if op not in ELIG or dv in reused or dv in addr_taken: continue
                    rhs = lines[i].split('=', 1)[1]
                    if op == 'const':
                        try: k = int(rhs.split()[1])
                        except (ValueError, IndexError): continue
                        if restrict and not (k >> 32): continue
                        opnds = []
                    else:
                        opnds = [int(x) for x in VAL.findall(rhs)]
                        if len(opnds) != 2: continue
                    # mutables are the fold barrier (spec section 1) -- an operand
                    # mutated through ref/store has no def line inside the region,
                    # so the defs-hoisted test alone reads it as invariant (rzrs's
                    # `loop; vN=lt mut,mut; brz` hoisted -> infinite loop).
                    if any(v in reused or v in addr_taken for v in opnds): continue
                    if not all(all(j in hoisted for j in defsall.get(v, [])
                                   if s < j < e) for v in opnds): continue
                    if not all(s < u < e for u in uses.get(dv, [])): continue
                    hoisted.add(i); changed = True
            if hoisted:
                plan.append((s, sorted(hoisted))); taken |= hoisted
        if not plan: return lines
        pre_ins = {}
        for s, idxs in plan:
            pre_ins.setdefault(s, []).extend(idxs)
        out = []
        for i, ln in enumerate(lines):
            if i in pre_ins:
                for j in pre_ins[i]:
                    out.append(lines[j])
                    stats['r_licm'] += 1
                    mj = DEF.match(lines[j])
                    if mj and mj.group(4) == 'const':
                        try:
                            if int(lines[j].split('=', 1)[1].split()[1]) >> 32:
                                stats['r_licm_ml'] += 1
                        except (ValueError, IndexError): pass
            if i in taken: continue
            out.append(ln)
        lines = out

O2_PASSES = []   # [(name, kind, fn)]; kind in ('unit', 'function')
def register_o2(name, kind, fn): O2_PASSES.append((name, kind, fn))
# O2b/c/d registration point -- pipeline order per specs/pipeline-spec.md:
register_o2('inline', 'unit', inline_pass)          # O2b
register_o2('lcse', 'function', lcse_pass)          # O2c
register_o2('licm', 'function', licm_pass)          # O2d
register_o2('lcse2', 'function', lcse_pass)         # spec section 3: multi-level
                                                    # hoists land same-K consts in
                                                    # one preheader; second LCSE
                                                    # round merges them

def run_o2(pre, funcs, stats):
    fresh = Fresh(funcs)
    stats['o2_base0'] = fresh.n   # renumber boundary: originals below stay fixed
    for name, kind, fn in O2_PASSES:
        if kind == 'unit':
            funcs = fn(pre, funcs, fresh, stats)
        else:
            funcs = [fn(f, tables(f), fresh, stats) for f in funcs]
    return funcs

def main():
    text = sys.stdin.read(); lines = text.splitlines()
    funcs = []; pre = []; cur = None
    for ln in lines:
        if re.match(r'(ir main|fn )', ln):
            cur = [ln]; funcs.append(cur)
        elif cur is not None: cur.append(ln)
        else: pre.append(ln)
    stats = {'values':0,'lines':0,'const_defs':0,'foldable':0,'zero_copies':0,
             'dead_after':0,'cmp_brz':0,'dup_consts':0,
             'r_fold':0,'r_copy':0,'r_dead':0,'r_dedup':0,'r_fuse':0,
             'r_inl_callees':0,'r_inl_sites':0,'r_inl_lines':0,'r_inl_budget':0,'r_inl_skipcap':0,
             'r_lcse':0,'r_licm':0,'r_licm_ml':0}
    for f in funcs: census_function(f, stats)
    nin = stats['lines']
    lvl2 = os.environ.get('RZIR_OPT_LEVEL') == '2'
    if lvl2:
        funcs = run_o2(pre, funcs, stats)
    out = list(pre)
    def renumber(fl, base0):
        # O2 vN compaction: inlining pushes unit-wide fresh vN past cf-darwin's
        # emitted-frame slot cap (T19: 8192 slots; probed usable ~8190 -- lr
        # rides the frame top).  ONLY the inline-fresh names (>= base0, the
        # Fresh seed) may be renamed: original vN numbers are LOAD-BEARING --
        # a local array occupies an anonymous slot range [base, base+n) where
        # only the base appears textually (`ref vB` with no def line), so any
        # rename that moves originals or lands names inside such a range
        # corrupts memory (found via parse_call: v3 = ref'd, never defined).
        # Fresh names are this pass's own scalar allocations; compact them to
        # fn-original-max+1.. (always above every array interior: ccc's body
        # temps outnumber trailing decls, so fn max vN > any interior slot).
        # Deterministic (textual order), so the O2 self-host fixpoint holds.
        orig_max = 0
        for ln in fl:
            for x in VAL.findall(ln):
                v = int(x)
                if v < base0: orig_max = max(orig_max, v)
        rn = {}
        def sub(mm):
            v = int(mm.group(1))
            if v < base0: return 'v%d' % v
            if v not in rn: rn[v] = orig_max + 1 + len(rn)
            return 'v%d' % rn[v]
        return [VAL.sub(sub, ln) for ln in fl]
    if lvl2 and os.environ.get('RZIR_OPT_O2_ONLY'):
        for f in funcs: out.extend(f)       # KAT view: post-O2, pre-fixpoint
    elif lvl2:
        base0 = stats.get('o2_base0', 0)
        for f in funcs: out.extend(renumber(opt_function(f, stats), base0))
    else:
        for f in funcs: out.extend(opt_function(f, stats))
    if lvl2:
        sys.stderr.write("rzir-opt O2: inline callees %(r_inl_callees)d, sites "
            "%(r_inl_sites)d (cap-skipped %(r_inl_skipcap)d), lines +%(r_inl_lines)d, budget %(r_inl_budget)d | "
            "lcse %(r_lcse)d | licm %(r_licm)d (multi-limb %(r_licm_ml)d)\n" % stats)
    sys.stderr.write("rzir-opt census: %(lines)d lines, %(values)d values | "
        "const defs %(const_defs)d | foldable arith/cmp %(foldable)d | "
        "zero-idiom copies %(zero_copies)d | dead after fold+prop %(dead_after)d | "
        "fusable cmp/brz %(cmp_brz)d | dup consts %(dup_consts)d\n" % stats)
    sys.stderr.write("rzir-opt M1: folded %(r_fold)d, deduped %(r_dedup)d, copies %(r_copy)d, "
        "dead %(r_dead)d, fused %(r_fuse)d" % stats + " | lines %d -> %d\n" % (nin, sum(1 for _ in out) - len(pre)))
    sys.stdout.write("\n".join(out) + "\n")

main()
