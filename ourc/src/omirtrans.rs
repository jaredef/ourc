pub fn fold(b: &Body) -> i64 {
    let mut tv = oindex::tv_new();
    let mut i = 0;
    while i < b.ntmp {
        let _ = oindex::tv_push(&mut tv, 0);
        i = i + 1;
    }
    let mut lv: Vec<i64> = Vec::new();
    let mut j = 0;
    while j < b.nloc {
        lv.push(0);
        j = j + 1;
    }
    let mut heap: Vec<i64> = Vec::new();
    run_ops(&b.ops, &mut tv, &mut lv, &mut heap)
}
fn tget(tv: &oindex::TempVec, t: u32) -> i64 {
    oindex::tv_get(tv, &oindex::tid(t))
}
// two's-complement truncation, matching what codegen emits (trunc + zext/sext). Without this the folder
// reported a cast as identity and the driver printed a value the compiled program would not produce.
pub fn trunc_val(v: i64, bits: u32, signed: bool) -> i64 {
    if bits >= 64 {
        return v;
    }
    let mut p: i64 = 1;
    let mut i = 0;
    while i < bits {
        p = p * 2;
        i = i + 1;
    }
    let mut m = v - (v / p) * p;
    if m < 0 {
        m = m + p;
    }
    if signed {
        let h = p / 2;
        if m >= h {
            m = m - p;
        }
    }
    m
}
fn tset(tv: &mut oindex::TempVec, t: u32, v: i64) {
    oindex::tv_set(tv, &oindex::tid(t), v);
}
fn run_ops(ops: &Vec<Op>, tv: &mut oindex::TempVec, lv: &mut Vec<i64>, heap: &mut Vec<i64>) -> i64 {
    let mut k = 0;
    let mut last: i64 = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::StrC(t, _v) => {
                tset(tv, *t, 0);
            }
            Op::CastOp(t, sx, ty) => {
                let v = tget(tv, *sx);
                if ty.as_str() == "$i2f"
                    || (ty.len() >= 5 && &ty[0..5] == "$f2i$")
                    || ty.as_str() == "$idcast"
                {
                    tset(tv, *t, v);
                } else {
                    tset(
                        tv,
                        *t,
                        trunc_val(v, oast::width_bits(ty), oast::width_signed(ty)),
                    );
                }
            }
            Op::Brk(_b) => {}
            Op::BrkN(_b, _n) => {}
            Op::Cont(_c) => {}
            Op::ContN(_c, _n) => {}
            Op::RetEarly(t) => {
                return tget(tv, *t);
            }
            Op::ParamOp(t, _p) => {
                tset(tv, *t, 0);
                last = 0;
            }
            Op::CallOp(t, _n, _a) => {
                tset(tv, *t, 0);
                last = 0;
            }
            Op::Const(t, v) => {
                tset(tv, *t, *v);
                last = *v;
            }
            Op::Bin(t, o, x, y) => {
                let a = tget(tv, *x);
                let c = tget(tv, *y);
                let mn: i64 = 0 - 9223372036854775807 - 1;
                let v = if o.as_str() == "u>>" {
                    if c <= 0 {
                        a
                    } else {
                        (a >> c) & ((1 << (64 - c)) - 1)
                    }
                } else if o.as_str() == "u<" {
                    if (a ^ mn) < (c ^ mn) {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "u<=" {
                    if (a ^ mn) <= (c ^ mn) {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "u>" {
                    if (a ^ mn) > (c ^ mn) {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "u>=" {
                    if (a ^ mn) >= (c ^ mn) {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "+" {
                    a + c
                } else if o.as_str() == "-" {
                    a - c
                } else if o.as_str() == "*" {
                    a * c
                } else if o.as_str() == "u/" {
                    udiv64(a, c)
                } else if o.as_str() == "u%" {
                    a.wrapping_sub(udiv64(a, c).wrapping_mul(c))
                } else if o.as_str() == "/" {
                    if c != 0 {
                        a / c
                    } else {
                        0
                    }
                } else if o.as_str() == "%" {
                    if c != 0 {
                        a - (a / c) * c
                    } else {
                        0
                    }
                } else if o.as_str() == "&" {
                    a & c
                } else if o.as_str() == "|" {
                    a | c
                } else if o.as_str() == "^" {
                    a ^ c
                } else if o.as_str() == "<<" {
                    if c >= 0 && c < 64 {
                        a << c
                    } else {
                        0
                    }
                } else if o.as_str() == ">>" {
                    if c >= 0 && c < 64 {
                        a >> c
                    } else {
                        0
                    }
                } else if o.as_str() == "<" {
                    if a < c {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "<=" {
                    if a <= c {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == ">" {
                    if a > c {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == ">=" {
                    if a >= c {
                        1
                    } else {
                        0
                    }
                } else if o.as_str() == "!=" {
                    if a != c {
                        1
                    } else {
                        0
                    }
                } else {
                    if a == c {
                        1
                    } else {
                        0
                    }
                };
                tset(tv, *t, v);
                last = v;
            }
            Op::LoadLoc(t, s) => {
                let v = lv[*s as usize];
                tset(tv, *t, v);
                last = v;
            }
            Op::StoreLoc(s, t) => {
                let v = tget(tv, *t);
                lv[*s as usize] = v;
                last = v;
            }
            Op::IfOp(t, c0, tops, tr, eops, er) => {
                let cv = tget(tv, *c0);
                let v = if cv != 0 {
                    let _ = run_ops(tops, tv, lv, heap);
                    tget(tv, *tr)
                } else {
                    let _ = run_ops(eops, tv, lv, heap);
                    tget(tv, *er)
                };
                tset(tv, *t, v);
                last = v;
            }
            Op::LoopOp(cops, cr, bops) => {
                let mut guard = 0;
                loop {
                    let _ = run_ops(cops, tv, lv, heap);
                    if tget(tv, *cr) == 0 {
                        break;
                    }
                    let _ = run_ops(bops, tv, lv, heap);
                    guard = guard + 1;
                    if guard > 1000000 {
                        break;
                    }
                }
                last = 0;
            }
            Op::MakeStruct(t, _nf, av) => {
                let base = heap.len() as i64;
                let mut k = 0;
                while k < av.len() {
                    let v = tget(tv, av[k]);
                    heap.push(v);
                    k = k + 1;
                }
                tset(tv, *t, base);
                last = base;
            }
            Op::LoadField(t, bt, ix) => {
                let base = tget(tv, *bt);
                let a = base + (*ix as i64);
                let v = if a >= 0 && (a as usize) < heap.len() {
                    heap[a as usize]
                } else {
                    0
                };
                tset(tv, *t, v);
                last = v;
            }
            Op::StoreIdx(bt, it, vt) => {
                let base = tget(tv, *bt);
                let ix = tget(tv, *it) + 1;
                let v = tget(tv, *vt);
                let a = base + ix;
                if a >= 0 && (a as usize) < heap.len() {
                    heap[a as usize] = v;
                }
                last = v;
            }
            Op::StoreField(bt, fx, vt) => {
                let base = tget(tv, *bt);
                let v = tget(tv, *vt);
                let a = base + (*fx as i64);
                if a >= 0 && (a as usize) < heap.len() {
                    heap[a as usize] = v;
                }
                last = v;
            }
            Op::MakeArr(t, n, av) => {
                let base = heap.len() as i64;
                heap.push(*n as i64);
                let mut k = 0;
                while k < av.len() {
                    let v = tget(tv, av[k]);
                    heap.push(v);
                    k = k + 1;
                }
                tset(tv, *t, base);
                last = base;
            }
            Op::LoadIdx(t, bt, it) => {
                let base = tget(tv, *bt);
                let ix = tget(tv, *it) + 1;
                let a = base + ix;
                let v = if a >= 0 && (a as usize) < heap.len() {
                    heap[a as usize]
                } else {
                    0
                };
                tset(tv, *t, v);
                last = v;
            }
            Op::MakeEnum(t, tag, av) => {
                let base = heap.len() as i64;
                heap.push(*tag as i64);
                let mut k = 0;
                while k < av.len() {
                    let v = tget(tv, av[k]);
                    heap.push(v);
                    k = k + 1;
                }
                tset(tv, *t, base);
                last = base;
            }
            Op::MatchOp(dest, st, arms) => {
                let base = tget(tv, *st);
                let tg = if base >= 0 && (base as usize) < heap.len() {
                    heap[base as usize]
                } else {
                    -1
                };
                let mut k = 0;
                let mut done = false;
                while k < arms.len() {
                    let a = &arms[k];
                    if !done && (a.tag < 0 || (a.tag as i64) == tg) {
                        let mut q = 0;
                        while q < a.slots.len() {
                            let hp = base + 1 + (q as i64);
                            let pv = if hp >= 0 && (hp as usize) < heap.len() {
                                heap[hp as usize]
                            } else {
                                0
                            };
                            let sl = a.slots[q] as usize;
                            while lv.len() <= sl {
                                lv.push(0);
                            }
                            lv[sl] = pv;
                            q = q + 1;
                        }
                        let _ = run_ops(&a.ops, tv, lv, heap);
                        let v = tget(tv, a.res);
                        tset(tv, *dest, v);
                        last = v;
                        done = true;
                    }
                    k = k + 1;
                }
                if !done {
                    tset(tv, *dest, 0);
                    last = 0;
                }
            }
            Op::Ret(s) => {
                last = tget(tv, *s);
            }
        }
        k = k + 1;
    }
    last
}
pub fn nops(b: &Body) -> u32 {
    b.ops.len() as u32
}
// does this body contain a CALL? then the const-folder cannot produce the program's value (it is produced by
// RUNNING the emitted code), so the driver reports the TYPE only -- never a misleading "= 0".
pub fn fop_in(o: &str) -> bool {
    let b = o.as_bytes();
    b.len() > 1 && b[0] == 102
}
pub fn has_call(b: &Body) -> bool {
    ops_have_call(&b.ops)
}
fn ops_have_call(ops: &Vec<Op>) -> bool {
    let mut k = 0;
    while k < ops.len() {
        match &ops[k] {
            Op::RetEarly(_t) => {}
            // a FLOAT op is not foldable by an i64 interpreter: treat it as a call so the value is
            // computed by the emitted code, never by the folder
            Op::Bin(_t, o, _x, _y) if fop_in(o) => {
                return true;
            }
            Op::Brk(_b) => {
                return true;
            }
            Op::BrkN(_b, _n) => {
                return true;
            }
            Op::Cont(_c) => {
                return true;
            }
            Op::ContN(_c, _n) => {
                return true;
            }
            Op::CallOp(_t, _n, _a) => {
                return true;
            }
            Op::StrC(_t, _v) => {
                return true;
            }
            Op::IfOp(_t, _c, tops, _tr, eops, _er) => {
                if ops_have_call(tops) {
                    return true;
                }
                if ops_have_call(eops) {
                    return true;
                }
            }
            Op::LoopOp(cops, _cr, bops) => {
                if ops_have_call(cops) {
                    return true;
                }
                if ops_have_call(bops) {
                    return true;
                }
            }
            Op::MatchOp(_d, _s, arms) => {
                let mut q = 0;
                while q < arms.len() {
                    if ops_have_call(&arms[q].ops) {
                        return true;
                    }
                    q = q + 1;
                }
            }
            _ => {}
        }
        k = k + 1;
    }
    false
}
// CONST-DIV0 (rustc_mir_transform const-prop lint `unconditional_panic`, deny by default): forward constant propagation
// over the body's top-level ops -- temps and locals that hold known constants -- reports a `/` or `%` whose divisor is
// KNOWN to be zero. Control flow (if/loop/match) forgets the known locals, so the pass never claims more than it knows.
pub fn const_div0(b: &Body) -> bool {
    let mut tk: Vec<u32> = Vec::new();
    let mut tv: Vec<i64> = Vec::new();
    let mut lk: Vec<u32> = Vec::new();
    let mut lv: Vec<i64> = Vec::new();
    let mut i = 0;
    while i < b.ops.len() {
        match &b.ops[i] {
            Op::Const(t, v) => {
                tk.push(*t);
                tv.push(*v);
            }
            Op::Bin(t, o, x, y) => {
                let ky = kget(&tk, &tv, *y);
                let kx = kget(&tk, &tv, *x);
                if (o.as_str() == "/"
                    || o.as_str() == "%"
                    || o.as_str() == "u/"
                    || o.as_str() == "u%")
                    && ky.len() == 1
                    && ky[0] == 0
                {
                    return true;
                }
                if kx.len() == 1 && ky.len() == 1 {
                    let a = kx[0];
                    let c = ky[0];
                    let r = if o.as_str() == "+" {
                        a + c
                    } else if o.as_str() == "-" {
                        a - c
                    } else if o.as_str() == "*" {
                        a * c
                    } else {
                        0
                    };
                    if o.as_str() == "+" || o.as_str() == "-" || o.as_str() == "*" {
                        tk.push(*t);
                        tv.push(r);
                    }
                }
            }
            Op::StoreLoc(l, t) => {
                let k = kget(&tk, &tv, *t);
                kdel(&mut lk, &mut lv, *l);
                if k.len() == 1 {
                    lk.push(*l);
                    lv.push(k[0]);
                }
            }
            Op::LoadLoc(t, l) => {
                let k = kget(&lk, &lv, *l);
                if k.len() == 1 {
                    tk.push(*t);
                    tv.push(k[0]);
                }
            }
            Op::IfOp(_a, _b, _c, _d, _e, _f) => {
                lk = Vec::new();
                lv = Vec::new();
            }
            Op::LoopOp(_a, _b, _c) => {
                lk = Vec::new();
                lv = Vec::new();
            }
            Op::MatchOp(_a, _b, _c) => {
                lk = Vec::new();
                lv = Vec::new();
            }
            _ => {}
        }
        i = i + 1;
    }
    false
}
fn kget(ks: &Vec<u32>, vs: &Vec<i64>, k: u32) -> Vec<i64> {
    let mut r: Vec<i64> = Vec::new();
    let mut i = ks.len();
    while i > 0 {
        i = i - 1;
        if ks[i] == k {
            r.push(vs[i]);
            return r;
        }
    }
    r
}
fn kdel(ks: &mut Vec<u32>, vs: &mut Vec<i64>, k: u32) {
    let mut i = 0;
    while i < ks.len() {
        if ks[i] == k {
            ks.remove(i);
            vs.remove(i);
        } else {
            i = i + 1;
        }
    }
}
// U-DIV: unsigned 64-bit division of two i64 bit patterns, with i64 arithmetic only (folds `!0u64 / 2`; signed it was 0)
fn ult64(a: i64, b: i64) -> bool {
    let mn: i64 = 0 - 9223372036854775807 - 1;
    (a ^ mn) < (b ^ mn)
}
fn udiv64(a: i64, c: i64) -> i64 {
    if c == 0 {
        return 0;
    }
    if c < 0 {
        if ult64(a, c) {
            return 0;
        }
        return 1;
    }
    if a >= 0 {
        return a / c;
    }
    let half = (a >> 1) & 9223372036854775807;
    let mut q = (half / c) << 1;
    let r = a.wrapping_sub(q.wrapping_mul(c));
    if !ult64(r, c) {
        q = q + 1;
    }
    q
}
