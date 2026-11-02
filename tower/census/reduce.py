#!/usr/bin/env python3
# Reliable reducer for SEM0 lowering failures.
# interesting(t) = rustc(-Zparse-only) accepts t as valid Rust  AND  SEM0 aborts on t.
# Two-oracle gate: the rustc check structurally forbids landing on a syntactically-broken
# reduction (the unsoundness that defeated the earlier ddmin/macro-safe reducers).
import re,subprocess,sys,os,time
J=os.environ["J"]  # the driver scratch dir
FE=os.environ.get("FE",J+"/pin/sem0.frozen")
NT=os.path.expanduser("~/.rustup/toolchains/nightly-2024-10-01-x86_64-unknown-linux-gnu/bin/rustc")
SEM0_T=int(os.environ.get("SEM0_T","30"))
# GUARD: a missing/broken FE makes every SEM0 run exit 127 (!=0), which the abort oracle reads as "aborts" ->
# the reducer over-reduces to a false minimal that PASSES on a good frozen. This is easy to hit because
# `J=$d FE=$J/pin/..` (shell applies env assignments left-to-right) resolves FE against the NEW J -> $d/pin/.. .
# Fail loudly instead of silently producing garbage. Pass FE as an ABSOLUTE path, not `$J/pin/..`.
if not (os.path.isfile(FE) and os.access(FE, os.X_OK)):
    sys.stderr.write("reduce.py: FE not an executable file: %r (set FE to an absolute path)\n" % FE); sys.exit(3)
if not os.path.isfile(NT):
    sys.stderr.write("reduce.py: rustc oracle missing: %r\n" % NT); sys.exit(3)
NS=[0]; NR=[0]
# macro-match errors from `-Zunpretty=expanded` -- these mean a reduction BROKE macro matching (deleted the
# matching arm/args), i.e. an ARTIFACT. Import/type/missing-proc-macro errors are NOT these and are tolerated
# (a reduced snippet need not resolve external deps to be a faithful macro-match test).
_MACROERR = ("no rules expected", "unexpected end of macro invocation", "while trying to match",
             "expected end of macro", "macro expansion ignores", "recursion limit reached while expanding")
def rustc_ok(t):
    # SOUND oracle: parse-valid (-Zparse-only) AND the reduction must not introduce a MACRO-MATCH failure under
    # -Zunpretty=expanded. -Zparse-only alone accepted non-matching macro invocations -> the reducer landed on
    # artifacts exposing a no-match bug, not the crate's real one (zerotrie/apfloat). Full expansion is too strict
    # (external proc-macros like displaydoc don't resolve in a snippet), so we reject ONLY on macro-match errors
    # and tolerate import/type errors. Env MACROUNSOUND=1 restores parse-only for non-macro parse-gap reductions.
    import os as _os
    open(f"{J}/r_chk.rs","w").write(t); NR[0]+=1
    if subprocess.run([NT,"-Zparse-only","--edition","2021","--crate-type","lib",f"{J}/r_chk.rs"],
                      stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode != 0:
        return False
    if _os.environ.get("MACROUNSOUND"): return True
    r = subprocess.run([NT,"-Zunpretty=expanded","--edition","2021","--crate-type","lib","--crate-name","rchk",f"{J}/r_chk.rs"],
                       stdout=subprocess.DEVNULL,stderr=subprocess.PIPE)
    if r.returncode == 0: return True
    err = r.stderr.decode("utf-8","replace")
    for m in _MACROERR:
        if m in err: return False   # the reduction broke macro matching -> artifact, reject
    return True   # expansion failed only on imports/types -> still a faithful macro-match test
def sem0_aborts(t):
    open(f"{J}/r_s9.rs","w").write("//@S9\n"+t+"\n")
    def run(lim): return subprocess.run(f"cat {J}/r_s9.rs | timeout {lim} {FE} > {J}/r_s9.rz 2>/dev/null",shell=True).returncode
    rc=run(SEM0_T)
    if rc==124: rc=run(SEM0_T*3)        # timeout is inconclusive (CPU contention) -> retry longer
    if rc==124: return False            # still timing out -> conservative: NOT a confirmed abort
    return rc!=0                         # rc=1 parse-abort / rc=139 crash = genuine; rc=0 = lowered
def interesting(t):
    if not t.strip(): return False
    if not rustc_ok(t): return False
    NS[0]+=1; return sem0_aborts(t)
def strip(t): return re.sub(r'"(?:\\.|[^"\\])*"','',re.sub(r"'(?:\\.|[^'\\])'",'',t))
def units(body):
    # balanced top-level segments of a brace body (never splits inside (){}[])
    ls=body.split("\n"); out=[]; d=0; s=0
    for i,l in enumerate(ls):
        for ch in strip(l):
            if ch in '([{':d+=1
            elif ch in ')]}':d-=1
        if d<=0:
            seg="\n".join(ls[s:i+1])
            if seg.strip(): out.append(seg)
            s=i+1; d=0
    if s<len(ls) and "\n".join(ls[s:]).strip(): out.append("\n".join(ls[s:]))
    return out
def ddmin_units(us,wrap):
    keep=list(us); changed=True
    while changed and len(keep)>1:
        changed=False
        # coarse: try removing contiguous halves/quarters first
        for frac in (2,4,8):
            step=max(1,len(keep)//frac); i=0; adv=False
            while i<len(keep):
                trial=keep[:i]+keep[i+step:]
                if trial and interesting(wrap("\n".join(trial))):
                    keep=trial; changed=True; adv=True; break
                i+=step
            if adv: break
        if changed: continue
        # fine: remove single units
        for idx in range(len(keep)):
            trial=keep[:idx]+keep[idx+1:]
            if trial and interesting(wrap("\n".join(trial))):
                keep=trial; changed=True; break
    return keep
def ident(x): return x
def reduce(src):
    # SOUND reducer: every candidate is a FULL standalone text, validated directly by interesting().
    # No wrapper/descent reconstruction (that was unsound -- it reported false minimals). Minimals keep any
    # enclosing `mod X { .. }` wrappers (removing a `mod X {` line unbalances -> rustc rejects -> kept), which is
    # fine: blank/bisect the result by hand. Repeats item+line passes until a fixpoint.
    cur=src
    while True:
        before=cur
        us=units(cur)
        if len(us)>1:
            us=ddmin_units(us,ident); cur="\n".join(us)
        lines=cur.split("\n"); n=2
        while len(lines)>=2:
            step=max(1,len(lines)//n); i=0; red=False
            while i<len(lines):
                comp=lines[:i]+lines[i+step:]
                if comp and interesting("\n".join(comp)): lines=comp; n=max(n-1,2); red=True; break
                i+=step
            if red: continue
            if n>=len(lines): break
            n=min(len(lines),2*n)
        cur="\n".join(lines)
        if cur==before: break
    return cur
if __name__=="__main__":
    src=open(sys.argv[1],errors='ignore').read(); t0=time.time()
    if not interesting(src):
        print("NOT-INTERESTING at start (rustc_ok=%s sem0_aborts=%s)"%(rustc_ok(src),sem0_aborts(src))); sys.exit(2)
    red=reduce(src)
    ok=interesting(red)
    print("===== MINIMAL: %d lines | SEM0 calls=%d rustc calls=%d | %.0fs | valid&fails=%s ====="%(
        len(red.split(chr(10))),NS[0],NR[0],time.time()-t0,ok))
    print(red)
