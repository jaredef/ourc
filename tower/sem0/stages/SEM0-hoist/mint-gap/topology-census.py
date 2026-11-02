#!/usr/bin/env python3
"""Re-derive the mint-gap failure topology from the r2 corpus.

Consumes a join of the corpus board's two sweeps (L6 and SEM0) and assigns every gap probe to a pipeline.
Emits the table in `mint-gap-pipeline-oracle.md`. Re-run it after any rung that could move the gap; a pipeline
whose count does not fall after its fix was landed is the falsifier for that row.

  python3 topology-census.py <join.txt>      # cols: <probe> <L6-status> <SEM0-status>

PRECISION is printed per pipeline and is the honest part of this instrument: it is the share of matching probes
that actually FAIL. A pipeline near 100% is a real family; one near 50% is a coincidence of syntax and must not
be ranked as a cause. That is why P5 is reported as residue and not as a pipeline.
"""
import os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
R2   = os.path.join(HERE, '../../../../2026-07-15-rust-frontend-completion/r2-corpus/probes')
S1   = os.path.join(HERE, '../../S1-lex-parse-ast/s1_parser.rs')

RX_AMP = re.compile(r'for\s+[\w()\s,]+\s+in\s+&|(?:if\s+let|match|while\s+let)[^={\n]*=\s*&|match\s+&')
RX_OP  = re.compile(r'impl\s+(std::ops::)?(Add|Sub|Mul|Div|Rem|Neg|Not|Index|IndexMut|Shl|Shr|BitAnd|BitOr)\b'
                    r'|#\[derive[^\]]*\b(PartialEq|Eq|PartialOrd|Ord)\b')
RX_BOX = re.compile(r'Box<|Box::new')
RX_CALL= re.compile(r'\.\s*([a-z_][a-z0-9_]*)\s*\(')
RX_FN  = re.compile(r'\bfn\s+([a-z_][a-z0-9_]*)')

NAMES = {'P1': 'P1  unknown std-method mouth', 'P2': 'P2  operator / derived-cmp mouth',
         'P3': 'P3  Box payload access',       'P4': 'P4  &-borrow transparency',
         'P5': 'P5  residue (NOT a pipeline)'}

def known_names():
    # The names s1_parser can dispatch on. A string literal in the emitter is the only machine-checkable
    # proxy we have; it OVER-counts (any literal), so P1 is a LOWER bound on the unknown-method population.
    with open(S1, encoding='utf8', errors='replace') as fh:
        return set(m.strip('"') for m in re.findall(r'"[a-z_][a-z0-9_]*"', fh.read()))

def src(n):
    for e in ('.rs', ''):
        p = os.path.join(R2, n + e)
        if os.path.exists(p):
            return open(p, encoding='utf8', errors='replace').read()
    return None

def classify(s, known):
    # ORDER IS THE CLAIM: P1 first because it is the only 100%-precision predicate, so anything it matches is
    # attributed there rather than to a weaker downstream predicate.
    selfdef = set(RX_FN.findall(s))
    if any(m not in known and m not in selfdef for m in set(RX_CALL.findall(s))): return 'P1'
    if RX_AMP.search(s): return 'P4'
    if RX_OP.search(s):  return 'P2'
    if RX_BOX.search(s): return 'P3'
    return 'P5'

def main(join):
    known = known_names()
    gap, ok = [], []
    for line in open(join):
        f = line.split()
        if len(f) < 3: continue
        (gap if (f[1] == 'OK' and f[2] != 'OK') else ok if f[2] == 'OK' else []).append(f[0])
    g, o = collections.Counter(), collections.Counter()
    for names, ctr in ((gap, g), (ok, o)):
        for n in names:
            s = src(n)
            if s is not None: ctr[classify(s, known)] += 1
    print(f"=== mint-gap failure topology: {len(gap)} gap / {len(ok)} passing ===")
    print("%-34s %6s %6s %10s" % ("pipeline", "gap", "ok", "precision"))
    for k in ('P1', 'P2', 'P3', 'P4', 'P5'):
        tot = g[k] + o[k]
        print("  %-32s %5d %5d %9s" % (NAMES[k], g[k], o[k], f"{100*g[k]/tot:.0f}%" if tot else "-"))
    print(f"\n  fail-open family (P1+P2): {g['P1']+g['P2']} of {len(gap)} "
          f"({100*(g['P1']+g['P2'])/len(gap):.0f}%)")
    print(f"  representation family (P3+P4): {g['P3']+g['P4']} of {len(gap)} "
          f"({100*(g['P3']+g['P4'])/len(gap):.0f}%)")

if __name__ == '__main__':
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
