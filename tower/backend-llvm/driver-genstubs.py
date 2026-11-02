import re,sys,collections
src=sys.argv[1]; fo=sys.argv[2]; so=sys.argv[3]
defs=set(); argc={}; calls=collections.Counter()
cur=None; pc=0
for ln in open(src):
    s=ln.rstrip('\n'); t=s.split()
    if t and t[0] in ("fn","ir") and len(t)>=2:
        if cur is not None: argc[cur]=pc
        nm=[]
        for x in t[1:]:
            if x=="->": break
            nm.append(x)
        cur=" ".join(nm); defs.add(cur); pc=0
    elif cur is not None and re.search(r':\s*i32\s*=\s*param', s):
        pc+=1
    if len(t)>=5 and t[3]=="call":
        cc=[]
        for x in t[4:]:
            if re.match(r'^v\d+$',x): break
            cc.append(x)
        c=" ".join(cc)
        if c and not c.startswith('__'): calls[c]+=1
if cur is not None: argc[cur]=pc
bybare=collections.defaultdict(list)
for d in defs:
    if '$' in d: bybare[d.split('$',1)[-1] if False else d.split('$')[-1]].append(d)
# prefer entry crate rustc_driver_impl, then rustc_interface, then rustc_session, then unique
HARD={"run_compiler":"rustc_driver_impl$run_compiler"}
def pick(bare):
    if bare in HARD: return HARD[bare]
    cand=bybare.get(bare,[])
    if len(cand)==1: return cand[0]   # unique qualified def only; ambiguous -> identity no-op (avoids wrong-crate self-recursion e.g. init_logger)
    return None
dang=[c for c in calls if c not in defs]
out=[]; aliased=0; ident=0
for c in dang:
    q=pick(c)
    if q:
        n=min(max(argc.get(q,2),1),6)
        out.append("fn %s"%c)
        for i in range(n): out.append("  v%d: i32 = param"%i)
        out.append("  v%d: i32 = call %s %s"%(n,q," ".join("v%d"%i for i in range(n))))
        out.append("  ret v%d"%n); aliased+=1
    else:
        out.append("fn %s"%c)
        for i in range(6): out.append("  v%d: i32 = param"%i)
        out.append("  ret v0"); ident+=1
open(so,"w").write("\n".join(out)+"\n")
open(fo,"w").write("".join("%d\t%s\n"%(calls[c],c) for c in sorted(dang,key=lambda x:-calls[x])))
print("danglers=%d aliased=%d identity=%d"%(len(dang),aliased,ident))
