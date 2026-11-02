pub enum Op {
    CastOp(u32, u32, String),
    Brk(u32),
    BrkN(u32, u32),
    Cont(u32),
    ContN(u32, u32),
    RetEarly(u32),
    StrC(u32, String),
    ParamOp(u32, u32),
    CallOp(u32, String, Vec<u32>),
    Const(u32, i64),
    Bin(u32, String, u32, u32),
    IfOp(u32, u32, Vec<Op>, u32, Vec<Op>, u32),
    LoopOp(Vec<Op>, u32, Vec<Op>),
    StoreLoc(u32, u32),
    LoadLoc(u32, u32),
    MakeStruct(u32, u32, Vec<u32>),
    LoadField(u32, u32, u32),
    MakeEnum(u32, u32, Vec<u32>),
    MatchOp(u32, u32, Vec<MArm>),
    MakeArr(u32, u32, Vec<u32>),
    LoadIdx(u32, u32, u32),
    StoreIdx(u32, u32, u32),
    StoreField(u32, u32, u32),
    Ret(u32),
}
pub struct MArm {
    pub tag: i32,
    pub slots: Vec<u32>,
    pub ops: Vec<Op>,
    pub res: u32,
}
pub struct Body {
    pub ops: Vec<Op>,
    pub ntmp: u32,
    pub nloc: u32,
}
pub fn veccopy_u32(v: &Vec<u32>) -> Vec<u32> {
    let mut o: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < v.len() {
        o.push(v[i]);
        i = i + 1;
    }
    o
}
fn t_new(b: &mut Body) -> u32 {
    let t = b.ntmp;
    b.ntmp = t + 1;
    t
}
fn note_loc(b: &mut Body, s: u32) {
    if s + 1 > b.nloc {
        b.nloc = s + 1;
    }
}
pub fn build(h: &H, b: &mut Body) -> u32 {
    match h {
        H::StoreIdxH(bs, ix, v, _i) => {
            let bt = build(bs, b);
            let it = build(ix, b);
            let vt = build(v, b);
            b.ops.push(Op::StoreIdx(bt, it, vt));
            vt
        }
        H::StoreFieldH(bs, fx, v, _i) => {
            let bt = build(bs, b);
            let vt = build(v, b);
            b.ops.push(Op::StoreField(bt, *fx, vt));
            vt
        }
        H::ArrLitH(xs, _i) => {
            let mut av: Vec<u32> = Vec::new();
            let mut k = 0;
            while k < xs.len() {
                av.push(build(&xs[k], b));
                k = k + 1;
            }
            let t = t_new(b);
            b.ops.push(Op::MakeArr(t, xs.len() as u32, av));
            t
        }
        H::IndexH(bs, ix, _i) => {
            let bt = build(bs, b);
            let it = build(ix, b);
            let t = t_new(b);
            b.ops.push(Op::LoadIdx(t, bt, it));
            t
        }
        H::EnumLitH(tag, ps, _i) => {
            let mut av: Vec<u32> = Vec::new();
            let mut k = 0;
            while k < ps.len() {
                av.push(build(&ps[k], b));
                k = k + 1;
            }
            let t = t_new(b);
            b.ops.push(Op::MakeEnum(t, *tag, av));
            t
        }
        H::MatchH(sc, arms, _i) => {
            let st = build(sc, b);
            let dest = t_new(b);
            let mut ma: Vec<MArm> = Vec::new();
            let mut k = 0;
            while k < arms.len() {
                let a = &arms[k];
                let mut ab = Body {
                    ops: Vec::new(),
                    ntmp: b.ntmp,
                    nloc: b.nloc,
                };
                let r = build(&a.body, &mut ab);
                b.ntmp = ab.ntmp;
                if ab.nloc > b.nloc {
                    b.nloc = ab.nloc;
                }
                let mut sl: Vec<u32> = Vec::new();
                let mut q = 0;
                while q < a.slots.len() {
                    sl.push(a.slots[q]);
                    if a.slots[q] + 1 > b.nloc {
                        b.nloc = a.slots[q] + 1;
                    }
                    q = q + 1;
                }
                ma.push(MArm {
                    tag: a.tag,
                    slots: sl,
                    ops: ab.ops,
                    res: r,
                });
                k = k + 1;
            }
            b.ops.push(Op::MatchOp(dest, st, ma));
            dest
        }
        H::StructLitH(nf, fs, _i) => {
            let mut av: Vec<u32> = Vec::new();
            let mut k = 0;
            while k < fs.len() {
                av.push(build(&fs[k], b));
                k = k + 1;
            }
            let t = t_new(b);
            b.ops.push(Op::MakeStruct(t, *nf, av));
            t
        }
        H::FieldH(base, ix, _i) => {
            let bt = build(base, b);
            let t = t_new(b);
            b.ops.push(Op::LoadField(t, bt, *ix));
            t
        }
        H::Lit(v, _i) => {
            let t = t_new(b);
            b.ops.push(Op::Const(t, *v));
            t
        }
        H::CastH(x, t, _i) => {
            let sx = build(x, b);
            let r = t_new(b);
            b.ops.push(Op::CastOp(r, sx, String::from(t.as_str())));
            r
        }
        H::BreakH(_i) => {
            let bid = t_new(b);
            b.ops.push(Op::Brk(bid));
            let t = t_new(b);
            b.ops.push(Op::Const(t, 0));
            t
        }
        H::BreakNH(n, _i) => {
            let bid = t_new(b);
            b.ops.push(Op::BrkN(bid, *n));
            let t = t_new(b);
            b.ops.push(Op::Const(t, 0));
            t
        }
        H::ContH(_i) => {
            let cid = t_new(b);
            b.ops.push(Op::Cont(cid));
            let t = t_new(b);
            b.ops.push(Op::Const(t, 0));
            t
        }
        H::ContNH(n, _i) => {
            let cid = t_new(b);
            b.ops.push(Op::ContN(cid, *n));
            let t = t_new(b);
            b.ops.push(Op::Const(t, 0));
            t
        }
        H::RetH(x, _i) => {
            let t = build(x, b);
            b.ops.push(Op::RetEarly(t));
            t
        }
        H::StrH(v, _i) => {
            let t = t_new(b);
            b.ops.push(Op::StrC(t, String::from(v.as_str())));
            t
        }
        H::LitB(v, _i) => {
            let t = t_new(b);
            b.ops.push(Op::Const(t, if *v { 1 } else { 0 }));
            t
        }
        H::Local(s, _i) => {
            note_loc(b, *s);
            let t = t_new(b);
            b.ops.push(Op::LoadLoc(t, *s));
            t
        }
        H::AssignH(s, v, _i) => {
            let vt = build(v, b);
            note_loc(b, *s);
            b.ops.push(Op::StoreLoc(*s, vt));
            vt
        }
        H::SeqH(a, c, _i) => {
            let _ = build(a, b);
            build(c, b)
        }
        H::BinH(o, x, y, _i) => {
            let a = build(x, b);
            let c = build(y, b);
            let t = t_new(b);
            b.ops.push(Op::Bin(t, String::from(o.as_str()), a, c));
            t
        }
        H::CallH(n, args, _i) => {
            let mut av: Vec<u32> = Vec::new();
            let mut k = 0;
            while k < args.len() {
                av.push(build(&args[k], b));
                k = k + 1;
            }
            let t = t_new(b);
            b.ops.push(Op::CallOp(t, String::from(n.as_str()), av));
            t
        }
        H::IfH(cd, x, y, _i) => {
            let c0 = build(cd, b);
            let mut tb = Body {
                ops: Vec::new(),
                ntmp: b.ntmp,
                nloc: b.nloc,
            };
            let tr = build(x, &mut tb);
            b.ntmp = tb.ntmp;
            if tb.nloc > b.nloc {
                b.nloc = tb.nloc;
            }
            let mut eb = Body {
                ops: Vec::new(),
                ntmp: b.ntmp,
                nloc: b.nloc,
            };
            let er = build(y, &mut eb);
            b.ntmp = eb.ntmp;
            if eb.nloc > b.nloc {
                b.nloc = eb.nloc;
            }
            let t = t_new(b);
            b.ops.push(Op::IfOp(t, c0, tb.ops, tr, eb.ops, er));
            t
        }
        H::WhileH(c, bd, _i) => {
            let mut cb = Body {
                ops: Vec::new(),
                ntmp: b.ntmp,
                nloc: b.nloc,
            };
            let cr = build(c, &mut cb);
            b.ntmp = cb.ntmp;
            if cb.nloc > b.nloc {
                b.nloc = cb.nloc;
            }
            let mut bb = Body {
                ops: Vec::new(),
                ntmp: b.ntmp,
                nloc: b.nloc,
            };
            let _ = build(bd, &mut bb);
            b.ntmp = bb.ntmp;
            if bb.nloc > b.nloc {
                b.nloc = bb.nloc;
            }
            b.ops.push(Op::LoopOp(cb.ops, cr, bb.ops));
            let t = t_new(b);
            b.ops.push(Op::Const(t, 0));
            t
        }
        H::LetH(s, v, bd, _i) => {
            let vt = build(v, b);
            note_loc(b, *s);
            b.ops.push(Op::StoreLoc(*s, vt));
            build(bd, b)
        }
    }
}
