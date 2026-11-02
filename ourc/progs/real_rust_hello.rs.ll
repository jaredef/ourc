@.s0000000000001505 = private unnamed_addr constant [1 x i8] c"\00"
@.s0000000000151a35 = private unnamed_addr constant [2 x i8] c"0\00"
@.s3bb13c327e0e3112 = private unnamed_addr constant [11 x i8] c"0123456789\00"
@.s00000000152f4c63 = private unnamed_addr constant [3 x i8] c"-1\00"
@.s0000000000151a36 = private unnamed_addr constant [2 x i8] c"1\00"
@.s0000000000151a32 = private unnamed_addr constant [2 x i8] c"-\00"
@.sc4f6992bd7eb6267 = private unnamed_addr constant [17 x i8] c"0123456789abcdef\00"
@.s0000000000151a60 = private unnamed_addr constant [2 x i8] c"[\00"
@.s00000000152f4b51 = private unnamed_addr constant [3 x i8] c", \00"
@.s0000000000151a62 = private unnamed_addr constant [2 x i8] c"]\00"
@.s0000155a0842dfc5 = private unnamed_addr constant [5 x i8] c"true\00"
@.s00156f5401923610 = private unnamed_addr constant [6 x i8] c"false\00"
@.s00000015447fb494 = private unnamed_addr constant [4 x i8] c"1.0\00"
@.s00000015447eb293 = private unnamed_addr constant [4 x i8] c"0.0\00"
@.s3e554ab522664be4 = private unnamed_addr constant [27 x i8] c"ABCDEFGHIJKLMNOPQRSTUVWXYZ\00"
@.s3fe03083278eef24 = private unnamed_addr constant [27 x i8] c"abcdefghijklmnopqrstuvwxyz\00"
@.s0000000000151a27 = private unnamed_addr constant [2 x i8] c"\22\00"
@.s00000000152f7b83 = private unnamed_addr constant [3 x i8] c"\5C\22\00"
@.s00000000152f7bbd = private unnamed_addr constant [3 x i8] c"\5C\5C\00"
@.s00000000152f7bcf = private unnamed_addr constant [3 x i8] c"\5Cn\00"
@.s00000000152f7bd5 = private unnamed_addr constant [3 x i8] c"\5Ct\00"
@.s00000000152f7b88 = private unnamed_addr constant [3 x i8] c"\5C'\00"
@.s0000000000151a2c = private unnamed_addr constant [2 x i8] c"'\00"
@.s4e85d907520ae4b0 = private unnamed_addr constant [19 x i8] c"9007199254740992.0\00"
@.s00000015447eb298 = private unnamed_addr constant [4 x i8] c"0.5\00"
@.s0000000000151a33 = private unnamed_addr constant [2 x i8] c".\00"
@.s0000000000151a6a = private unnamed_addr constant [2 x i8] c"e\00"
@.s0000000000151a30 = private unnamed_addr constant [2 x i8] c"+\00"
@.s00000015449d2202 = private unnamed_addr constant [4 x i8] c"NaN\00"
@.s0000001544b86542 = private unnamed_addr constant [4 x i8] c"inf\00"
@.s00001559c063f16f = private unnamed_addr constant [5 x i8] c"-inf\00"
@.s00001559c46bb802 = private unnamed_addr constant [5 x i8] c"1e16\00"
@.s00000000152f4d63 = private unnamed_addr constant [3 x i8] c".0\00"
@.fmtd = private unnamed_addr constant [5 x i8] c"%ld\0A\00"
@.fmts = private unnamed_addr constant [3 x i8] c"%s\00"
@.fmtdr = private unnamed_addr constant [4 x i8] c"%ld\00"
@.fmtur = private unnamed_addr constant [4 x i8] c"%lu\00"
@.fmtnl = private unnamed_addr constant [2 x i8] c"\0A\00"
@.fmtc = private unnamed_addr constant [3 x i8] c"%c\00"
@.fmtfp = private unnamed_addr constant [5 x i8] c"%.*f\00"
@.fmtfe = private unnamed_addr constant [5 x i8] c"%.*e\00"
declare double @llvm.sin.f64(double)
declare double @llvm.cos.f64(double)
declare double @llvm.exp.f64(double)
declare double @llvm.exp2.f64(double)
declare double @llvm.log.f64(double)
declare double @llvm.log10.f64(double)
declare double @llvm.log2.f64(double)
declare double @llvm.pow.f64(double, double)
declare double @llvm.sqrt.f64(double)
declare i8 @llvm.fptosi.sat.i8.f64(double)
declare i16 @llvm.fptosi.sat.i16.f64(double)
declare i32 @llvm.fptosi.sat.i32.f64(double)
declare i64 @llvm.fptosi.sat.i64.f64(double)
declare i8 @llvm.fptoui.sat.i8.f64(double)
declare i16 @llvm.fptoui.sat.i16.f64(double)
declare i32 @llvm.fptoui.sat.i32.f64(double)
declare i64 @llvm.fptoui.sat.i64.f64(double)
declare void @exit(i32)
declare i32 @puts(i8*)
declare i64 @strlen(i8*)
declare i32 @strcmp(i8*, i8*)
declare i32 @printf(i8*, ...)
declare i8* @malloc(i64)
declare i8* @realloc(i8*, i64)
declare double @strtod(i8*, i8**)
declare i32 @snprintf(i8*, i64, i8*, ...)
define i64 @__divchk(i64 %d) {
entry:
  %z = icmp eq i64 %d, 0
  br i1 %z, label %bad, label %ok
bad:
  call void @exit(i32 101)
  unreachable
ok:
  ret i64 %d
}
define i64 @__ixchk(i64 %i, i64 %n) {
entry:
  %lo = icmp slt i64 %i, 0
  %hi = icmp sge i64 %i, %n
  %bad = or i1 %lo, %hi
  br i1 %bad, label %oob, label %ok
oob:
  call void @exit(i32 101)
  unreachable
ok:
  ret i64 %i
}
define i64 @__vec_new() {
entry:
  %p = call i8* @malloc(i64 24)
  %h = bitcast i8* %p to i64*
  store i64 0, i64* %h
  %cp = getelementptr i64, i64* %h, i64 1
  store i64 4, i64* %cp
  %d = call i8* @malloc(i64 32)
  %di = ptrtoint i8* %d to i64
  %dp = getelementptr i64, i64* %h, i64 2
  store i64 %di, i64* %dp
  %r = ptrtoint i64* %h to i64
  ret i64 %r
}
define i64 @__vec_push(i64 %v, i64 %x) {
entry:
  %h = inttoptr i64 %v to i64*
  %len = load i64, i64* %h
  %cp = getelementptr i64, i64* %h, i64 1
  %cap = load i64, i64* %cp
  %full = icmp sge i64 %len, %cap
  br i1 %full, label %grow, label %put
grow:
  %nc = mul i64 %cap, 2
  %dpg = getelementptr i64, i64* %h, i64 2
  %old = load i64, i64* %dpg
  %oldp = inttoptr i64 %old to i8*
  %bytes = mul i64 %nc, 8
  %new = call i8* @realloc(i8* %oldp, i64 %bytes)
  %newi = ptrtoint i8* %new to i64
  store i64 %newi, i64* %dpg
  store i64 %nc, i64* %cp
  br label %put
put:
  %dp2 = getelementptr i64, i64* %h, i64 2
  %d = load i64, i64* %dp2
  %dpp = inttoptr i64 %d to i64*
  %slot = getelementptr i64, i64* %dpp, i64 %len
  store i64 %x, i64* %slot
  %nl = add i64 %len, 1
  store i64 %nl, i64* %h
  ret i64 %nl
}
define i64 @__vec_get(i64 %v, i64 %i) {
entry:
  %h = inttoptr i64 %v to i64*
  %ln = load i64, i64* %h
  %ck = call i64 @__ixchk(i64 %i, i64 %ln)
  %dp = getelementptr i64, i64* %h, i64 2
  %d = load i64, i64* %dp
  %dpp = inttoptr i64 %d to i64*
  %slot = getelementptr i64, i64* %dpp, i64 %ck
  %r = load i64, i64* %slot
  ret i64 %r
}
define i64 @__vec_elem_addr(i64 %v, i64 %i) {
entry:
  %h = inttoptr i64 %v to i64*
  %dp = getelementptr i64, i64* %h, i64 2
  %d = load i64, i64* %dp
  %dpp = inttoptr i64 %d to i64*
  %slot = getelementptr i64, i64* %dpp, i64 %i
  %r = ptrtoint i64* %slot to i64
  ret i64 %r
}
define i64 @__fld_addr(i64 %b, i64 %i) {
entry:
  %p = inttoptr i64 %b to i64*
  %q = getelementptr i64, i64* %p, i64 %i
  %r = ptrtoint i64* %q to i64
  ret i64 %r
}
define i64 @__store_at(i64 %a, i64 %x) {
entry:
  %p = inttoptr i64 %a to i64*
  store i64 %x, i64* %p
  ret i64 0
}
define i64 @__load_at(i64 %a) {
entry:
  %p = inttoptr i64 %a to i64*
  %r = load i64, i64* %p
  ret i64 %r
}
define i64 @__vec_set(i64 %v, i64 %i, i64 %x) {
entry:
  %h = inttoptr i64 %v to i64*
  %dp = getelementptr i64, i64* %h, i64 2
  %d = load i64, i64* %dp
  %dpp = inttoptr i64 %d to i64*
  %slot = getelementptr i64, i64* %dpp, i64 %i
  store i64 %x, i64* %slot
  ret i64 %x
}
define i64 @__vec_popraw(i64 %v) {
entry:
  %h = inttoptr i64 %v to i64*
  %n = load i64, i64* %h
  %n1 = sub i64 %n, 1
  store i64 %n1, i64* %h
  %dp = getelementptr i64, i64* %h, i64 2
  %d = load i64, i64* %dp
  %dpp = inttoptr i64 %d to i64*
  %slot = getelementptr i64, i64* %dpp, i64 %n1
  %r = load i64, i64* %slot
  ret i64 %r
}
define i64 @__arr_get(i64 %a, i64 %i) {
entry:
  %p = inttoptr i64 %a to i64*
  %n = load i64, i64* %p
  %ck = call i64 @__ixchk(i64 %i, i64 %n)
  %ix = add i64 1, %ck
  %ep = getelementptr i64, i64* %p, i64 %ix
  %r = load i64, i64* %ep
  ret i64 %r
}
define i64 @__arr_set(i64 %a, i64 %i, i64 %v) {
entry:
  %p = inttoptr i64 %a to i64*
  %n = load i64, i64* %p
  %ck = call i64 @__ixchk(i64 %i, i64 %n)
  %ix = add i64 1, %ck
  %ep = getelementptr i64, i64* %p, i64 %ix
  store i64 %v, i64* %ep
  ret i64 %v
}
define i64 @__arr_len(i64 %a) {
entry:
  %p = inttoptr i64 %a to i64*
  %n = load i64, i64* %p
  ret i64 %n
}
define i64 @__enumeq(i64 %a, i64 %b, i64 %n) {
entry:
  %ap = inttoptr i64 %a to i64*
  %bp = inttoptr i64 %b to i64*
  br label %loop
loop:
  %i = phi i64 [ 0, %entry ], [ %i2, %body ]
  %c = icmp slt i64 %i, %n
  br i1 %c, label %body, label %same
body:
  %aep = getelementptr i64, i64* %ap, i64 %i
  %bep = getelementptr i64, i64* %bp, i64 %i
  %av = load i64, i64* %aep
  %bv = load i64, i64* %bep
  %eq = icmp eq i64 %av, %bv
  %i2 = add i64 %i, 1
  br i1 %eq, label %loop, label %diff
diff:
  ret i64 0
same:
  ret i64 1
}
define i64 @__vec_slice(i64 %v, i64 %a, i64 %b) {
entry:
  %n = call i64 @__vec_new()
  br label %loop
loop:
  %i = phi i64 [ %a, %entry ], [ %i2, %body ]
  %c = icmp slt i64 %i, %b
  br i1 %c, label %body, label %done
body:
  %x = call i64 @__vec_get(i64 %v, i64 %i)
  %p = call i64 @__vec_push(i64 %n, i64 %x)
  %i2 = add i64 %i, 1
  br label %loop
done:
  ret i64 %n
}
define i64 @__arr_slice(i64 %v, i64 %a, i64 %b) {
entry:
  %n = call i64 @__vec_new()
  %bp = inttoptr i64 %v to i64*
  br label %loop
loop:
  %i = phi i64 [ %a, %entry ], [ %i2, %body ]
  %c = icmp slt i64 %i, %b
  br i1 %c, label %body, label %done
body:
  %ix = add i64 1, %i
  %ep = getelementptr i64, i64* %bp, i64 %ix
  %x = load i64, i64* %ep
  %p = call i64 @__vec_push(i64 %n, i64 %x)
  %i2 = add i64 %i, 1
  br label %loop
done:
  ret i64 %n
}
define i64 @__arr_fill(i64 %n, i64 %x) {
entry:
  %n1 = add i64 %n, 1
  %sz = mul i64 %n1, 8
  %p = call i8* @malloc(i64 %sz)
  %h = bitcast i8* %p to i64*
  store i64 %n, i64* %h
  br label %lc
lc:
  %i = phi i64 [ 1, %entry ], [ %i2, %lb ]
  %more = icmp sle i64 %i, %n
  br i1 %more, label %lb, label %done
lb:
  %ep = getelementptr i64, i64* %h, i64 %i
  store i64 %x, i64* %ep
  %i2 = add i64 %i, 1
  br label %lc
done:
  %r = ptrtoint i8* %p to i64
  ret i64 %r
}
define i64 @__arreq(i64 %a, i64 %b) {
entry:
  %pa = inttoptr i64 %a to i64*
  %pb = inttoptr i64 %b to i64*
  %na = load i64, i64* %pa
  %nb = load i64, i64* %pb
  %sl = icmp eq i64 %na, %nb
  br i1 %sl, label %go, label %ne
ne:
  ret i64 0
go:
  br label %lc
lc:
  %i = phi i64 [ 1, %go ], [ %i2, %lb2 ]
  %more = icmp sle i64 %i, %na
  br i1 %more, label %lb2, label %eq
lb2:
  %ga = getelementptr i64, i64* %pa, i64 %i
  %gb = getelementptr i64, i64* %pb, i64 %i
  %va = load i64, i64* %ga
  %vb = load i64, i64* %gb
  %same = icmp eq i64 %va, %vb
  %i2 = add i64 %i, 1
  br i1 %same, label %lc, label %ne
eq:
  ret i64 1
}
define i64 @__veceq(i64 %a, i64 %b) {
entry:
  %ha = inttoptr i64 %a to i64*
  %hb = inttoptr i64 %b to i64*
  %la = load i64, i64* %ha
  %lb = load i64, i64* %hb
  %sl = icmp eq i64 %la, %lb
  br i1 %sl, label %go, label %ne
ne:
  ret i64 0
go:
  %dpa = getelementptr i64, i64* %ha, i64 2
  %dpb = getelementptr i64, i64* %hb, i64 2
  %da = load i64, i64* %dpa
  %db = load i64, i64* %dpb
  %pa = inttoptr i64 %da to i64*
  %pb = inttoptr i64 %db to i64*
  br label %lc
lc:
  %i = phi i64 [ 0, %go ], [ %i2, %lb2 ]
  %more = icmp slt i64 %i, %la
  br i1 %more, label %lb2, label %eq
lb2:
  %ga = getelementptr i64, i64* %pa, i64 %i
  %gb = getelementptr i64, i64* %pb, i64 %i
  %va = load i64, i64* %ga
  %vb = load i64, i64* %gb
  %same = icmp eq i64 %va, %vb
  %i2 = add i64 %i, 1
  br i1 %same, label %lc, label %ne
eq:
  ret i64 1
}
define i64 @__rc_slot(i64 %p) {
entry:
  %n = load i64, i64* @__rcn
  br label %lc
lc:
  %i = phi i64 [ 0, %entry ], [ %i2, %nx ]
  %more = icmp slt i64 %i, %n
  br i1 %more, label %lb, label %miss
lb:
  %ep = getelementptr [65536 x i64], [65536 x i64]* @__rcp, i64 0, i64 %i
  %e = load i64, i64* %ep
  %hit = icmp eq i64 %e, %p
  br i1 %hit, label %found, label %nx
nx:
  %i2 = add i64 %i, 1
  br label %lc
found:
  ret i64 %i
miss:
  ret i64 -1
}
define i64 @__rc_new(i64 %p) {
entry:
  %k = call i64 @__rc_slot(i64 %p)
  %have = icmp sge i64 %k, 0
  br i1 %have, label %set, label %add
add:
  %n = load i64, i64* @__rcn
  %pp = getelementptr [65536 x i64], [65536 x i64]* @__rcp, i64 0, i64 %n
  store i64 %p, i64* %pp
  %n2 = add i64 %n, 1
  store i64 %n2, i64* @__rcn
  br label %set
set:
  %s = phi i64 [ %k, %entry ], [ %n, %add ]
  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %s
  store i64 1, i64* %cp
  ret i64 %p
}
define i64 @__rc_inc(i64 %p) {
entry:
  %k = call i64 @__rc_slot(i64 %p)
  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k
  %c = load i64, i64* %cp
  %c2 = add i64 %c, 1
  store i64 %c2, i64* %cp
  ret i64 %p
}
define i64 @__rc_dec(i64 %p) {
entry:
  %k = call i64 @__rc_slot(i64 %p)
  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k
  %c = load i64, i64* %cp
  %c2 = sub i64 %c, 1
  store i64 %c2, i64* %cp
  ret i64 %c2
}
define i64 @__rc_count(i64 %p) {
entry:
  %k = call i64 @__rc_slot(i64 %p)
  %cp = getelementptr [65536 x i64], [65536 x i64]* @__rcc, i64 0, i64 %k
  %c = load i64, i64* %cp
  ret i64 %c
}
define i64 @__vec_len(i64 %v) {
entry:
  %h = inttoptr i64 %v to i64*
  %r = load i64, i64* %h
  ret i64 %r
}
declare i8* @memcpy(i8*, i8*, i64)
define i64 @__str_at(i64 %s, i64 %i) {
entry:
  %p = inttoptr i64 %s to i8*
  %q = getelementptr i8, i8* %p, i64 %i
  %c = load i8, i8* %q
  %r = zext i8 %c to i64
  ret i64 %r
}
define i64 @__vec_push1(i64 %v, i64 %x) {
entry:
  %n = call i64 @__vec_push(i64 %v, i64 %x)
  ret i64 %v
}
define i64 @__str_copy(i64 %a) {
entry:
  %ap = inttoptr i64 %a to i8*
  %la = call i64 @strlen(i8* %ap)
  %n1 = add i64 %la, 1
  %d = call i8* @malloc(i64 %n1)
  %x = call i8* @memcpy(i8* %d, i8* %ap, i64 %n1)
  %r = ptrtoint i8* %d to i64
  ret i64 %r
}
define i64 @__str_cat(i64 %a, i64 %b) {
entry:
  %ap = inttoptr i64 %a to i8*
  %bp = inttoptr i64 %b to i8*
  %la = call i64 @strlen(i8* %ap)
  %lb = call i64 @strlen(i8* %bp)
  %t1 = add i64 %la, %lb
  %t2 = add i64 %t1, 1
  %d = call i8* @malloc(i64 %t2)
  %c1 = call i8* @memcpy(i8* %d, i8* %ap, i64 %la)
  %dh = getelementptr i8, i8* %d, i64 %la
  %lb1 = add i64 %lb, 1
  %c2 = call i8* @memcpy(i8* %dh, i8* %bp, i64 %lb1)
  %r = ptrtoint i8* %d to i64
  ret i64 %r
}
@__gp = global [16 x i64] zeroinitializer
@__gl = global [16 x i64] zeroinitializer
@__gc = global [16 x i64] zeroinitializer
@__gi = global i64 0
define i64 @__str_grow(i64 %a, i64 %b) {
entry:
  %ap = inttoptr i64 %a to i8*
  %bp = inttoptr i64 %b to i8*
  %lb = call i64 @strlen(i8* %bp)
  br label %scan
scan:
  %i = phi i64 [ 0, %entry ], [ %i2, %next ]
  %pp = getelementptr [16 x i64], [16 x i64]* @__gp, i64 0, i64 %i
  %pv = load i64, i64* %pp
  %eq = icmp eq i64 %pv, %a
  br i1 %eq, label %fnd, label %next
next:
  %i2 = add i64 %i, 1
  %more = icmp ult i64 %i2, 16
  br i1 %more, label %scan, label %noca
fnd:
  %lpp = getelementptr [16 x i64], [16 x i64]* @__gl, i64 0, i64 %i
  %cpp = getelementptr [16 x i64], [16 x i64]* @__gc, i64 0, i64 %i
  %gl = load i64, i64* %lpp
  %gc = load i64, i64* %cpp
  %nl1 = add i64 %gl, %lb
  %nd1 = add i64 %nl1, 1
  %fits = icmp ule i64 %nd1, %gc
  br i1 %fits, label %inpl, label %fr2
inpl:
  %dh0 = getelementptr i8, i8* %ap, i64 %gl
  %lb1 = add i64 %lb, 1
  %x0 = call i8* @memcpy(i8* %dh0, i8* %bp, i64 %lb1)
  store i64 %nl1, i64* %lpp
  ret i64 %a
noca:
  %la2 = call i64 @strlen(i8* %ap)
  br label %frsh
fr2:
  br label %frsh
frsh:
  %la = phi i64 [ %gl, %fr2 ], [ %la2, %noca ]
  %nl = add i64 %la, %lb
  %need = add i64 %nl, 1
  %cap0 = mul i64 %need, 2
  %cap = add i64 %cap0, 64
  %d = call i8* @malloc(i64 %cap)
  %x1 = call i8* @memcpy(i8* %d, i8* %ap, i64 %la)
  %dh = getelementptr i8, i8* %d, i64 %la
  %lb2 = add i64 %lb, 1
  %x2 = call i8* @memcpy(i8* %dh, i8* %bp, i64 %lb2)
  %r = ptrtoint i8* %d to i64
  %sl = load i64, i64* @__gi
  %sl2 = add i64 %sl, 1
  %sl3 = urem i64 %sl2, 16
  store i64 %sl3, i64* @__gi
  %spp = getelementptr [16 x i64], [16 x i64]* @__gp, i64 0, i64 %sl3
  %slp = getelementptr [16 x i64], [16 x i64]* @__gl, i64 0, i64 %sl3
  %scp = getelementptr [16 x i64], [16 x i64]* @__gc, i64 0, i64 %sl3
  store i64 %r, i64* %spp
  store i64 %nl, i64* %slp
  store i64 %cap, i64* %scp
  ret i64 %r
}
declare i8* @fopen(i8*, i8*)
declare i64 @fseek(i8*, i64, i32)
declare i64 @ftell(i8*)
declare i64 @fread(i8*, i64, i64, i8*)
declare i32 @fclose(i8*)
@.rdmode = private unnamed_addr constant [2 x i8] c"r\00"
declare i32 @fflush(i8*)
declare i32 @dup(i32)
declare i32 @dup2(i32, i32)
declare i32 @close(i32)
@__errsv = global i32 -1
define i64 @__stdio_swap(i64 %on) {
entry:
  %f0 = call i32 @fflush(i8* null)
  %is = icmp ne i64 %on, 0
  br i1 %is, label %to2, label %back
to2:
  %sv = call i32 @dup(i32 1)
  store i32 %sv, i32* @__errsv
  %d1 = call i32 @dup2(i32 2, i32 1)
  ret i64 0
back:
  %o = load i32, i32* @__errsv
  %ok = icmp sge i32 %o, 0
  br i1 %ok, label %rs, label %done
rs:
  %d2 = call i32 @dup2(i32 %o, i32 1)
  %c = call i32 @close(i32 %o)
  store i32 -1, i32* @__errsv
  br label %done
done:
  ret i64 0
}
@__argc = global i64 0
@__argv = global i64 0
@__rcp = global [65536 x i64] zeroinitializer
@__rcc = global [65536 x i64] zeroinitializer
@__rcn = global i64 0
define i64 @__argc_get() {
entry:
  %v = load i64, i64* @__argc
  ret i64 %v
}
define i64 @__argv_at(i64 %i) {
entry:
  %b = load i64, i64* @__argv
  %p = inttoptr i64 %b to i8**
  %q = getelementptr i8*, i8** %p, i64 %i
  %c = load i8*, i8** %q
  %r = ptrtoint i8* %c to i64
  ret i64 %r
}
@.strg = private unnamed_addr constant [59 x i8] c"internal: a str comparison received a non-pointer operand\0A\00"
define i64 @__chrstr(i64 %c) {
entry:
  %p = call i8* @malloc(i64 5)
  %p1 = getelementptr i8, i8* %p, i64 1
  %p2 = getelementptr i8, i8* %p, i64 2
  %p3 = getelementptr i8, i8* %p, i64 3
  %p4 = getelementptr i8, i8* %p, i64 4
  %lo6 = and i64 %c, 63
  %s6 = lshr i64 %c, 6
  %mid6 = and i64 %s6, 63
  %s12 = lshr i64 %c, 12
  %hi6 = and i64 %s12, 63
  %s18 = lshr i64 %c, 18
  %clo = or i64 %lo6, 128
  %cmid = or i64 %mid6, 128
  %chi = or i64 %hi6, 128
  %tlo = trunc i64 %clo to i8
  %tmid = trunc i64 %cmid to i8
  %thi = trunc i64 %chi to i8
  %a1 = icmp ult i64 %c, 128
  br i1 %a1, label %one, label %m2
one:
  %b = trunc i64 %c to i8
  store i8 %b, i8* %p
  store i8 0, i8* %p1
  br label %done
m2:
  %a2 = icmp ult i64 %c, 2048
  br i1 %a2, label %two, label %m3
two:
  %l2 = or i64 %s6, 192
  %t2 = trunc i64 %l2 to i8
  store i8 %t2, i8* %p
  store i8 %tlo, i8* %p1
  store i8 0, i8* %p2
  br label %done
m3:
  %a3 = icmp ult i64 %c, 65536
  br i1 %a3, label %three, label %four
three:
  %l3 = or i64 %s12, 224
  %t3 = trunc i64 %l3 to i8
  store i8 %t3, i8* %p
  store i8 %tmid, i8* %p1
  store i8 %tlo, i8* %p2
  store i8 0, i8* %p3
  br label %done
four:
  %l4 = or i64 %s18, 240
  %t4 = trunc i64 %l4 to i8
  store i8 %t4, i8* %p
  store i8 %thi, i8* %p1
  store i8 %tmid, i8* %p2
  store i8 %tlo, i8* %p3
  store i8 0, i8* %p4
  br label %done
done:
  %r = ptrtoint i8* %p to i64
  ret i64 %r
}
define i64 @__streq(i64 %a, i64 %b) {
entry:
  %la = icmp ult i64 %a, 4096
  %lb = icmp ult i64 %b, 4096
  %bad = or i1 %la, %lb
  br i1 %bad, label %oops, label %ok
oops:
  %m = getelementptr [59 x i8], [59 x i8]* @.strg, i64 0, i64 0
  %pr = call i32 @printf(i8* %m)
  call void @exit(i32 101)
  unreachable
ok:
  %pa = inttoptr i64 %a to i8*
  %pb = inttoptr i64 %b to i8*
  %c = call i32 @strcmp(i8* %pa, i8* %pb)
  %z = icmp eq i32 %c, 0
  %r = zext i1 %z to i64
  ret i64 %r
}
define i64 @__fileok(i64 %pth) {
entry:
  %p = inttoptr i64 %pth to i8*
  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0
  %f = call i8* @fopen(i8* %p, i8* %m)
  %isn = icmp eq i8* %f, null
  br i1 %isn, label %bad, label %ok
bad:
  ret i64 0
ok:
  %cl = call i32 @fclose(i8* %f)
  ret i64 1
}
define i64 @__readfile(i64 %pth) {
entry:
  %p = inttoptr i64 %pth to i8*
  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0
  %f = call i8* @fopen(i8* %p, i8* %m)
  %isn = icmp eq i8* %f, null
  br i1 %isn, label %bad, label %ok
bad:
  %z = call i8* @malloc(i64 1)
  store i8 0, i8* %z
  %zr = ptrtoint i8* %z to i64
  ret i64 %zr
ok:
  %s0 = call i64 @fseek(i8* %f, i64 0, i32 2)
  %n = call i64 @ftell(i8* %f)
  %s1 = call i64 @fseek(i8* %f, i64 0, i32 0)
  %n1 = add i64 %n, 1
  %d = call i8* @malloc(i64 %n1)
  %got = call i64 @fread(i8* %d, i64 1, i64 %n, i8* %f)
  %e = getelementptr i8, i8* %d, i64 %got
  store i8 0, i8* %e
  %cl = call i32 @fclose(i8* %f)
  %r = ptrtoint i8* %d to i64
  ret i64 %r
}
define i64 @__chr1(i64 %c) {
entry:
  %r = call i64 @__chrstr(i64 %c)
  ret i64 %r
}
define i64 @__str_sub(i64 %s, i64 %o, i64 %n) {
entry:
  %p = inttoptr i64 %s to i8*
  %q = getelementptr i8, i8* %p, i64 %o
  %n1 = add i64 %n, 1
  %d = call i8* @malloc(i64 %n1)
  %c1 = call i8* @memcpy(i8* %d, i8* %q, i64 %n)
  %e = getelementptr i8, i8* %d, i64 %n
  store i8 0, i8* %e
  %r = ptrtoint i8* %d to i64
  ret i64 %r
}
define i64 @f64_tieup(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %L9 = alloca i64
  store i64 0, i64* %L9
  %L10 = alloca i64
  store i64 0, i64* %L10
  %L11 = alloca i64
  store i64 0, i64* %L11
  %L12 = alloca i64
  store i64 0, i64* %L12
  %L13 = alloca i64
  store i64 0, i64* %L13
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 40
  %t5 = add i64 %t3, %t4
  %t6 = load i64, i64* %L0
  %t7b = call i8* @malloc(i64 1400)
  %t7d = bitcast i64 %t6 to double
  %t7w = trunc i64 %t5 to i32
  %t7c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %t7b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfp, i64 0, i64 0), i32 %t7w, double %t7d)
  %t7 = ptrtoint i8* %t7b to i64
  store i64 %t7, i64* %L3
  %t8 = load i64, i64* %L3
  %t9p = inttoptr i64 %t8 to i8*
  %t9 = call i64 @strlen(i8* %t9p)
  store i64 %t9, i64* %L4
  %t10 = add i64 0, 0
  store i64 %t10, i64* %L5
  br label %lcond20
lcond20:
  %t11 = load i64, i64* %L5
  %t12 = load i64, i64* %L4
  %t13.c = icmp slt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t20.b = icmp ne i64 %t13, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t14 = load i64, i64* %L3
  %t15 = load i64, i64* %L5
  %t16 = call i64 @__str_at(i64 %t14, i64 %t15)
  %t17 = add i64 0, 46
  %t18.c = icmp ne i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = phi i64 [ %t18, %thx20 ], [ %t19, %elx20 ]
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L5
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L5
  br label %lcond20
lend20:
  %t24 = add i64 0, 0
  %t25 = load i64, i64* %L5
  %t26 = add i64 0, 1
  %t27 = add i64 %t25, %t26
  %t28 = load i64, i64* %L1
  %t29 = add i64 %t27, %t28
  store i64 %t29, i64* %L6
  %t30 = load i64, i64* %L6
  %t31 = load i64, i64* %L4
  %t32.c = icmp sge i64 %t30, %t31
  %t32 = zext i1 %t32.c to i64
  %t39.b = icmp ne i64 %t32, 0
  br i1 %t39.b, label %then39, label %else39
then39:
  %t33 = add i64 0, 1
  br label %thx39
thx39:
  br label %join39
else39:
  %t34 = load i64, i64* %L3
  %t35 = load i64, i64* %L6
  %t36 = call i64 @__str_at(i64 %t34, i64 %t35)
  %t37 = add i64 0, 53
  %t38.c = icmp ne i64 %t36, %t37
  %t38 = zext i1 %t38.c to i64
  br label %elx39
elx39:
  br label %join39
join39:
  %t39 = phi i64 [ %t33, %thx39 ], [ %t38, %elx39 ]
  %t43.b = icmp ne i64 %t39, 0
  br i1 %t43.b, label %then43, label %else43
then43:
  %t40 = load i64, i64* %L2
  store i64 %t40, i64* %L7
  %t41 = load i64, i64* %L7
  ret i64 %t41
ar41:
  br label %thx43
thx43:
  br label %join43
else43:
  %t42 = add i64 0, 0
  br label %elx43
elx43:
  br label %join43
join43:
  %t43 = add i64 0, %t42
  %t44 = load i64, i64* %L6
  %t45 = add i64 0, 1
  %t46 = add i64 %t44, %t45
  store i64 %t46, i64* %L7
  br label %lcond49
lcond49:
  %t47 = load i64, i64* %L7
  %t48 = load i64, i64* %L4
  %t49.c = icmp slt i64 %t47, %t48
  %t49 = zext i1 %t49.c to i64
  %lc49 = icmp ne i64 %t49, 0
  br i1 %lc49, label %lbody49, label %lend49
lbody49:
  %t50 = load i64, i64* %L3
  %t51 = load i64, i64* %L7
  %t52 = call i64 @__str_at(i64 %t50, i64 %t51)
  %t53 = add i64 0, 48
  %t54.c = icmp ne i64 %t52, %t53
  %t54 = zext i1 %t54.c to i64
  %t58.b = icmp ne i64 %t54, 0
  br i1 %t58.b, label %then58, label %else58
then58:
  %t55 = load i64, i64* %L2
  store i64 %t55, i64* %L8
  %t56 = load i64, i64* %L8
  ret i64 %t56
ar56:
  br label %thx58
thx58:
  br label %join58
else58:
  %t57 = add i64 0, 0
  br label %elx58
elx58:
  br label %join58
join58:
  %t58 = add i64 0, %t57
  %t59 = load i64, i64* %L7
  %t60 = add i64 0, 1
  %t61 = add i64 %t59, %t60
  store i64 %t61, i64* %L7
  br label %lcond49
lend49:
  %t62 = add i64 0, 0
  %t63 = load i64, i64* %L6
  store i64 %t63, i64* %L8
  %t64 = load i64, i64* %L1
  %t65 = add i64 0, 0
  %t66.c = icmp eq i64 %t64, %t65
  %t66 = zext i1 %t66.c to i64
  %t69.b = icmp ne i64 %t66, 0
  br i1 %t69.b, label %then69, label %else69
then69:
  %t67 = load i64, i64* %L5
  store i64 %t67, i64* %L8
  br label %thx69
thx69:
  br label %join69
else69:
  %t68 = add i64 0, 0
  br label %elx69
elx69:
  br label %join69
join69:
  %t69 = phi i64 [ %t67, %thx69 ], [ %t68, %elx69 ]
  %t70 = load i64, i64* %L3
  %t71 = add i64 0, 0
  %t72 = load i64, i64* %L8
  %t73 = call i64 @__str_sub(i64 %t70, i64 %t71, i64 %t72)
  store i64 %t73, i64* %L9
  %t74 = load i64, i64* %L2
  %t75 = load i64, i64* %L9
  %t76 = call i64 @__streq(i64 %t74, i64 %t75)
  %t77 = add i64 0, 0
  %t78.c = icmp eq i64 %t76, %t77
  %t78 = zext i1 %t78.c to i64
  %t82.b = icmp ne i64 %t78, 0
  br i1 %t82.b, label %then82, label %else82
then82:
  %t79 = load i64, i64* %L2
  store i64 %t79, i64* %L10
  %t80 = load i64, i64* %L10
  ret i64 %t80
ar80:
  br label %thx82
thx82:
  br label %join82
else82:
  %t81 = add i64 0, 0
  br label %elx82
elx82:
  br label %join82
join82:
  %t82 = add i64 0, %t81
  %t83 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t83, i64* %L10
  %t84 = load i64, i64* %L9
  %t85p = inttoptr i64 %t84 to i8*
  %t85 = call i64 @strlen(i8* %t85p)
  %t86 = add i64 0, 1
  %t87 = sub i64 %t85, %t86
  store i64 %t87, i64* %L11
  %t88 = add i64 0, 1
  store i64 %t88, i64* %L12
  br label %lcond91
lcond91:
  %t89 = load i64, i64* %L11
  %t90 = add i64 0, 0
  %t91.c = icmp sge i64 %t89, %t90
  %t91 = zext i1 %t91.c to i64
  %lc91 = icmp ne i64 %t91, 0
  br i1 %lc91, label %lbody91, label %lend91
lbody91:
  %t92 = load i64, i64* %L9
  %t93 = load i64, i64* %L11
  %t94 = call i64 @__str_at(i64 %t92, i64 %t93)
  store i64 %t94, i64* %L13
  %t95 = load i64, i64* %L12
  %t96 = add i64 0, 1
  %t97.c = icmp eq i64 %t95, %t96
  %t97 = zext i1 %t97.c to i64
  %t102.b = icmp ne i64 %t97, 0
  br i1 %t102.b, label %then102, label %else102
then102:
  %t98 = load i64, i64* %L13
  %t99 = add i64 0, 48
  %t100.c = icmp sge i64 %t98, %t99
  %t100 = zext i1 %t100.c to i64
  br label %thx102
thx102:
  br label %join102
else102:
  %t101 = add i64 0, 0
  br label %elx102
elx102:
  br label %join102
join102:
  %t102 = phi i64 [ %t100, %thx102 ], [ %t101, %elx102 ]
  %t107.b = icmp ne i64 %t102, 0
  br i1 %t107.b, label %then107, label %else107
then107:
  %t103 = load i64, i64* %L13
  %t104 = add i64 0, 57
  %t105.c = icmp sle i64 %t103, %t104
  %t105 = zext i1 %t105.c to i64
  br label %thx107
thx107:
  br label %join107
else107:
  %t106 = add i64 0, 0
  br label %elx107
elx107:
  br label %join107
join107:
  %t107 = phi i64 [ %t105, %thx107 ], [ %t106, %elx107 ]
  %t130.b = icmp ne i64 %t107, 0
  br i1 %t130.b, label %then130, label %else130
then130:
  %t108 = load i64, i64* %L13
  %t109 = add i64 0, 57
  %t110.c = icmp eq i64 %t108, %t109
  %t110 = zext i1 %t110.c to i64
  %t123.b = icmp ne i64 %t110, 0
  br i1 %t123.b, label %then123, label %else123
then123:
  %t111 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t112 = load i64, i64* %L10
  %t113 = call i64 @__str_cat(i64 %t111, i64 %t112)
  store i64 %t113, i64* %L10
  br label %thx123
thx123:
  br label %join123
else123:
  %t114 = ptrtoint [11 x i8]* @.s3bb13c327e0e3112 to i64
  %t115 = load i64, i64* %L13
  %t116 = add i64 0, 47
  %t117 = sub i64 %t115, %t116
  %t118 = add i64 0, 1
  %t119 = call i64 @__str_sub(i64 %t114, i64 %t117, i64 %t118)
  %t120 = load i64, i64* %L10
  %t121 = call i64 @__str_cat(i64 %t119, i64 %t120)
  store i64 %t121, i64* %L10
  %t122 = add i64 0, 0
  store i64 %t122, i64* %L12
  br label %elx123
elx123:
  br label %join123
join123:
  %t123 = phi i64 [ %t113, %thx123 ], [ %t122, %elx123 ]
  br label %thx130
thx130:
  br label %join130
else130:
  %t124 = load i64, i64* %L9
  %t125 = load i64, i64* %L11
  %t126 = add i64 0, 1
  %t127 = call i64 @__str_sub(i64 %t124, i64 %t125, i64 %t126)
  %t128 = load i64, i64* %L10
  %t129 = call i64 @__str_cat(i64 %t127, i64 %t128)
  store i64 %t129, i64* %L10
  br label %elx130
elx130:
  br label %join130
join130:
  %t130 = phi i64 [ %t123, %thx130 ], [ %t129, %elx130 ]
  %t131 = load i64, i64* %L11
  %t132 = add i64 0, 1
  %t133 = sub i64 %t131, %t132
  store i64 %t133, i64* %L11
  br label %lcond91
lend91:
  %t134 = add i64 0, 0
  %t135 = load i64, i64* %L12
  %t136 = add i64 0, 1
  %t137.c = icmp eq i64 %t135, %t136
  %t137 = zext i1 %t137.c to i64
  %t157.b = icmp ne i64 %t137, 0
  br i1 %t157.b, label %then157, label %else157
then157:
  %t138 = load i64, i64* %L9
  %t139 = add i64 0, 0
  %t140 = call i64 @__str_at(i64 %t138, i64 %t139)
  %t141 = add i64 0, 45
  %t142.c = icmp eq i64 %t140, %t141
  %t142 = zext i1 %t142.c to i64
  %t155.b = icmp ne i64 %t142, 0
  br i1 %t155.b, label %then155, label %else155
then155:
  %t143 = ptrtoint [3 x i8]* @.s00000000152f4c63 to i64
  %t144 = load i64, i64* %L10
  %t145 = add i64 0, 1
  %t146 = load i64, i64* %L10
  %t147p = inttoptr i64 %t146 to i8*
  %t147 = call i64 @strlen(i8* %t147p)
  %t148 = add i64 0, 1
  %t149 = sub i64 %t147, %t148
  %t150 = call i64 @__str_sub(i64 %t144, i64 %t145, i64 %t149)
  %t151 = call i64 @__str_cat(i64 %t143, i64 %t150)
  store i64 %t151, i64* %L10
  br label %thx155
thx155:
  br label %join155
else155:
  %t152 = ptrtoint [2 x i8]* @.s0000000000151a36 to i64
  %t153 = load i64, i64* %L10
  %t154 = call i64 @__str_cat(i64 %t152, i64 %t153)
  store i64 %t154, i64* %L10
  br label %elx155
elx155:
  br label %join155
join155:
  %t155 = phi i64 [ %t151, %thx155 ], [ %t154, %elx155 ]
  br label %thx157
thx157:
  br label %join157
else157:
  %t156 = add i64 0, 0
  br label %elx157
elx157:
  br label %join157
join157:
  %t157 = phi i64 [ %t155, %thx157 ], [ %t156, %elx157 ]
  %t158 = load i64, i64* %L10
  ret i64 %t158
}
define i64 @f64_big(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %L9 = alloca i64
  store i64 0, i64* %L9
  %L10 = alloca i64
  store i64 0, i64* %L10
  %L11 = alloca i64
  store i64 0, i64* %L11
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 0
  store i64 %t2, i64* %L2
  %t3 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 17
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  %t8 = load i64, i64* %L0
  %t9b = call i8* @malloc(i64 1400)
  %t9d = bitcast i64 %t8 to double
  %t9w = trunc i64 %t7 to i32
  %t9c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %t9b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfe, i64 0, i64 0), i32 %t9w, double %t9d)
  %t9 = ptrtoint i8* %t9b to i64
  store i64 %t9, i64* %L3
  %t10 = load i64, i64* %L3
  %t11p = inttoptr i64 %t10 to i8*
  %t11d = call double @strtod(i8* %t11p, i8** null)
  %t11 = bitcast double %t11d to i64
  %t12 = load i64, i64* %L1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t17.b = icmp ne i64 %t13, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  br label %lend6
bk14_0:
  %t15 = add i64 0, 0
  br label %thx17
thx17:
  br label %join17
else17:
  %t16 = add i64 0, 0
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = phi i64 [ %t15, %thx17 ], [ %t16, %elx17 ]
  %t18 = load i64, i64* %L2
  %t19 = add i64 0, 1
  %t20 = add i64 %t18, %t19
  store i64 %t20, i64* %L2
  br label %lcond6
lend6:
  %t21 = add i64 0, 0
  %t22 = load i64, i64* %L3
  %t23p = inttoptr i64 %t22 to i8*
  %t23 = call i64 @strlen(i8* %t23p)
  store i64 %t23, i64* %L4
  %t24 = add i64 0, 0
  store i64 %t24, i64* %L5
  %t25 = add i64 0, 0
  store i64 %t25, i64* %L6
  %t26 = load i64, i64* %L3
  %t27 = add i64 0, 0
  %t28 = call i64 @__str_at(i64 %t26, i64 %t27)
  %t29 = add i64 0, 45
  %t30.c = icmp eq i64 %t28, %t29
  %t30 = zext i1 %t30.c to i64
  %t34.b = icmp ne i64 %t30, 0
  br i1 %t34.b, label %then34, label %else34
then34:
  %t31 = add i64 0, 1
  store i64 %t31, i64* %L6
  %t32 = add i64 0, 1
  store i64 %t32, i64* %L5
  br label %thx34
thx34:
  br label %join34
else34:
  %t33 = add i64 0, 0
  br label %elx34
elx34:
  br label %join34
join34:
  %t34 = phi i64 [ %t32, %thx34 ], [ %t33, %elx34 ]
  %t35 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t35, i64* %L7
  br label %lcond45
lcond45:
  %t36 = load i64, i64* %L5
  %t37 = load i64, i64* %L4
  %t38.c = icmp slt i64 %t36, %t37
  %t38 = zext i1 %t38.c to i64
  %t45.b = icmp ne i64 %t38, 0
  br i1 %t45.b, label %then45, label %else45
then45:
  %t39 = load i64, i64* %L3
  %t40 = load i64, i64* %L5
  %t41 = call i64 @__str_at(i64 %t39, i64 %t40)
  %t42 = add i64 0, 101
  %t43.c = icmp ne i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  br label %thx45
thx45:
  br label %join45
else45:
  %t44 = add i64 0, 0
  br label %elx45
elx45:
  br label %join45
join45:
  %t45 = phi i64 [ %t43, %thx45 ], [ %t44, %elx45 ]
  %lc45 = icmp ne i64 %t45, 0
  br i1 %lc45, label %lbody45, label %lend45
lbody45:
  %t46 = load i64, i64* %L3
  %t47 = load i64, i64* %L5
  %t48 = call i64 @__str_at(i64 %t46, i64 %t47)
  %t49 = add i64 0, 46
  %t50.c = icmp ne i64 %t48, %t49
  %t50 = zext i1 %t50.c to i64
  %t58.b = icmp ne i64 %t50, 0
  br i1 %t58.b, label %then58, label %else58
then58:
  %t51 = load i64, i64* %L7
  %t52 = load i64, i64* %L3
  %t53 = load i64, i64* %L5
  %t54 = add i64 0, 1
  %t55 = call i64 @__str_sub(i64 %t52, i64 %t53, i64 %t54)
  %t56 = call i64 @__str_cat(i64 %t51, i64 %t55)
  store i64 %t56, i64* %L7
  br label %thx58
thx58:
  br label %join58
else58:
  %t57 = add i64 0, 0
  br label %elx58
elx58:
  br label %join58
join58:
  %t58 = phi i64 [ %t56, %thx58 ], [ %t57, %elx58 ]
  %t59 = load i64, i64* %L5
  %t60 = add i64 0, 1
  %t61 = add i64 %t59, %t60
  store i64 %t61, i64* %L5
  br label %lcond45
lend45:
  %t62 = add i64 0, 0
  %t63 = load i64, i64* %L5
  %t64 = add i64 0, 1
  %t65 = add i64 %t63, %t64
  store i64 %t65, i64* %L5
  %t66 = add i64 0, 1
  store i64 %t66, i64* %L8
  %t67 = load i64, i64* %L3
  %t68 = load i64, i64* %L5
  %t69 = call i64 @__str_at(i64 %t67, i64 %t68)
  %t70 = add i64 0, 45
  %t71.c = icmp eq i64 %t69, %t70
  %t71 = zext i1 %t71.c to i64
  %t88.b = icmp ne i64 %t71, 0
  br i1 %t88.b, label %then88, label %else88
then88:
  %t72 = add i64 0, 0
  %t73 = add i64 0, 1
  %t74 = sub i64 %t72, %t73
  store i64 %t74, i64* %L8
  %t75 = load i64, i64* %L5
  %t76 = add i64 0, 1
  %t77 = add i64 %t75, %t76
  store i64 %t77, i64* %L5
  br label %thx88
thx88:
  br label %join88
else88:
  %t78 = load i64, i64* %L3
  %t79 = load i64, i64* %L5
  %t80 = call i64 @__str_at(i64 %t78, i64 %t79)
  %t81 = add i64 0, 43
  %t82.c = icmp eq i64 %t80, %t81
  %t82 = zext i1 %t82.c to i64
  %t87.b = icmp ne i64 %t82, 0
  br i1 %t87.b, label %then87, label %else87
then87:
  %t83 = load i64, i64* %L5
  %t84 = add i64 0, 1
  %t85 = add i64 %t83, %t84
  store i64 %t85, i64* %L5
  br label %thx87
thx87:
  br label %join87
else87:
  %t86 = add i64 0, 0
  br label %elx87
elx87:
  br label %join87
join87:
  %t87 = phi i64 [ %t85, %thx87 ], [ %t86, %elx87 ]
  br label %elx88
elx88:
  br label %join88
join88:
  %t88 = phi i64 [ %t77, %thx88 ], [ %t87, %elx88 ]
  %t89 = add i64 0, 0
  store i64 %t89, i64* %L9
  br label %lcond92
lcond92:
  %t90 = load i64, i64* %L5
  %t91 = load i64, i64* %L4
  %t92.c = icmp slt i64 %t90, %t91
  %t92 = zext i1 %t92.c to i64
  %lc92 = icmp ne i64 %t92, 0
  br i1 %lc92, label %lbody92, label %lend92
lbody92:
  %t93 = load i64, i64* %L9
  %t94 = add i64 0, 10
  %t95 = mul i64 %t93, %t94
  %t96 = load i64, i64* %L3
  %t97 = load i64, i64* %L5
  %t98 = call i64 @__str_at(i64 %t96, i64 %t97)
  %t99 = add i64 0, 48
  %t100 = sub i64 %t98, %t99
  %t101 = add i64 %t95, %t100
  store i64 %t101, i64* %L9
  %t102 = load i64, i64* %L5
  %t103 = add i64 0, 1
  %t104 = add i64 %t102, %t103
  store i64 %t104, i64* %L5
  br label %lcond92
lend92:
  %t105 = add i64 0, 0
  %t106 = load i64, i64* %L9
  %t107 = load i64, i64* %L8
  %t108 = mul i64 %t106, %t107
  store i64 %t108, i64* %L9
  %t109 = load i64, i64* %L9
  %t110 = load i64, i64* %L7
  %t111p = inttoptr i64 %t110 to i8*
  %t111 = call i64 @strlen(i8* %t111p)
  %t112 = add i64 0, 1
  %t113 = sub i64 %t111, %t112
  %t114 = sub i64 %t109, %t113
  store i64 %t114, i64* %L10
  %t115 = load i64, i64* %L7
  store i64 %t115, i64* %L11
  %t116 = load i64, i64* %L10
  %t117 = add i64 0, 0
  %t118.c = icmp sgt i64 %t116, %t117
  %t118 = zext i1 %t118.c to i64
  %t125.b = icmp ne i64 %t118, 0
  br i1 %t125.b, label %then125, label %else125
then125:
  %t119 = load i64, i64* %L11
  %t120 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t121 = load i64, i64* %L10
  %t122 = call i64 @fmt_rep(i64 %t120, i64 %t121)
  %t123 = call i64 @__str_cat(i64 %t119, i64 %t122)
  store i64 %t123, i64* %L11
  br label %thx125
thx125:
  br label %join125
else125:
  %t124 = add i64 0, 0
  br label %elx125
elx125:
  br label %join125
join125:
  %t125 = phi i64 [ %t123, %thx125 ], [ %t124, %elx125 ]
  %t126 = load i64, i64* %L6
  %t127 = add i64 0, 1
  %t128.c = icmp eq i64 %t126, %t127
  %t128 = zext i1 %t128.c to i64
  %t133.b = icmp ne i64 %t128, 0
  br i1 %t133.b, label %then133, label %else133
then133:
  %t129 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t130 = load i64, i64* %L11
  %t131 = call i64 @__str_cat(i64 %t129, i64 %t130)
  store i64 %t131, i64* %L11
  br label %thx133
thx133:
  br label %join133
else133:
  %t132 = add i64 0, 0
  br label %elx133
elx133:
  br label %join133
join133:
  %t133 = phi i64 [ %t131, %thx133 ], [ %t132, %elx133 ]
  %t134 = load i64, i64* %L11
  ret i64 %t134
}
define i64 @fmt_rep(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  %t8 = load i64, i64* %L0
  %t9 = call i64 @__str_cat(i64 %t7, i64 %t8)
  store i64 %t9, i64* %L2
  %t10 = load i64, i64* %L3
  %t11 = add i64 0, 1
  %t12 = add i64 %t10, %t11
  store i64 %t12, i64* %L3
  br label %lcond6
lend6:
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L2
  ret i64 %t14
}
define i64 @fmt_pad(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %L9 = alloca i64
  store i64 0, i64* %L9
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5p = inttoptr i64 %t4 to i8*
  %t5 = call i64 @strlen(i8* %t5p)
  %t6 = add i64 0, %t5
  store i64 %t6, i64* %L4
  %t7 = add i64 0, 0
  store i64 %t7, i64* %L5
  %t8 = add i64 0, 0
  store i64 %t8, i64* %L6
  br label %lcond11
lcond11:
  %t9 = load i64, i64* %L6
  %t10 = load i64, i64* %L4
  %t11.c = icmp slt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L0
  %t13 = load i64, i64* %L6
  %t14 = call i64 @__str_at(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L7
  %t15 = load i64, i64* %L7
  %t16 = add i64 0, 128
  %t17.c = icmp slt i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = add i64 0, 1
  br label %thx22
thx22:
  br label %join22
else22:
  %t19 = load i64, i64* %L7
  %t20 = add i64 0, 192
  %t21.c = icmp sge i64 %t19, %t20
  %t21 = zext i1 %t21.c to i64
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = phi i64 [ %t18, %thx22 ], [ %t21, %elx22 ]
  %t27.b = icmp ne i64 %t22, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t23 = load i64, i64* %L5
  %t24 = add i64 0, 1
  %t25 = add i64 %t23, %t24
  store i64 %t25, i64* %L5
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  %t28 = load i64, i64* %L6
  %t29 = add i64 0, 1
  %t30 = add i64 %t28, %t29
  store i64 %t30, i64* %L6
  br label %lcond11
lend11:
  %t31 = add i64 0, 0
  %t32 = load i64, i64* %L5
  %t33 = load i64, i64* %L1
  %t34.c = icmp sge i64 %t32, %t33
  %t34 = zext i1 %t34.c to i64
  %t38.b = icmp ne i64 %t34, 0
  br i1 %t38.b, label %then38, label %else38
then38:
  %t35 = load i64, i64* %L0
  store i64 %t35, i64* %L7
  %t36 = load i64, i64* %L7
  ret i64 %t36
ar36:
  br label %thx38
thx38:
  br label %join38
else38:
  %t37 = add i64 0, 0
  br label %elx38
elx38:
  br label %join38
join38:
  %t38 = add i64 0, %t37
  %t39 = load i64, i64* %L1
  %t40 = load i64, i64* %L5
  %t41 = sub i64 %t39, %t40
  store i64 %t41, i64* %L7
  %t42 = load i64, i64* %L2
  %t43 = add i64 0, 1
  %t44.c = icmp eq i64 %t42, %t43
  %t44 = zext i1 %t44.c to i64
  %t52.b = icmp ne i64 %t44, 0
  br i1 %t52.b, label %then52, label %else52
then52:
  %t45 = load i64, i64* %L0
  %t46 = load i64, i64* %L3
  %t47 = load i64, i64* %L7
  %t48 = call i64 @fmt_rep(i64 %t46, i64 %t47)
  %t49 = call i64 @__str_cat(i64 %t45, i64 %t48)
  store i64 %t49, i64* %L8
  %t50 = load i64, i64* %L8
  ret i64 %t50
ar50:
  br label %thx52
thx52:
  br label %join52
else52:
  %t51 = add i64 0, 0
  br label %elx52
elx52:
  br label %join52
join52:
  %t52 = add i64 0, %t51
  %t53 = load i64, i64* %L2
  %t54 = add i64 0, 3
  %t55.c = icmp eq i64 %t53, %t54
  %t55 = zext i1 %t55.c to i64
  %t72.b = icmp ne i64 %t55, 0
  br i1 %t72.b, label %then72, label %else72
then72:
  %t56 = load i64, i64* %L7
  %t57 = add i64 0, 2
  %t58d = call i64 @__divchk(i64 %t57)
  %t58 = sdiv i64 %t56, %t58d
  store i64 %t58, i64* %L8
  %t59 = load i64, i64* %L3
  %t60 = load i64, i64* %L8
  %t61 = call i64 @fmt_rep(i64 %t59, i64 %t60)
  %t62 = load i64, i64* %L0
  %t63 = call i64 @__str_cat(i64 %t61, i64 %t62)
  %t64 = load i64, i64* %L3
  %t65 = load i64, i64* %L7
  %t66 = load i64, i64* %L8
  %t67 = sub i64 %t65, %t66
  %t68 = call i64 @fmt_rep(i64 %t64, i64 %t67)
  %t69 = call i64 @__str_cat(i64 %t63, i64 %t68)
  store i64 %t69, i64* %L9
  %t70 = load i64, i64* %L9
  ret i64 %t70
ar70:
  br label %thx72
thx72:
  br label %join72
else72:
  %t71 = add i64 0, 0
  br label %elx72
elx72:
  br label %join72
join72:
  %t72 = add i64 0, %t71
  %t73 = load i64, i64* %L2
  %t74 = add i64 0, 4
  %t75.c = icmp eq i64 %t73, %t74
  %t75 = zext i1 %t75.c to i64
  %t102.b = icmp ne i64 %t75, 0
  br i1 %t102.b, label %then102, label %else102
then102:
  %t76 = load i64, i64* %L0
  %t77 = add i64 0, 0
  %t78 = add i64 0, 1
  %t79 = call i64 @__str_sub(i64 %t76, i64 %t77, i64 %t78)
  %t80 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t81 = call i64 @__streq(i64 %t79, i64 %t80)
  %t94.b = icmp ne i64 %t81, 0
  br i1 %t94.b, label %then94, label %else94
then94:
  %t82 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t83 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t84 = load i64, i64* %L7
  %t85 = call i64 @fmt_rep(i64 %t83, i64 %t84)
  %t86 = load i64, i64* %L0
  %t87 = add i64 0, 1
  %t88 = load i64, i64* %L4
  %t89 = call i64 @__str_sub(i64 %t86, i64 %t87, i64 %t88)
  %t90 = call i64 @__str_cat(i64 %t85, i64 %t89)
  %t91 = call i64 @__str_cat(i64 %t82, i64 %t90)
  store i64 %t91, i64* %L8
  %t92 = load i64, i64* %L8
  ret i64 %t92
ar92:
  br label %thx94
thx94:
  br label %join94
else94:
  %t93 = add i64 0, 0
  br label %elx94
elx94:
  br label %join94
join94:
  %t94 = add i64 0, %t93
  %t95 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t96 = load i64, i64* %L7
  %t97 = call i64 @fmt_rep(i64 %t95, i64 %t96)
  %t98 = load i64, i64* %L0
  %t99 = call i64 @__str_cat(i64 %t97, i64 %t98)
  store i64 %t99, i64* %L8
  %t100 = load i64, i64* %L8
  ret i64 %t100
ar100:
  br label %thx102
thx102:
  br label %join102
else102:
  %t101 = add i64 0, 0
  br label %elx102
elx102:
  br label %join102
join102:
  %t102 = add i64 0, %t101
  %t103 = load i64, i64* %L3
  %t104 = load i64, i64* %L7
  %t105 = call i64 @fmt_rep(i64 %t103, i64 %t104)
  %t106 = load i64, i64* %L0
  %t107 = call i64 @__str_cat(i64 %t105, i64 %t106)
  ret i64 %t107
}
define i64 @vstr_join(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L4
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t16.b = icmp ne i64 %t11, 0
  br i1 %t16.b, label %then16, label %else16
then16:
  %t12 = load i64, i64* %L3
  %t13 = load i64, i64* %L1
  %t14 = call i64 @__str_cat(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L3
  br label %thx16
thx16:
  br label %join16
else16:
  %t15 = add i64 0, 0
  br label %elx16
elx16:
  br label %join16
join16:
  %t16 = phi i64 [ %t14, %thx16 ], [ %t15, %elx16 ]
  %t17 = load i64, i64* %L3
  %t18 = load i64, i64* %L0
  %t19 = load i64, i64* %L4
  %t20 = call i64 @__vec_get(i64 %t18, i64 %t19)
  %t21 = call i64 @__str_cat(i64 %t17, i64 %t20)
  store i64 %t21, i64* %L3
  %t22 = load i64, i64* %L4
  %t23 = add i64 0, 1
  %t24 = add i64 %t22, %t23
  store i64 %t24, i64* %L4
  br label %lcond8
lend8:
  %t25 = add i64 0, 0
  %t26 = load i64, i64* %L3
  ret i64 %t26
}
define i64 @vstr_concat(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  %t3 = call i64 @vstr_join(i64 %t1, i64 %t2)
  ret i64 %t3
}
define i64 @HashMap$new() {
entry:
  %t0 = call i64 @__vec_new()
  %t1 = call i64 @__vec_new()
  %t2.mb = call i8* @malloc(i64 16)
  %t2.m = bitcast i8* %t2.mb to [2 x i64]*
  %t2.f0 = getelementptr [2 x i64], [2 x i64]* %t2.m, i64 0, i64 0
  store i64 %t0, i64* %t2.f0
  %t2.f1 = getelementptr [2 x i64], [2 x i64]* %t2.m, i64 0, i64 1
  store i64 %t1, i64* %t2.f1
  %t2.p = getelementptr [2 x i64], [2 x i64]* %t2.m, i64 0, i64 0
  %t2 = ptrtoint i64* %t2.p to i64   ; struct size=16
  ret i64 %t2
}
define i64 @hmi_find(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3.bp = inttoptr i64 %t2 to i8*
  %t3.fp8 = getelementptr i8, i8* %t3.bp, i64 0
  %t3.fp = bitcast i8* %t3.fp8 to i64*
  %t3 = load i64, i64* %t3.fp
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10.bp = inttoptr i64 %t9 to i8*
  %t10.fp8 = getelementptr i8, i8* %t10.bp, i64 0
  %t10.fp = bitcast i8* %t10.fp8 to i64*
  %t10 = load i64, i64* %t10.fp
  %t11 = load i64, i64* %L3
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13 = load i64, i64* %L1
  %t14.c = icmp eq i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t18.b = icmp ne i64 %t14, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t15 = load i64, i64* %L3
  store i64 %t15, i64* %L4
  %t16 = load i64, i64* %L4
  ret i64 %t16
ar16:
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = add i64 0, %t17
  %t19 = load i64, i64* %L3
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L3
  br label %lcond8
lend8:
  %t22 = add i64 0, 0
  %t23 = add i64 0, 0
  %t24 = add i64 0, 1
  %t25 = sub i64 %t23, %t24
  ret i64 %t25
}
define i64 @hms_find(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3.bp = inttoptr i64 %t2 to i8*
  %t3.fp8 = getelementptr i8, i8* %t3.bp, i64 0
  %t3.fp = bitcast i8* %t3.fp8 to i64*
  %t3 = load i64, i64* %t3.fp
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10.bp = inttoptr i64 %t9 to i8*
  %t10.fp8 = getelementptr i8, i8* %t10.bp, i64 0
  %t10.fp = bitcast i8* %t10.fp8 to i64*
  %t10 = load i64, i64* %t10.fp
  %t11 = load i64, i64* %L3
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13 = load i64, i64* %L1
  %t14 = call i64 @__streq(i64 %t12, i64 %t13)
  %t15 = add i64 0, 1
  %t16.c = icmp eq i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = load i64, i64* %L3
  store i64 %t17, i64* %L4
  %t18 = load i64, i64* %L4
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L3
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L3
  br label %lcond8
lend8:
  %t24 = add i64 0, 0
  %t25 = add i64 0, 0
  %t26 = add i64 0, 1
  %t27 = sub i64 %t25, %t26
  ret i64 %t27
}
define i64 @hmi_insert(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = load i64, i64* %L1
  %t5 = call i64 @hmi_find(i64 %t3, i64 %t4)
  store i64 %t5, i64* %L3
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 0
  %t8.c = icmp sge i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t24.b = icmp ne i64 %t8, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t9 = load i64, i64* %L0
  %t10.bp = inttoptr i64 %t9 to i8*
  %t10.fp8 = getelementptr i8, i8* %t10.bp, i64 8
  %t10.fp = bitcast i8* %t10.fp8 to i64*
  %t10 = load i64, i64* %t10.fp
  %t11 = load i64, i64* %L3
  %t12 = load i64, i64* %L2
  %t13 = call i64 @__vec_set(i64 %t10, i64 %t11, i64 %t12)
  %t14 = add i64 0, 0
  br label %thx24
thx24:
  br label %join24
else24:
  %t15 = load i64, i64* %L0
  %t16.bp = inttoptr i64 %t15 to i8*
  %t16.fp8 = getelementptr i8, i8* %t16.bp, i64 0
  %t16.fp = bitcast i8* %t16.fp8 to i64*
  %t16 = load i64, i64* %t16.fp
  %t17 = load i64, i64* %L1
  %t18 = call i64 @__vec_push(i64 %t16, i64 %t17)
  %t19 = load i64, i64* %L0
  %t20.bp = inttoptr i64 %t19 to i8*
  %t20.fp8 = getelementptr i8, i8* %t20.bp, i64 8
  %t20.fp = bitcast i8* %t20.fp8 to i64*
  %t20 = load i64, i64* %t20.fp
  %t21 = load i64, i64* %L2
  %t22 = call i64 @__vec_push(i64 %t20, i64 %t21)
  %t23 = add i64 0, 0
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t14, %thx24 ], [ %t23, %elx24 ]
  ret i64 %t24
}
define i64 @hms_insert(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = load i64, i64* %L1
  %t5 = call i64 @hms_find(i64 %t3, i64 %t4)
  store i64 %t5, i64* %L3
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 0
  %t8.c = icmp sge i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t24.b = icmp ne i64 %t8, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t9 = load i64, i64* %L0
  %t10.bp = inttoptr i64 %t9 to i8*
  %t10.fp8 = getelementptr i8, i8* %t10.bp, i64 8
  %t10.fp = bitcast i8* %t10.fp8 to i64*
  %t10 = load i64, i64* %t10.fp
  %t11 = load i64, i64* %L3
  %t12 = load i64, i64* %L2
  %t13 = call i64 @__vec_set(i64 %t10, i64 %t11, i64 %t12)
  %t14 = add i64 0, 0
  br label %thx24
thx24:
  br label %join24
else24:
  %t15 = load i64, i64* %L0
  %t16.bp = inttoptr i64 %t15 to i8*
  %t16.fp8 = getelementptr i8, i8* %t16.bp, i64 0
  %t16.fp = bitcast i8* %t16.fp8 to i64*
  %t16 = load i64, i64* %t16.fp
  %t17 = load i64, i64* %L1
  %t18 = call i64 @__vec_push(i64 %t16, i64 %t17)
  %t19 = load i64, i64* %L0
  %t20.bp = inttoptr i64 %t19 to i8*
  %t20.fp8 = getelementptr i8, i8* %t20.bp, i64 8
  %t20.fp = bitcast i8* %t20.fp8 to i64*
  %t20 = load i64, i64* %t20.fp
  %t21 = load i64, i64* %L2
  %t22 = call i64 @__vec_push(i64 %t20, i64 %t21)
  %t23 = add i64 0, 0
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t14, %thx24 ], [ %t23, %elx24 ]
  ret i64 %t24
}
define i64 @hmi_remove(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hmi_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 0
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t12.b = icmp ne i64 %t7, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t8 = add i64 0, 0
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 0, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %L3
  %t10 = load i64, i64* %L3
  ret i64 %t10
ar10:
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = add i64 0, %t11
  %t13 = load i64, i64* %L0
  %t14.bp = inttoptr i64 %t13 to i8*
  %t14.fp8 = getelementptr i8, i8* %t14.bp, i64 8
  %t14.fp = bitcast i8* %t14.fp8 to i64*
  %t14 = load i64, i64* %t14.fp
  %t15 = load i64, i64* %L2
  %t16 = call i64 @__vec_get(i64 %t14, i64 %t15)
  store i64 %t16, i64* %L3
  %t17 = load i64, i64* %L0
  %t18.bp = inttoptr i64 %t17 to i8*
  %t18.fp8 = getelementptr i8, i8* %t18.bp, i64 0
  %t18.fp = bitcast i8* %t18.fp8 to i64*
  %t18 = load i64, i64* %t18.fp
  %t19 = call i64 @__vec_len(i64 %t18)
  %t20 = add i64 0, 1
  %t21 = sub i64 %t19, %t20
  store i64 %t21, i64* %L4
  %t22 = load i64, i64* %L0
  %t23.bp = inttoptr i64 %t22 to i8*
  %t23.fp8 = getelementptr i8, i8* %t23.bp, i64 0
  %t23.fp = bitcast i8* %t23.fp8 to i64*
  %t23 = load i64, i64* %t23.fp
  %t24 = load i64, i64* %L2
  %t25 = load i64, i64* %L0
  %t26.bp = inttoptr i64 %t25 to i8*
  %t26.fp8 = getelementptr i8, i8* %t26.bp, i64 0
  %t26.fp = bitcast i8* %t26.fp8 to i64*
  %t26 = load i64, i64* %t26.fp
  %t27 = load i64, i64* %L4
  %t28 = call i64 @__vec_get(i64 %t26, i64 %t27)
  %t29 = call i64 @__vec_set(i64 %t23, i64 %t24, i64 %t28)
  %t30 = load i64, i64* %L0
  %t31.bp = inttoptr i64 %t30 to i8*
  %t31.fp8 = getelementptr i8, i8* %t31.bp, i64 8
  %t31.fp = bitcast i8* %t31.fp8 to i64*
  %t31 = load i64, i64* %t31.fp
  %t32 = load i64, i64* %L2
  %t33 = load i64, i64* %L0
  %t34.bp = inttoptr i64 %t33 to i8*
  %t34.fp8 = getelementptr i8, i8* %t34.bp, i64 8
  %t34.fp = bitcast i8* %t34.fp8 to i64*
  %t34 = load i64, i64* %t34.fp
  %t35 = load i64, i64* %L4
  %t36 = call i64 @__vec_get(i64 %t34, i64 %t35)
  %t37 = call i64 @__vec_set(i64 %t31, i64 %t32, i64 %t36)
  %t38 = load i64, i64* %L0
  %t39.bp = inttoptr i64 %t38 to i8*
  %t39.fp8 = getelementptr i8, i8* %t39.bp, i64 0
  %t39.fp = bitcast i8* %t39.fp8 to i64*
  %t39 = load i64, i64* %t39.fp
  %t40 = call i64 @vpop(i64 %t39)
  %t41 = load i64, i64* %L0
  %t42.bp = inttoptr i64 %t41 to i8*
  %t42.fp8 = getelementptr i8, i8* %t42.bp, i64 8
  %t42.fp = bitcast i8* %t42.fp8 to i64*
  %t42 = load i64, i64* %t42.fp
  %t43 = call i64 @vpop(i64 %t42)
  %t44 = load i64, i64* %L3
  %t45.mb = call i8* @malloc(i64 16)
  %t45.m = bitcast i8* %t45.mb to [2 x i64]*
  %t45.t = getelementptr [2 x i64], [2 x i64]* %t45.m, i64 0, i64 0
  store i64 1, i64* %t45.t
  %t45.p0 = getelementptr [2 x i64], [2 x i64]* %t45.m, i64 0, i64 1
  store i64 %t44, i64* %t45.p0
  %t45 = ptrtoint [2 x i64]* %t45.m to i64
  ret i64 %t45
}
define i64 @hms_remove(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hms_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 0
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t12.b = icmp ne i64 %t7, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t8 = add i64 0, 0
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 0, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %L3
  %t10 = load i64, i64* %L3
  ret i64 %t10
ar10:
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = add i64 0, %t11
  %t13 = load i64, i64* %L0
  %t14.bp = inttoptr i64 %t13 to i8*
  %t14.fp8 = getelementptr i8, i8* %t14.bp, i64 8
  %t14.fp = bitcast i8* %t14.fp8 to i64*
  %t14 = load i64, i64* %t14.fp
  %t15 = load i64, i64* %L2
  %t16 = call i64 @__vec_get(i64 %t14, i64 %t15)
  store i64 %t16, i64* %L3
  %t17 = load i64, i64* %L0
  %t18.bp = inttoptr i64 %t17 to i8*
  %t18.fp8 = getelementptr i8, i8* %t18.bp, i64 0
  %t18.fp = bitcast i8* %t18.fp8 to i64*
  %t18 = load i64, i64* %t18.fp
  %t19 = call i64 @__vec_len(i64 %t18)
  %t20 = add i64 0, 1
  %t21 = sub i64 %t19, %t20
  store i64 %t21, i64* %L4
  %t22 = load i64, i64* %L0
  %t23.bp = inttoptr i64 %t22 to i8*
  %t23.fp8 = getelementptr i8, i8* %t23.bp, i64 0
  %t23.fp = bitcast i8* %t23.fp8 to i64*
  %t23 = load i64, i64* %t23.fp
  %t24 = load i64, i64* %L2
  %t25 = load i64, i64* %L0
  %t26.bp = inttoptr i64 %t25 to i8*
  %t26.fp8 = getelementptr i8, i8* %t26.bp, i64 0
  %t26.fp = bitcast i8* %t26.fp8 to i64*
  %t26 = load i64, i64* %t26.fp
  %t27 = load i64, i64* %L4
  %t28 = call i64 @__vec_get(i64 %t26, i64 %t27)
  %t29 = call i64 @__vec_set(i64 %t23, i64 %t24, i64 %t28)
  %t30 = load i64, i64* %L0
  %t31.bp = inttoptr i64 %t30 to i8*
  %t31.fp8 = getelementptr i8, i8* %t31.bp, i64 8
  %t31.fp = bitcast i8* %t31.fp8 to i64*
  %t31 = load i64, i64* %t31.fp
  %t32 = load i64, i64* %L2
  %t33 = load i64, i64* %L0
  %t34.bp = inttoptr i64 %t33 to i8*
  %t34.fp8 = getelementptr i8, i8* %t34.bp, i64 8
  %t34.fp = bitcast i8* %t34.fp8 to i64*
  %t34 = load i64, i64* %t34.fp
  %t35 = load i64, i64* %L4
  %t36 = call i64 @__vec_get(i64 %t34, i64 %t35)
  %t37 = call i64 @__vec_set(i64 %t31, i64 %t32, i64 %t36)
  %t38 = load i64, i64* %L0
  %t39.bp = inttoptr i64 %t38 to i8*
  %t39.fp8 = getelementptr i8, i8* %t39.bp, i64 0
  %t39.fp = bitcast i8* %t39.fp8 to i64*
  %t39 = load i64, i64* %t39.fp
  %t40 = call i64 @vpop(i64 %t39)
  %t41 = load i64, i64* %L0
  %t42.bp = inttoptr i64 %t41 to i8*
  %t42.fp8 = getelementptr i8, i8* %t42.bp, i64 8
  %t42.fp = bitcast i8* %t42.fp8 to i64*
  %t42 = load i64, i64* %t42.fp
  %t43 = call i64 @vpop(i64 %t42)
  %t44 = load i64, i64* %L3
  %t45.mb = call i8* @malloc(i64 16)
  %t45.m = bitcast i8* %t45.mb to [2 x i64]*
  %t45.t = getelementptr [2 x i64], [2 x i64]* %t45.m, i64 0, i64 0
  store i64 1, i64* %t45.t
  %t45.p0 = getelementptr [2 x i64], [2 x i64]* %t45.m, i64 0, i64 1
  store i64 %t44, i64* %t45.p0
  %t45 = ptrtoint [2 x i64]* %t45.m to i64
  ret i64 %t45
}
define i64 @hmi_get(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hmi_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 0
  %t7.c = icmp sge i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t15.b = icmp ne i64 %t7, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t8 = load i64, i64* %L0
  %t9.bp = inttoptr i64 %t8 to i8*
  %t9.fp8 = getelementptr i8, i8* %t9.bp, i64 8
  %t9.fp = bitcast i8* %t9.fp8 to i64*
  %t9 = load i64, i64* %t9.fp
  %t10 = load i64, i64* %L2
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12.mb = call i8* @malloc(i64 16)
  %t12.m = bitcast i8* %t12.mb to [2 x i64]*
  %t12.t = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 0
  store i64 1, i64* %t12.t
  %t12.p0 = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 1
  store i64 %t11, i64* %t12.p0
  %t12 = ptrtoint [2 x i64]* %t12.m to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t13 = add i64 0, 0
  %t14.mb = call i8* @malloc(i64 16)
  %t14.m = bitcast i8* %t14.mb to [2 x i64]*
  %t14.t = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 0
  store i64 0, i64* %t14.t
  %t14.p0 = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 1
  store i64 %t13, i64* %t14.p0
  %t14 = ptrtoint [2 x i64]* %t14.m to i64
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t12, %thx15 ], [ %t14, %elx15 ]
  ret i64 %t15
}
define i64 @hms_get(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hms_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 0
  %t7.c = icmp sge i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t15.b = icmp ne i64 %t7, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t8 = load i64, i64* %L0
  %t9.bp = inttoptr i64 %t8 to i8*
  %t9.fp8 = getelementptr i8, i8* %t9.bp, i64 8
  %t9.fp = bitcast i8* %t9.fp8 to i64*
  %t9 = load i64, i64* %t9.fp
  %t10 = load i64, i64* %L2
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12.mb = call i8* @malloc(i64 16)
  %t12.m = bitcast i8* %t12.mb to [2 x i64]*
  %t12.t = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 0
  store i64 1, i64* %t12.t
  %t12.p0 = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 1
  store i64 %t11, i64* %t12.p0
  %t12 = ptrtoint [2 x i64]* %t12.m to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t13 = add i64 0, 0
  %t14.mb = call i8* @malloc(i64 16)
  %t14.m = bitcast i8* %t14.mb to [2 x i64]*
  %t14.t = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 0
  store i64 0, i64* %t14.t
  %t14.p0 = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 1
  store i64 %t13, i64* %t14.p0
  %t14 = ptrtoint [2 x i64]* %t14.m to i64
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t12, %thx15 ], [ %t14, %elx15 ]
  ret i64 %t15
}
define i64 @hmi_contains(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hmi_find(i64 %t2, i64 %t3)
  %t5 = add i64 0, 0
  %t6.c = icmp sge i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  ret i64 %t6
}
define i64 @hms_contains(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hms_find(i64 %t2, i64 %t3)
  %t5 = add i64 0, 0
  %t6.c = icmp sge i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  ret i64 %t6
}
define i64 @hmi_idx(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hmi_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L0
  %t6.bp = inttoptr i64 %t5 to i8*
  %t6.fp8 = getelementptr i8, i8* %t6.bp, i64 8
  %t6.fp = bitcast i8* %t6.fp8 to i64*
  %t6 = load i64, i64* %t6.fp
  %t7 = load i64, i64* %L2
  %t8 = call i64 @__vec_get(i64 %t6, i64 %t7)
  ret i64 %t8
}
define i64 @hms_idx(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @hms_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L0
  %t6.bp = inttoptr i64 %t5 to i8*
  %t6.fp8 = getelementptr i8, i8* %t6.bp, i64 8
  %t6.fp = bitcast i8* %t6.fp8 to i64*
  %t6 = load i64, i64* %t6.fp
  %t7 = load i64, i64* %L2
  %t8 = call i64 @__vec_get(i64 %t6, i64 %t7)
  ret i64 %t8
}
define i64 @hm_len(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2.bp = inttoptr i64 %t1 to i8*
  %t2.fp8 = getelementptr i8, i8* %t2.bp, i64 0
  %t2.fp = bitcast i8* %t2.fp8 to i64*
  %t2 = load i64, i64* %t2.fp
  %t3 = call i64 @__vec_len(i64 %t2)
  ret i64 %t3
}
define i64 @hm_empty(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2.bp = inttoptr i64 %t1 to i8*
  %t2.fp8 = getelementptr i8, i8* %t2.bp, i64 0
  %t2.fp = bitcast i8* %t2.fp8 to i64*
  %t2 = load i64, i64* %t2.fp
  %t3 = call i64 @__vec_len(i64 %t2)
  %t4 = add i64 0, 0
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  ret i64 %t5
}
define i64 @hm_keys(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3.bp = inttoptr i64 %t2 to i8*
  %t3.fp8 = getelementptr i8, i8* %t3.bp, i64 0
  %t3.fp = bitcast i8* %t3.fp8 to i64*
  %t3 = load i64, i64* %t3.fp
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L0
  %t11.bp = inttoptr i64 %t10 to i8*
  %t11.fp8 = getelementptr i8, i8* %t11.bp, i64 0
  %t11.fp = bitcast i8* %t11.fp8 to i64*
  %t11 = load i64, i64* %t11.fp
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  %t14 = call i64 @__vec_push(i64 %t9, i64 %t13)
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  br label %lcond8
lend8:
  %t18 = add i64 0, 0
  %t19 = load i64, i64* %L1
  ret i64 %t19
}
define i64 @Ordering$discr(i64 %p0) {
entry:
  %t2.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2.bp = inttoptr i64 %t1 to i64*
  %t2.tg = load i64, i64* %t2.bp
  br label %m2a0
m2a0:
  %t2.c0 = icmp eq i64 %t2.tg, 0
  br i1 %t2.c0, label %m2a0b, label %m2a1
m2a0b:
  %t3 = add i64 0, 0
  store i64 %t3, i64* %t2.slot
  br label %m2end
m2a1:
  %t2.c1 = icmp eq i64 %t2.tg, 1
  br i1 %t2.c1, label %m2a1b, label %m2a2
m2a1b:
  %t4 = add i64 0, 1
  store i64 %t4, i64* %t2.slot
  br label %m2end
m2a2:
  %t2.c2 = icmp eq i64 %t2.tg, 2
  br i1 %t2.c2, label %m2a2b, label %m2dflt
m2a2b:
  %t5 = add i64 0, 2
  store i64 %t5, i64* %t2.slot
  br label %m2end
m2dflt:
  store i64 0, i64* %t2.slot
  br label %m2end
m2end:
  %t2 = load i64, i64* %t2.slot
  ret i64 %t2
}
define i64 @int_cmp(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4.c = icmp slt i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t12.b = icmp ne i64 %t4, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t5.mb = call i8* @malloc(i64 8)
  %t5.m = bitcast i8* %t5.mb to [1 x i64]*
  %t5.t = getelementptr [1 x i64], [1 x i64]* %t5.m, i64 0, i64 0
  store i64 0, i64* %t5.t
  %t5 = ptrtoint [1 x i64]* %t5.m to i64
  br label %thx12
thx12:
  br label %join12
else12:
  %t6 = load i64, i64* %L0
  %t7 = load i64, i64* %L1
  %t8.c = icmp sgt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t11.b = icmp ne i64 %t8, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t9.mb = call i8* @malloc(i64 8)
  %t9.m = bitcast i8* %t9.mb to [1 x i64]*
  %t9.t = getelementptr [1 x i64], [1 x i64]* %t9.m, i64 0, i64 0
  store i64 2, i64* %t9.t
  %t9 = ptrtoint [1 x i64]* %t9.m to i64
  br label %thx11
thx11:
  br label %join11
else11:
  %t10.mb = call i8* @malloc(i64 8)
  %t10.m = bitcast i8* %t10.mb to [1 x i64]*
  %t10.t = getelementptr [1 x i64], [1 x i64]* %t10.m, i64 0, i64 0
  store i64 1, i64* %t10.t
  %t10 = ptrtoint [1 x i64]* %t10.m to i64
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = phi i64 [ %t9, %thx11 ], [ %t10, %elx11 ]
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = phi i64 [ %t5, %thx12 ], [ %t11, %elx12 ]
  ret i64 %t12
}
define i64 @ord_is_gt(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 2
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t4 = add i64 0, 1
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  br label %m3a1b
m3a1b:
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @vsort_by(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 1
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L3
  store i64 %t11, i64* %L5
  br label %lcond24
lcond24:
  %t12 = load i64, i64* %L5
  %t13 = add i64 0, 0
  %t14.c = icmp sgt i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t24.b = icmp ne i64 %t14, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L5
  %t17 = add i64 0, 1
  %t18 = sub i64 %t16, %t17
  %t19 = call i64 @__vec_get(i64 %t15, i64 %t18)
  store i64 %t19, i64* %L6
  %t20 = load i64, i64* %L4
  store i64 %t20, i64* %L7
  %t21 = add i64 0, 0
  %t22 = call i64 @ord_is_gt(i64 %t21)
  br label %thx24
thx24:
  br label %join24
else24:
  %t23 = add i64 0, 0
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t22, %thx24 ], [ %t23, %elx24 ]
  %lc24 = icmp ne i64 %t24, 0
  br i1 %lc24, label %lbody24, label %lend24
lbody24:
  %t25 = load i64, i64* %L0
  %t26 = load i64, i64* %L5
  %t27 = load i64, i64* %L0
  %t28 = load i64, i64* %L5
  %t29 = add i64 0, 1
  %t30 = sub i64 %t28, %t29
  %t31 = call i64 @__vec_get(i64 %t27, i64 %t30)
  %t32 = call i64 @__vec_set(i64 %t25, i64 %t26, i64 %t31)
  %t33 = load i64, i64* %L5
  %t34 = add i64 0, 1
  %t35 = sub i64 %t33, %t34
  store i64 %t35, i64* %L5
  br label %lcond24
lend24:
  %t36 = add i64 0, 0
  %t37 = load i64, i64* %L0
  %t38 = load i64, i64* %L5
  %t39 = load i64, i64* %L4
  %t40 = call i64 @__vec_set(i64 %t37, i64 %t38, i64 %t39)
  %t41 = load i64, i64* %L3
  %t42 = add i64 0, 1
  %t43 = add i64 %t41, %t42
  store i64 %t43, i64* %L3
  br label %lcond7
lend7:
  %t44 = add i64 0, 0
  %t45 = add i64 0, 0
  ret i64 %t45
}
define i64 @vmax(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3 = call i64 @__vec_get(i64 %t1, i64 %t2)
  store i64 %t3, i64* %L1
  %t4 = load i64, i64* %L0
  store i64 %t4, i64* %L2
  %t5 = add i64 0, -1
  store i64 %t5, i64* %L3
  br label %lcond12
lcond12:
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 1
  %t8 = add i64 %t6, %t7
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  %t10 = load i64, i64* %L2
  %t11 = call i64 @__vec_len(i64 %t10)
  %t12.c = icmp slt i64 %t9, %t11
  %t12 = zext i1 %t12.c to i64
  %lc12 = icmp ne i64 %t12, 0
  br i1 %lc12, label %lbody12, label %lend12
lbody12:
  %t13 = load i64, i64* %L2
  %t14 = load i64, i64* %L3
  %t15 = call i64 @__vec_get(i64 %t13, i64 %t14)
  store i64 %t15, i64* %L4
  %t16 = load i64, i64* %L1
  %t17 = load i64, i64* %L4
  %t18.c = icmp slt i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %t21.b = icmp ne i64 %t18, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t19 = load i64, i64* %L4
  store i64 %t19, i64* %L1
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  br label %lcond12
lend12:
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L1
  ret i64 %t23
}
define i64 @range(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  %t4 = add i64 0, 1
  %t5 = sub i64 %t3, %t4
  store i64 %t5, i64* %L3
  br label %lcond11
lcond11:
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 1
  %t8 = add i64 %t6, %t7
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  %t10 = load i64, i64* %L2
  %t11.c = icmp slt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L3
  store i64 %t12, i64* %L4
  %t13 = load i64, i64* %L1
  %t14 = load i64, i64* %L4
  %t15 = call i64 @__vec_push(i64 %t13, i64 %t14)
  br label %lcond11
lend11:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @str_from_int(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3.c = icmp eq i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t7.b = icmp ne i64 %t3, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t4 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  store i64 %t4, i64* %L1
  %t5 = load i64, i64* %L1
  ret i64 %t5
ar5:
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = add i64 0, 0
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = add i64 0, %t6
  %t8 = load i64, i64* %L0
  store i64 %t8, i64* %L1
  %t9 = add i64 0, 0
  store i64 %t9, i64* %L2
  %t10 = load i64, i64* %L1
  %t11 = add i64 0, 0
  %t12.c = icmp slt i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t18.b = icmp ne i64 %t12, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t13 = add i64 0, 1
  store i64 %t13, i64* %L2
  %t14 = add i64 0, 0
  %t15 = load i64, i64* %L1
  %t16 = sub i64 %t14, %t15
  store i64 %t16, i64* %L1
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t16, %thx18 ], [ %t17, %elx18 ]
  %t19 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t19, i64* %L3
  br label %lcond22
lcond22:
  %t20 = load i64, i64* %L1
  %t21 = add i64 0, 0
  %t22.c = icmp sgt i64 %t20, %t21
  %t22 = zext i1 %t22.c to i64
  %lc22 = icmp ne i64 %t22, 0
  br i1 %lc22, label %lbody22, label %lend22
lbody22:
  %t23 = load i64, i64* %L1
  %t24 = load i64, i64* %L1
  %t25 = add i64 0, 10
  %t26d = call i64 @__divchk(i64 %t25)
  %t26 = sdiv i64 %t24, %t26d
  %t27 = add i64 0, 10
  %t28 = mul i64 %t26, %t27
  %t29 = sub i64 %t23, %t28
  store i64 %t29, i64* %L4
  %t30 = ptrtoint [11 x i8]* @.s3bb13c327e0e3112 to i64
  %t31 = load i64, i64* %L4
  %t32 = add i64 0, 1
  %t33 = call i64 @__str_sub(i64 %t30, i64 %t31, i64 %t32)
  %t34 = load i64, i64* %L3
  %t35 = call i64 @__str_cat(i64 %t33, i64 %t34)
  store i64 %t35, i64* %L3
  %t36 = load i64, i64* %L1
  %t37 = add i64 0, 10
  %t38d = call i64 @__divchk(i64 %t37)
  %t38 = sdiv i64 %t36, %t38d
  store i64 %t38, i64* %L1
  br label %lcond22
lend22:
  %t39 = add i64 0, 0
  %t40 = load i64, i64* %L2
  %t41 = add i64 0, 1
  %t42.c = icmp eq i64 %t40, %t41
  %t42 = zext i1 %t42.c to i64
  %t47.b = icmp ne i64 %t42, 0
  br i1 %t47.b, label %then47, label %else47
then47:
  %t43 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t44 = load i64, i64* %L3
  %t45 = call i64 @__str_cat(i64 %t43, i64 %t44)
  store i64 %t45, i64* %L3
  br label %thx47
thx47:
  br label %join47
else47:
  %t46 = add i64 0, 0
  br label %elx47
elx47:
  br label %join47
join47:
  %t47 = phi i64 [ %t45, %thx47 ], [ %t46, %elx47 ]
  %t48 = load i64, i64* %L3
  ret i64 %t48
}
define i64 @str_split(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L1
  %t4p = inttoptr i64 %t3 to i8*
  %t4 = call i64 @strlen(i8* %t4p)
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L3
  %t6 = add i64 0, 0
  %t7.c = icmp eq i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t14.b = icmp ne i64 %t7, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t8 = load i64, i64* %L2
  %t9 = load i64, i64* %L0
  %t10 = call i64 @__vec_push(i64 %t8, i64 %t9)
  %t11 = load i64, i64* %L2
  store i64 %t11, i64* %L4
  %t12 = load i64, i64* %L4
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L0
  store i64 %t15, i64* %L4
  %t16 = add i64 0, 1
  store i64 %t16, i64* %L5
  br label %lcond19
lcond19:
  %t17 = load i64, i64* %L5
  %t18 = add i64 0, 1
  %t19.c = icmp eq i64 %t17, %t18
  %t19 = zext i1 %t19.c to i64
  %lc19 = icmp ne i64 %t19, 0
  br i1 %lc19, label %lbody19, label %lend19
lbody19:
  %t20 = load i64, i64* %L4
  %t21 = load i64, i64* %L1
  %t22 = call i64 @str_find(i64 %t20, i64 %t21)
  store i64 %t22, i64* %L6
  %t23 = load i64, i64* %L6
  %t24 = add i64 0, 0
  %t25.c = icmp slt i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  %t47.b = icmp ne i64 %t25, 0
  br i1 %t47.b, label %then47, label %else47
then47:
  %t26 = load i64, i64* %L2
  %t27 = load i64, i64* %L4
  %t28 = call i64 @__vec_push(i64 %t26, i64 %t27)
  %t29 = add i64 0, 0
  store i64 %t29, i64* %L5
  br label %thx47
thx47:
  br label %join47
else47:
  %t30 = load i64, i64* %L2
  %t31 = load i64, i64* %L4
  %t32 = add i64 0, 0
  %t33 = load i64, i64* %L6
  %t34 = call i64 @__str_sub(i64 %t31, i64 %t32, i64 %t33)
  %t35 = call i64 @__vec_push(i64 %t30, i64 %t34)
  %t36 = load i64, i64* %L4
  %t37 = load i64, i64* %L6
  %t38 = load i64, i64* %L3
  %t39 = add i64 %t37, %t38
  %t40 = load i64, i64* %L4
  %t41p = inttoptr i64 %t40 to i8*
  %t41 = call i64 @strlen(i8* %t41p)
  %t42 = load i64, i64* %L6
  %t43 = sub i64 %t41, %t42
  %t44 = load i64, i64* %L3
  %t45 = sub i64 %t43, %t44
  %t46 = call i64 @__str_sub(i64 %t36, i64 %t39, i64 %t45)
  store i64 %t46, i64* %L4
  br label %elx47
elx47:
  br label %join47
join47:
  %t47 = phi i64 [ %t29, %thx47 ], [ %t46, %elx47 ]
  br label %lcond19
lend19:
  %t48 = add i64 0, 0
  %t49 = load i64, i64* %L2
  ret i64 %t49
}
define i64 @str_find(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L1
  %t5p = inttoptr i64 %t4 to i8*
  %t5 = call i64 @strlen(i8* %t5p)
  store i64 %t5, i64* %L3
  %t6 = add i64 0, 0
  store i64 %t6, i64* %L4
  br label %lcond11
lcond11:
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L3
  %t9 = add i64 %t7, %t8
  %t10 = load i64, i64* %L2
  %t11.c = icmp sle i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L0
  %t13 = load i64, i64* %L4
  %t14 = load i64, i64* %L3
  %t15 = call i64 @__str_sub(i64 %t12, i64 %t13, i64 %t14)
  %t16 = load i64, i64* %L1
  %t17 = call i64 @__streq(i64 %t15, i64 %t16)
  %t21.b = icmp ne i64 %t17, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t18 = load i64, i64* %L4
  store i64 %t18, i64* %L5
  %t19 = load i64, i64* %L5
  ret i64 %t19
ar19:
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = add i64 0, %t20
  %t22 = load i64, i64* %L4
  %t23 = add i64 0, 1
  %t24 = add i64 %t22, %t23
  store i64 %t24, i64* %L4
  br label %lcond11
lend11:
  %t25 = add i64 0, 0
  %t26 = add i64 0, 0
  %t27 = add i64 0, 1
  %t28 = sub i64 %t26, %t27
  ret i64 %t28
}
define i64 @str_has(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @str_find(i64 %t2, i64 %t3)
  %t5 = add i64 0, 0
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t9.b = icmp ne i64 %t6, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t7 = add i64 0, 0
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 1
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = phi i64 [ %t7, %thx9 ], [ %t8, %elx9 ]
  ret i64 %t9
}
define i64 @say(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @str_from_int(i64 %t3)
  %t5 = call i64 @__str_cat(i64 %t2, i64 %t4)
  %t6p = inttoptr i64 %t5 to i8*
  %t6c = call i32 @puts(i8* %t6p)
  %t6 = sext i32 %t6c to i64
  ret i64 %t6
}
define i64 @dbghex(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [17 x i8]* @.sc4f6992bd7eb6267 to i64
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = add i64 0, 16
  %t4.c = icmp slt i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t11.b = icmp ne i64 %t4, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t5 = load i64, i64* %L1
  %t6 = load i64, i64* %L0
  %t7 = add i64 0, 1
  %t8 = call i64 @__str_sub(i64 %t5, i64 %t6, i64 %t7)
  store i64 %t8, i64* %L2
  %t9 = load i64, i64* %L2
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = load i64, i64* %L1
  %t13 = load i64, i64* %L0
  %t14 = add i64 0, 16
  %t15d = call i64 @__divchk(i64 %t14)
  %t15 = sdiv i64 %t13, %t15d
  %t16 = add i64 0, 1
  %t17 = call i64 @__str_sub(i64 %t12, i64 %t15, i64 %t16)
  %t18 = load i64, i64* %L1
  %t19 = load i64, i64* %L0
  %t20 = load i64, i64* %L0
  %t21 = add i64 0, 16
  %t22d = call i64 @__divchk(i64 %t21)
  %t22 = sdiv i64 %t20, %t22d
  %t23 = add i64 0, 10
  %t24 = mul i64 %t22, %t23
  %t25 = sub i64 %t19, %t24
  %t26 = load i64, i64* %L0
  %t27 = add i64 0, 16
  %t28d = call i64 @__divchk(i64 %t27)
  %t28 = sdiv i64 %t26, %t28d
  %t29 = add i64 0, 6
  %t30 = mul i64 %t28, %t29
  %t31 = sub i64 %t25, %t30
  %t32 = add i64 0, 1
  %t33 = call i64 @__str_sub(i64 %t18, i64 %t31, i64 %t32)
  %t34 = call i64 @__str_cat(i64 %t17, i64 %t33)
  ret i64 %t34
}
define i64 @dbgq(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, 34
  %t2w = trunc i64 %t1 to i32
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t2w)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4p = inttoptr i64 %t3 to i8*
  %t4 = call i64 @strlen(i8* %t4p)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L2
  %t11 = call i64 @__str_at(i64 %t9, i64 %t10)
  store i64 %t11, i64* %L3
  %t12 = load i64, i64* %L3
  %t13 = add i64 0, 34
  %t14.c = icmp eq i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t71.b = icmp ne i64 %t14, 0
  br i1 %t71.b, label %then71, label %else71
then71:
  %t15 = add i64 0, 92
  %t16w = trunc i64 %t15 to i32
  %t16c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t16w)
  %t16 = sext i32 %t16c to i64
  %t17 = add i64 0, 34
  %t18w = trunc i64 %t17 to i32
  %t18c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t18w)
  %t18 = sext i32 %t18c to i64
  br label %thx71
thx71:
  br label %join71
else71:
  %t19 = load i64, i64* %L3
  %t20 = add i64 0, 92
  %t21.c = icmp eq i64 %t19, %t20
  %t21 = zext i1 %t21.c to i64
  %t70.b = icmp ne i64 %t21, 0
  br i1 %t70.b, label %then70, label %else70
then70:
  %t22 = add i64 0, 92
  %t23w = trunc i64 %t22 to i32
  %t23c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t23w)
  %t23 = sext i32 %t23c to i64
  %t24 = add i64 0, 92
  %t25w = trunc i64 %t24 to i32
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t25w)
  %t25 = sext i32 %t25c to i64
  br label %thx70
thx70:
  br label %join70
else70:
  %t26 = load i64, i64* %L3
  %t27 = add i64 0, 10
  %t28.c = icmp eq i64 %t26, %t27
  %t28 = zext i1 %t28.c to i64
  %t69.b = icmp ne i64 %t28, 0
  br i1 %t69.b, label %then69, label %else69
then69:
  %t29 = add i64 0, 92
  %t30w = trunc i64 %t29 to i32
  %t30c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t30w)
  %t30 = sext i32 %t30c to i64
  %t31 = add i64 0, 110
  %t32w = trunc i64 %t31 to i32
  %t32c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t32w)
  %t32 = sext i32 %t32c to i64
  br label %thx69
thx69:
  br label %join69
else69:
  %t33 = load i64, i64* %L3
  %t34 = add i64 0, 9
  %t35.c = icmp eq i64 %t33, %t34
  %t35 = zext i1 %t35.c to i64
  %t68.b = icmp ne i64 %t35, 0
  br i1 %t68.b, label %then68, label %else68
then68:
  %t36 = add i64 0, 92
  %t37w = trunc i64 %t36 to i32
  %t37c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t37w)
  %t37 = sext i32 %t37c to i64
  %t38 = add i64 0, 116
  %t39w = trunc i64 %t38 to i32
  %t39c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t39w)
  %t39 = sext i32 %t39c to i64
  br label %thx68
thx68:
  br label %join68
else68:
  %t40 = load i64, i64* %L3
  %t41 = add i64 0, 13
  %t42.c = icmp eq i64 %t40, %t41
  %t42 = zext i1 %t42.c to i64
  %t67.b = icmp ne i64 %t42, 0
  br i1 %t67.b, label %then67, label %else67
then67:
  %t43 = add i64 0, 92
  %t44w = trunc i64 %t43 to i32
  %t44c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t44w)
  %t44 = sext i32 %t44c to i64
  %t45 = add i64 0, 114
  %t46w = trunc i64 %t45 to i32
  %t46c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t46w)
  %t46 = sext i32 %t46c to i64
  br label %thx67
thx67:
  br label %join67
else67:
  %t47 = load i64, i64* %L3
  %t48 = add i64 0, 32
  %t49.c = icmp slt i64 %t47, %t48
  %t49 = zext i1 %t49.c to i64
  %t66.b = icmp ne i64 %t49, 0
  br i1 %t66.b, label %then66, label %else66
then66:
  %t50 = add i64 0, 92
  %t51w = trunc i64 %t50 to i32
  %t51c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t51w)
  %t51 = sext i32 %t51c to i64
  %t52 = add i64 0, 117
  %t53w = trunc i64 %t52 to i32
  %t53c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t53w)
  %t53 = sext i32 %t53c to i64
  %t54 = add i64 0, 123
  %t55w = trunc i64 %t54 to i32
  %t55c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t55w)
  %t55 = sext i32 %t55c to i64
  %t56 = load i64, i64* %L3
  %t57 = call i64 @dbghex(i64 %t56)
  %t58p = inttoptr i64 %t57 to i8*
  %t58c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t58p)
  %t58 = sext i32 %t58c to i64
  %t59 = add i64 0, 125
  %t60w = trunc i64 %t59 to i32
  %t60c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t60w)
  %t60 = sext i32 %t60c to i64
  br label %thx66
thx66:
  br label %join66
else66:
  %t61 = load i64, i64* %L0
  %t62 = load i64, i64* %L2
  %t63 = add i64 0, 1
  %t64 = call i64 @__str_sub(i64 %t61, i64 %t62, i64 %t63)
  %t65p = inttoptr i64 %t64 to i8*
  %t65c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t65p)
  %t65 = sext i32 %t65c to i64
  br label %elx66
elx66:
  br label %join66
join66:
  %t66 = phi i64 [ %t60, %thx66 ], [ %t65, %elx66 ]
  br label %elx67
elx67:
  br label %join67
join67:
  %t67 = phi i64 [ %t46, %thx67 ], [ %t66, %elx67 ]
  br label %elx68
elx68:
  br label %join68
join68:
  %t68 = phi i64 [ %t39, %thx68 ], [ %t67, %elx68 ]
  br label %elx69
elx69:
  br label %join69
join69:
  %t69 = phi i64 [ %t32, %thx69 ], [ %t68, %elx69 ]
  br label %elx70
elx70:
  br label %join70
join70:
  %t70 = phi i64 [ %t25, %thx70 ], [ %t69, %elx70 ]
  br label %elx71
elx71:
  br label %join71
join71:
  %t71 = phi i64 [ %t18, %thx71 ], [ %t70, %elx71 ]
  %t72 = load i64, i64* %L2
  %t73 = add i64 0, 1
  %t74 = add i64 %t72, %t73
  store i64 %t74, i64* %L2
  br label %lcond8
lend8:
  %t75 = add i64 0, 0
  %t76 = add i64 0, 34
  %t77w = trunc i64 %t76 to i32
  %t77c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t77w)
  %t77 = sext i32 %t77c to i64
  ret i64 %t77
}
define i64 @dbgc(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, 39
  %t2w = trunc i64 %t1 to i32
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t2w)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = add i64 0, 39
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t67.b = icmp ne i64 %t5, 0
  br i1 %t67.b, label %then67, label %else67
then67:
  %t6 = add i64 0, 92
  %t7w = trunc i64 %t6 to i32
  %t7c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t7w)
  %t7 = sext i32 %t7c to i64
  %t8 = add i64 0, 39
  %t9w = trunc i64 %t8 to i32
  %t9c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t9w)
  %t9 = sext i32 %t9c to i64
  br label %thx67
thx67:
  br label %join67
else67:
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 92
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t66.b = icmp ne i64 %t12, 0
  br i1 %t66.b, label %then66, label %else66
then66:
  %t13 = add i64 0, 92
  %t14w = trunc i64 %t13 to i32
  %t14c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t14w)
  %t14 = sext i32 %t14c to i64
  %t15 = add i64 0, 92
  %t16w = trunc i64 %t15 to i32
  %t16c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t16w)
  %t16 = sext i32 %t16c to i64
  br label %thx66
thx66:
  br label %join66
else66:
  %t17 = load i64, i64* %L0
  %t18 = add i64 0, 10
  %t19.c = icmp eq i64 %t17, %t18
  %t19 = zext i1 %t19.c to i64
  %t65.b = icmp ne i64 %t19, 0
  br i1 %t65.b, label %then65, label %else65
then65:
  %t20 = add i64 0, 92
  %t21w = trunc i64 %t20 to i32
  %t21c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t21w)
  %t21 = sext i32 %t21c to i64
  %t22 = add i64 0, 110
  %t23w = trunc i64 %t22 to i32
  %t23c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t23w)
  %t23 = sext i32 %t23c to i64
  br label %thx65
thx65:
  br label %join65
else65:
  %t24 = load i64, i64* %L0
  %t25 = add i64 0, 9
  %t26.c = icmp eq i64 %t24, %t25
  %t26 = zext i1 %t26.c to i64
  %t64.b = icmp ne i64 %t26, 0
  br i1 %t64.b, label %then64, label %else64
then64:
  %t27 = add i64 0, 92
  %t28w = trunc i64 %t27 to i32
  %t28c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t28w)
  %t28 = sext i32 %t28c to i64
  %t29 = add i64 0, 116
  %t30w = trunc i64 %t29 to i32
  %t30c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t30w)
  %t30 = sext i32 %t30c to i64
  br label %thx64
thx64:
  br label %join64
else64:
  %t31 = load i64, i64* %L0
  %t32 = add i64 0, 13
  %t33.c = icmp eq i64 %t31, %t32
  %t33 = zext i1 %t33.c to i64
  %t63.b = icmp ne i64 %t33, 0
  br i1 %t63.b, label %then63, label %else63
then63:
  %t34 = add i64 0, 92
  %t35w = trunc i64 %t34 to i32
  %t35c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t35w)
  %t35 = sext i32 %t35c to i64
  %t36 = add i64 0, 114
  %t37w = trunc i64 %t36 to i32
  %t37c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t37w)
  %t37 = sext i32 %t37c to i64
  br label %thx63
thx63:
  br label %join63
else63:
  %t38 = load i64, i64* %L0
  %t39 = add i64 0, 32
  %t40.c = icmp slt i64 %t38, %t39
  %t40 = zext i1 %t40.c to i64
  %t62.b = icmp ne i64 %t40, 0
  br i1 %t62.b, label %then62, label %else62
then62:
  %t41 = add i64 0, 92
  %t42w = trunc i64 %t41 to i32
  %t42c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t42w)
  %t42 = sext i32 %t42c to i64
  %t43 = add i64 0, 117
  %t44w = trunc i64 %t43 to i32
  %t44c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t44w)
  %t44 = sext i32 %t44c to i64
  %t45 = add i64 0, 123
  %t46w = trunc i64 %t45 to i32
  %t46c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t46w)
  %t46 = sext i32 %t46c to i64
  %t47 = load i64, i64* %L0
  %t48 = call i64 @dbghex(i64 %t47)
  %t49p = inttoptr i64 %t48 to i8*
  %t49c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t49p)
  %t49 = sext i32 %t49c to i64
  %t50 = add i64 0, 125
  %t51w = trunc i64 %t50 to i32
  %t51c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t51w)
  %t51 = sext i32 %t51c to i64
  br label %thx62
thx62:
  br label %join62
else62:
  %t52 = load i64, i64* %L0
  %t53 = add i64 0, 127
  %t54.c = icmp sgt i64 %t52, %t53
  %t54 = zext i1 %t54.c to i64
  %t61.b = icmp ne i64 %t54, 0
  br i1 %t61.b, label %then61, label %else61
then61:
  %t55 = load i64, i64* %L0
  store i64 %t55, i64* %L1
  %t56 = load i64, i64* %L1
  %t57 = call i64 @__chrstr(i64 %t56)
  %t58p = inttoptr i64 %t57 to i8*
  %t58c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t58p)
  %t58 = sext i32 %t58c to i64
  br label %thx61
thx61:
  br label %join61
else61:
  %t59 = load i64, i64* %L0
  %t60w = trunc i64 %t59 to i32
  %t60c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t60w)
  %t60 = sext i32 %t60c to i64
  br label %elx61
elx61:
  br label %join61
join61:
  %t61 = phi i64 [ %t58, %thx61 ], [ %t60, %elx61 ]
  br label %elx62
elx62:
  br label %join62
join62:
  %t62 = phi i64 [ %t51, %thx62 ], [ %t61, %elx62 ]
  br label %elx63
elx63:
  br label %join63
join63:
  %t63 = phi i64 [ %t37, %thx63 ], [ %t62, %elx63 ]
  br label %elx64
elx64:
  br label %join64
join64:
  %t64 = phi i64 [ %t30, %thx64 ], [ %t63, %elx64 ]
  br label %elx65
elx65:
  br label %join65
join65:
  %t65 = phi i64 [ %t23, %thx65 ], [ %t64, %elx65 ]
  br label %elx66
elx66:
  br label %join66
join66:
  %t66 = phi i64 [ %t16, %thx66 ], [ %t65, %elx66 ]
  br label %elx67
elx67:
  br label %join67
join67:
  %t67 = phi i64 [ %t9, %thx67 ], [ %t66, %elx67 ]
  %t68 = add i64 0, 39
  %t69w = trunc i64 %t68 to i32
  %t69c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 %t69w)
  %t69 = sext i32 %t69c to i64
  ret i64 %t69
}
define i64 @dbg_vi(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtdr, i64 0, i64 0), i64 %t18)
  %t19 = sext i32 %t19c to i64
  %t20 = load i64, i64* %L2
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L2
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t25p = inttoptr i64 %t24 to i8*
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t25p)
  %t25 = sext i32 %t25c to i64
  ret i64 %t25
}
define i64 @dbg_vb(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = add i64 0, 1
  %t20.c = icmp eq i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  %t25.b = icmp ne i64 %t20, 0
  br i1 %t25.b, label %then25, label %else25
then25:
  %t21 = ptrtoint [5 x i8]* @.s0000155a0842dfc5 to i64
  %t22p = inttoptr i64 %t21 to i8*
  %t22c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t22p)
  %t22 = sext i32 %t22c to i64
  br label %thx25
thx25:
  br label %join25
else25:
  %t23 = ptrtoint [6 x i8]* @.s00156f5401923610 to i64
  %t24p = inttoptr i64 %t23 to i8*
  %t24c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t24p)
  %t24 = sext i32 %t24c to i64
  br label %elx25
elx25:
  br label %join25
join25:
  %t25 = phi i64 [ %t22, %thx25 ], [ %t24, %elx25 ]
  %t26 = load i64, i64* %L2
  %t27 = add i64 0, 1
  %t28 = add i64 %t26, %t27
  store i64 %t28, i64* %L2
  br label %lcond8
lend8:
  %t29 = add i64 0, 0
  %t30 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t31p = inttoptr i64 %t30 to i8*
  %t31c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t31p)
  %t31 = sext i32 %t31c to i64
  ret i64 %t31
}
define i64 @fis_nan(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = load i64, i64* %L0
  %t3a = bitcast i64 %t1 to double
  %t3b = bitcast i64 %t2 to double
  %t3c = fcmp une double %t3a, %t3b
  %t3 = zext i1 %t3c to i64
  ret i64 %t3
}
define i64 @fis_inf(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  %t4 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t5p = inttoptr i64 %t4 to i8*
  %t5d = call double @strtod(i8* %t5p, i8** null)
  %t5 = bitcast double %t5d to i64
  %t6a = bitcast i64 %t3 to double
  %t6b = bitcast i64 %t5 to double
  %t6r = fdiv double %t6a, %t6b
  %t6 = bitcast double %t6r to i64
  %t7a = bitcast i64 %t1 to double
  %t7b = bitcast i64 %t6 to double
  %t7c = fcmp oeq double %t7a, %t7b
  %t7 = zext i1 %t7c to i64
  %t18.b = icmp ne i64 %t7, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t8 = add i64 0, 1
  br label %thx18
thx18:
  br label %join18
else18:
  %t9 = load i64, i64* %L0
  %t10 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t11p = inttoptr i64 %t10 to i8*
  %t11d = call double @strtod(i8* %t11p, i8** null)
  %t11 = bitcast double %t11d to i64
  %t12 = add i64 0, -9223372036854775808
  %t13 = xor i64 %t11, %t12
  %t14 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t15p = inttoptr i64 %t14 to i8*
  %t15d = call double @strtod(i8* %t15p, i8** null)
  %t15 = bitcast double %t15d to i64
  %t16a = bitcast i64 %t13 to double
  %t16b = bitcast i64 %t15 to double
  %t16r = fdiv double %t16a, %t16b
  %t16 = bitcast double %t16r to i64
  %t17a = bitcast i64 %t9 to double
  %t17b = bitcast i64 %t16 to double
  %t17c = fcmp oeq double %t17a, %t17b
  %t17 = zext i1 %t17c to i64
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t8, %thx18 ], [ %t17, %elx18 ]
  ret i64 %t18
}
define i64 @fis_fin(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = load i64, i64* %L0
  %t3a = bitcast i64 %t1 to double
  %t3b = bitcast i64 %t2 to double
  %t3c = fcmp une double %t3a, %t3b
  %t3 = zext i1 %t3c to i64
  %t4 = add i64 0, 0
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t27.b = icmp ne i64 %t5, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t6 = load i64, i64* %L0
  %t7 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t8p = inttoptr i64 %t7 to i8*
  %t8d = call double @strtod(i8* %t8p, i8** null)
  %t8 = bitcast double %t8d to i64
  %t9 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t10p = inttoptr i64 %t9 to i8*
  %t10d = call double @strtod(i8* %t10p, i8** null)
  %t10 = bitcast double %t10d to i64
  %t11a = bitcast i64 %t8 to double
  %t11b = bitcast i64 %t10 to double
  %t11r = fdiv double %t11a, %t11b
  %t11 = bitcast double %t11r to i64
  %t12a = bitcast i64 %t6 to double
  %t12b = bitcast i64 %t11 to double
  %t12c = fcmp oeq double %t12a, %t12b
  %t12 = zext i1 %t12c to i64
  %t23.b = icmp ne i64 %t12, 0
  br i1 %t23.b, label %then23, label %else23
then23:
  %t13 = add i64 0, 1
  br label %thx23
thx23:
  br label %join23
else23:
  %t14 = load i64, i64* %L0
  %t15 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t16p = inttoptr i64 %t15 to i8*
  %t16d = call double @strtod(i8* %t16p, i8** null)
  %t16 = bitcast double %t16d to i64
  %t17 = add i64 0, -9223372036854775808
  %t18 = xor i64 %t16, %t17
  %t19 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t20p = inttoptr i64 %t19 to i8*
  %t20d = call double @strtod(i8* %t20p, i8** null)
  %t20 = bitcast double %t20d to i64
  %t21a = bitcast i64 %t18 to double
  %t21b = bitcast i64 %t20 to double
  %t21r = fdiv double %t21a, %t21b
  %t21 = bitcast double %t21r to i64
  %t22a = bitcast i64 %t14 to double
  %t22b = bitcast i64 %t21 to double
  %t22c = fcmp oeq double %t22a, %t22b
  %t22 = zext i1 %t22c to i64
  br label %elx23
elx23:
  br label %join23
join23:
  %t23 = phi i64 [ %t13, %thx23 ], [ %t22, %elx23 ]
  %t24 = add i64 0, 0
  %t25.c = icmp eq i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  ret i64 %t27
}
define i64 @fsignum(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = load i64, i64* %L0
  %t3a = bitcast i64 %t1 to double
  %t3b = bitcast i64 %t2 to double
  %t3c = fcmp une double %t3a, %t3b
  %t3 = zext i1 %t3c to i64
  %t7.b = icmp ne i64 %t3, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t4 = load i64, i64* %L0
  store i64 %t4, i64* %L1
  %t5 = load i64, i64* %L1
  ret i64 %t5
ar5:
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = add i64 0, 0
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = add i64 0, %t6
  %t8 = load i64, i64* %L0
  %t9 = add i64 0, 0
  %t10.c = icmp slt i64 %t8, %t9
  %t10 = zext i1 %t10.c to i64
  %t17.b = icmp ne i64 %t10, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  %t11 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t12p = inttoptr i64 %t11 to i8*
  %t12d = call double @strtod(i8* %t12p, i8** null)
  %t12 = bitcast double %t12d to i64
  %t13 = add i64 0, -9223372036854775808
  %t14 = xor i64 %t12, %t13
  store i64 %t14, i64* %L1
  %t15 = load i64, i64* %L1
  ret i64 %t15
ar15:
  br label %thx17
thx17:
  br label %join17
else17:
  %t16 = add i64 0, 0
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = add i64 0, %t16
  %t18 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t19p = inttoptr i64 %t18 to i8*
  %t19d = call double @strtod(i8* %t19p, i8** null)
  %t19 = bitcast double %t19d to i64
  ret i64 %t19
}
define i64 @dbg_vf(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = add i64 0, 1
  %t20 = call i64 @f64_str(i64 %t18, i64 %t19)
  %t21p = inttoptr i64 %t20 to i8*
  %t21c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t21p)
  %t21 = sext i32 %t21c to i64
  %t22 = load i64, i64* %L2
  %t23 = add i64 0, 1
  %t24 = add i64 %t22, %t23
  store i64 %t24, i64* %L2
  br label %lcond8
lend8:
  %t25 = add i64 0, 0
  %t26 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t27p = inttoptr i64 %t26 to i8*
  %t27c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t27p)
  %t27 = sext i32 %t27c to i64
  ret i64 %t27
}
define i64 @dbg_vc(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = call i64 @dbgc(i64 %t18)
  %t20 = load i64, i64* %L2
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L2
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t25p = inttoptr i64 %t24 to i8*
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t25p)
  %t25 = sext i32 %t25c to i64
  ret i64 %t25
}
define i64 @dbg_vs(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = call i64 @dbgq(i64 %t18)
  %t20 = load i64, i64* %L2
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L2
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t25p = inttoptr i64 %t24 to i8*
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t25p)
  %t25 = sext i32 %t25c to i64
  ret i64 %t25
}
define i64 @dbg_vvi(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = call i64 @dbg_vi(i64 %t18)
  %t20 = load i64, i64* %L2
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L2
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t25p = inttoptr i64 %t24 to i8*
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t25p)
  %t25 = sext i32 %t25c to i64
  ret i64 %t25
}
define i64 @dbg_ai(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a60 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t2p)
  %t2 = sext i32 %t2c to i64
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__arr_len(i64 %t3)
  store i64 %t4, i64* %L1
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L2
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L1
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 0
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = ptrtoint [3 x i8]* @.s00000000152f4b51 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t13p)
  %t13 = sext i32 %t13c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__arr_get(i64 %t16, i64 %t17)
  %t19c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtdr, i64 0, i64 0), i64 %t18)
  %t19 = sext i32 %t19c to i64
  %t20 = load i64, i64* %L2
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L2
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a62 to i64
  %t25p = inttoptr i64 %t24 to i8*
  %t25c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* %t25p)
  %t25 = sext i32 %t25c to i64
  ret i64 %t25
}
define i64 @sempty(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t7.b = icmp ne i64 %t4, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t5 = add i64 0, 1
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = add i64 0, 0
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = phi i64 [ %t5, %thx7 ], [ %t6, %elx7 ]
  ret i64 %t7
}
define i64 @shas(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @str_find(i64 %t2, i64 %t3)
  %t5 = add i64 0, 0
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t9.b = icmp ne i64 %t6, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t7 = add i64 0, 0
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 1
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = phi i64 [ %t7, %thx9 ], [ %t8, %elx9 ]
  ret i64 %t9
}
define i64 @sfind(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @str_find(i64 %t2, i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 0
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t12.b = icmp ne i64 %t7, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t8 = add i64 0, 0
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 0, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %L3
  %t10 = load i64, i64* %L3
  ret i64 %t10
ar10:
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = add i64 0, %t11
  %t13 = load i64, i64* %L2
  %t14.mb = call i8* @malloc(i64 16)
  %t14.m = bitcast i8* %t14.mb to [2 x i64]*
  %t14.t = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 0
  store i64 1, i64* %t14.t
  %t14.p0 = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 1
  store i64 %t13, i64* %t14.p0
  %t14 = ptrtoint [2 x i64]* %t14.m to i64
  ret i64 %t14
}
define i64 @sstarts(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  %t5 = load i64, i64* %L0
  %t6p = inttoptr i64 %t5 to i8*
  %t6 = call i64 @strlen(i8* %t6p)
  %t7.c = icmp sgt i64 %t4, %t6
  %t7 = zext i1 %t7.c to i64
  %t11.b = icmp ne i64 %t7, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t8 = add i64 0, 0
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = load i64, i64* %L0
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L2
  %t15 = call i64 @__str_sub(i64 %t12, i64 %t13, i64 %t14)
  %t16 = load i64, i64* %L1
  %t17 = call i64 @__streq(i64 %t15, i64 %t16)
  %t20.b = icmp ne i64 %t17, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t18 = add i64 0, 1
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = phi i64 [ %t18, %thx20 ], [ %t19, %elx20 ]
  ret i64 %t20
}
define i64 @sends(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L0
  %t5p = inttoptr i64 %t4 to i8*
  %t5 = call i64 @strlen(i8* %t5p)
  store i64 %t5, i64* %L3
  %t6 = load i64, i64* %L2
  %t7 = load i64, i64* %L3
  %t8.c = icmp sgt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t12.b = icmp ne i64 %t8, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t9 = add i64 0, 0
  store i64 %t9, i64* %L4
  %t10 = load i64, i64* %L4
  ret i64 %t10
ar10:
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = add i64 0, %t11
  %t13 = load i64, i64* %L0
  %t14 = load i64, i64* %L3
  %t15 = load i64, i64* %L2
  %t16 = sub i64 %t14, %t15
  %t17 = load i64, i64* %L2
  %t18 = call i64 @__str_sub(i64 %t13, i64 %t16, i64 %t17)
  %t19 = load i64, i64* %L1
  %t20 = call i64 @__streq(i64 %t18, i64 %t19)
  %t23.b = icmp ne i64 %t20, 0
  br i1 %t23.b, label %then23, label %else23
then23:
  %t21 = add i64 0, 1
  br label %thx23
thx23:
  br label %join23
else23:
  %t22 = add i64 0, 0
  br label %elx23
elx23:
  br label %join23
join23:
  %t23 = phi i64 [ %t21, %thx23 ], [ %t22, %elx23 ]
  ret i64 %t23
}
define i64 @sws(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 32
  %t3.c = icmp eq i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t21.b = icmp ne i64 %t3, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t4 = add i64 0, 1
  br label %thx21
thx21:
  br label %join21
else21:
  %t5 = load i64, i64* %L0
  %t6 = add i64 0, 9
  %t7.c = icmp eq i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t20.b = icmp ne i64 %t7, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t8 = add i64 0, 1
  br label %thx20
thx20:
  br label %join20
else20:
  %t9 = load i64, i64* %L0
  %t10 = add i64 0, 10
  %t11.c = icmp eq i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t19.b = icmp ne i64 %t11, 0
  br i1 %t19.b, label %then19, label %else19
then19:
  %t12 = add i64 0, 1
  br label %thx19
thx19:
  br label %join19
else19:
  %t13 = load i64, i64* %L0
  %t14 = add i64 0, 13
  %t15.c = icmp eq i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  %t18.b = icmp ne i64 %t15, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t16 = add i64 0, 1
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t16, %thx18 ], [ %t17, %elx18 ]
  br label %elx19
elx19:
  br label %join19
join19:
  %t19 = phi i64 [ %t12, %thx19 ], [ %t18, %elx19 ]
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = phi i64 [ %t8, %thx20 ], [ %t19, %elx20 ]
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t4, %thx21 ], [ %t20, %elx21 ]
  ret i64 %t21
}
define i64 @strims(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  store i64 %t2, i64* %L1
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L2
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L2
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__str_at(i64 %t7, i64 %t8)
  %t10 = call i64 @sws(i64 %t9)
  %t11 = add i64 0, 1
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t18.b = icmp ne i64 %t12, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t13 = load i64, i64* %L2
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L2
  br label %thx18
thx18:
  br label %join18
else18:
  br label %lend6
bk16_0:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t15, %thx18 ], [ %t17, %elx18 ]
  br label %lcond6
lend6:
  %t19 = add i64 0, 0
  %t20 = load i64, i64* %L0
  %t21 = load i64, i64* %L2
  %t22 = load i64, i64* %L1
  %t23 = load i64, i64* %L2
  %t24 = sub i64 %t22, %t23
  %t25 = call i64 @__str_sub(i64 %t20, i64 %t21, i64 %t24)
  ret i64 %t25
}
define i64 @strime(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  store i64 %t2, i64* %L1
  br label %lcond5
lcond5:
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 0
  %t5.c = icmp sgt i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %lc5 = icmp ne i64 %t5, 0
  br i1 %lc5, label %lbody5, label %lend5
lbody5:
  %t6 = load i64, i64* %L0
  %t7 = load i64, i64* %L1
  %t8 = add i64 0, 1
  %t9 = sub i64 %t7, %t8
  %t10 = call i64 @__str_at(i64 %t6, i64 %t9)
  %t11 = call i64 @sws(i64 %t10)
  %t12 = add i64 0, 1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t19.b = icmp ne i64 %t13, 0
  br i1 %t19.b, label %then19, label %else19
then19:
  %t14 = load i64, i64* %L1
  %t15 = add i64 0, 1
  %t16 = sub i64 %t14, %t15
  store i64 %t16, i64* %L1
  br label %thx19
thx19:
  br label %join19
else19:
  br label %lend5
bk17_0:
  %t18 = add i64 0, 0
  br label %elx19
elx19:
  br label %join19
join19:
  %t19 = phi i64 [ %t16, %thx19 ], [ %t18, %elx19 ]
  br label %lcond5
lend5:
  %t20 = add i64 0, 0
  %t21 = load i64, i64* %L0
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L1
  %t24 = call i64 @__str_sub(i64 %t21, i64 %t22, i64 %t23)
  ret i64 %t24
}
define i64 @strim(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @strims(i64 %t1)
  %t3 = call i64 @strime(i64 %t2)
  ret i64 %t3
}
define i64 @ccase_up(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 97
  %t3.c = icmp sge i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = load i64, i64* %L0
  %t5 = add i64 0, 122
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t6, %thx8 ], [ %t7, %elx8 ]
  %t14.b = icmp ne i64 %t8, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t9 = load i64, i64* %L0
  %t10 = add i64 0, 32
  %t11 = sub i64 %t9, %t10
  store i64 %t11, i64* %L1
  %t12 = load i64, i64* %L1
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L0
  %t16 = add i64 0, 224
  %t17.c = icmp sge i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = load i64, i64* %L0
  %t19 = add i64 0, 254
  %t20.c = icmp sle i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = phi i64 [ %t20, %thx22 ], [ %t21, %elx22 ]
  %t27.b = icmp ne i64 %t22, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t23 = load i64, i64* %L0
  %t24 = add i64 0, 247
  %t25.c = icmp ne i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  %t33.b = icmp ne i64 %t27, 0
  br i1 %t33.b, label %then33, label %else33
then33:
  %t28 = load i64, i64* %L0
  %t29 = add i64 0, 32
  %t30 = sub i64 %t28, %t29
  store i64 %t30, i64* %L1
  %t31 = load i64, i64* %L1
  ret i64 %t31
ar31:
  br label %thx33
thx33:
  br label %join33
else33:
  %t32 = add i64 0, 0
  br label %elx33
elx33:
  br label %join33
join33:
  %t33 = add i64 0, %t32
  %t34 = load i64, i64* %L0
  %t35 = add i64 0, 255
  %t36.c = icmp eq i64 %t34, %t35
  %t36 = zext i1 %t36.c to i64
  %t40.b = icmp ne i64 %t36, 0
  br i1 %t40.b, label %then40, label %else40
then40:
  %t37 = add i64 0, 376
  store i64 %t37, i64* %L1
  %t38 = load i64, i64* %L1
  ret i64 %t38
ar38:
  br label %thx40
thx40:
  br label %join40
else40:
  %t39 = add i64 0, 0
  br label %elx40
elx40:
  br label %join40
join40:
  %t40 = add i64 0, %t39
  %t41 = load i64, i64* %L0
  %t42 = add i64 0, 256
  %t43.c = icmp sge i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  %t48.b = icmp ne i64 %t43, 0
  br i1 %t48.b, label %then48, label %else48
then48:
  %t44 = load i64, i64* %L0
  %t45 = add i64 0, 311
  %t46.c = icmp sle i64 %t44, %t45
  %t46 = zext i1 %t46.c to i64
  br label %thx48
thx48:
  br label %join48
else48:
  %t47 = add i64 0, 0
  br label %elx48
elx48:
  br label %join48
join48:
  %t48 = phi i64 [ %t46, %thx48 ], [ %t47, %elx48 ]
  %t67.b = icmp ne i64 %t48, 0
  br i1 %t67.b, label %then67, label %else67
then67:
  %t49 = load i64, i64* %L0
  %t50 = load i64, i64* %L0
  %t51 = add i64 0, 2
  %t52d = call i64 @__divchk(i64 %t51)
  %t52 = sdiv i64 %t50, %t52d
  %t53 = add i64 0, 2
  %t54 = mul i64 %t52, %t53
  %t55 = sub i64 %t49, %t54
  %t56 = add i64 0, 1
  %t57.c = icmp eq i64 %t55, %t56
  %t57 = zext i1 %t57.c to i64
  %t63.b = icmp ne i64 %t57, 0
  br i1 %t63.b, label %then63, label %else63
then63:
  %t58 = load i64, i64* %L0
  %t59 = add i64 0, 1
  %t60 = sub i64 %t58, %t59
  store i64 %t60, i64* %L1
  %t61 = load i64, i64* %L1
  ret i64 %t61
ar61:
  br label %thx63
thx63:
  br label %join63
else63:
  %t62 = add i64 0, 0
  br label %elx63
elx63:
  br label %join63
join63:
  %t63 = add i64 0, %t62
  %t64 = load i64, i64* %L0
  store i64 %t64, i64* %L1
  %t65 = load i64, i64* %L1
  ret i64 %t65
ar65:
  br label %thx67
thx67:
  br label %join67
else67:
  %t66 = add i64 0, 0
  br label %elx67
elx67:
  br label %join67
join67:
  %t67 = add i64 0, %t66
  %t68 = load i64, i64* %L0
  %t69 = add i64 0, 330
  %t70.c = icmp sge i64 %t68, %t69
  %t70 = zext i1 %t70.c to i64
  %t75.b = icmp ne i64 %t70, 0
  br i1 %t75.b, label %then75, label %else75
then75:
  %t71 = load i64, i64* %L0
  %t72 = add i64 0, 375
  %t73.c = icmp sle i64 %t71, %t72
  %t73 = zext i1 %t73.c to i64
  br label %thx75
thx75:
  br label %join75
else75:
  %t74 = add i64 0, 0
  br label %elx75
elx75:
  br label %join75
join75:
  %t75 = phi i64 [ %t73, %thx75 ], [ %t74, %elx75 ]
  %t94.b = icmp ne i64 %t75, 0
  br i1 %t94.b, label %then94, label %else94
then94:
  %t76 = load i64, i64* %L0
  %t77 = load i64, i64* %L0
  %t78 = add i64 0, 2
  %t79d = call i64 @__divchk(i64 %t78)
  %t79 = sdiv i64 %t77, %t79d
  %t80 = add i64 0, 2
  %t81 = mul i64 %t79, %t80
  %t82 = sub i64 %t76, %t81
  %t83 = add i64 0, 1
  %t84.c = icmp eq i64 %t82, %t83
  %t84 = zext i1 %t84.c to i64
  %t90.b = icmp ne i64 %t84, 0
  br i1 %t90.b, label %then90, label %else90
then90:
  %t85 = load i64, i64* %L0
  %t86 = add i64 0, 1
  %t87 = sub i64 %t85, %t86
  store i64 %t87, i64* %L1
  %t88 = load i64, i64* %L1
  ret i64 %t88
ar88:
  br label %thx90
thx90:
  br label %join90
else90:
  %t89 = add i64 0, 0
  br label %elx90
elx90:
  br label %join90
join90:
  %t90 = add i64 0, %t89
  %t91 = load i64, i64* %L0
  store i64 %t91, i64* %L1
  %t92 = load i64, i64* %L1
  ret i64 %t92
ar92:
  br label %thx94
thx94:
  br label %join94
else94:
  %t93 = add i64 0, 0
  br label %elx94
elx94:
  br label %join94
join94:
  %t94 = add i64 0, %t93
  %t95 = load i64, i64* %L0
  %t96 = add i64 0, 313
  %t97.c = icmp sge i64 %t95, %t96
  %t97 = zext i1 %t97.c to i64
  %t102.b = icmp ne i64 %t97, 0
  br i1 %t102.b, label %then102, label %else102
then102:
  %t98 = load i64, i64* %L0
  %t99 = add i64 0, 328
  %t100.c = icmp sle i64 %t98, %t99
  %t100 = zext i1 %t100.c to i64
  br label %thx102
thx102:
  br label %join102
else102:
  %t101 = add i64 0, 0
  br label %elx102
elx102:
  br label %join102
join102:
  %t102 = phi i64 [ %t100, %thx102 ], [ %t101, %elx102 ]
  %t121.b = icmp ne i64 %t102, 0
  br i1 %t121.b, label %then121, label %else121
then121:
  %t103 = load i64, i64* %L0
  %t104 = load i64, i64* %L0
  %t105 = add i64 0, 2
  %t106d = call i64 @__divchk(i64 %t105)
  %t106 = sdiv i64 %t104, %t106d
  %t107 = add i64 0, 2
  %t108 = mul i64 %t106, %t107
  %t109 = sub i64 %t103, %t108
  %t110 = add i64 0, 0
  %t111.c = icmp eq i64 %t109, %t110
  %t111 = zext i1 %t111.c to i64
  %t117.b = icmp ne i64 %t111, 0
  br i1 %t117.b, label %then117, label %else117
then117:
  %t112 = load i64, i64* %L0
  %t113 = add i64 0, 1
  %t114 = sub i64 %t112, %t113
  store i64 %t114, i64* %L1
  %t115 = load i64, i64* %L1
  ret i64 %t115
ar115:
  br label %thx117
thx117:
  br label %join117
else117:
  %t116 = add i64 0, 0
  br label %elx117
elx117:
  br label %join117
join117:
  %t117 = add i64 0, %t116
  %t118 = load i64, i64* %L0
  store i64 %t118, i64* %L1
  %t119 = load i64, i64* %L1
  ret i64 %t119
ar119:
  br label %thx121
thx121:
  br label %join121
else121:
  %t120 = add i64 0, 0
  br label %elx121
elx121:
  br label %join121
join121:
  %t121 = add i64 0, %t120
  %t122 = load i64, i64* %L0
  %t123 = add i64 0, 377
  %t124.c = icmp sge i64 %t122, %t123
  %t124 = zext i1 %t124.c to i64
  %t129.b = icmp ne i64 %t124, 0
  br i1 %t129.b, label %then129, label %else129
then129:
  %t125 = load i64, i64* %L0
  %t126 = add i64 0, 382
  %t127.c = icmp sle i64 %t125, %t126
  %t127 = zext i1 %t127.c to i64
  br label %thx129
thx129:
  br label %join129
else129:
  %t128 = add i64 0, 0
  br label %elx129
elx129:
  br label %join129
join129:
  %t129 = phi i64 [ %t127, %thx129 ], [ %t128, %elx129 ]
  %t148.b = icmp ne i64 %t129, 0
  br i1 %t148.b, label %then148, label %else148
then148:
  %t130 = load i64, i64* %L0
  %t131 = load i64, i64* %L0
  %t132 = add i64 0, 2
  %t133d = call i64 @__divchk(i64 %t132)
  %t133 = sdiv i64 %t131, %t133d
  %t134 = add i64 0, 2
  %t135 = mul i64 %t133, %t134
  %t136 = sub i64 %t130, %t135
  %t137 = add i64 0, 0
  %t138.c = icmp eq i64 %t136, %t137
  %t138 = zext i1 %t138.c to i64
  %t144.b = icmp ne i64 %t138, 0
  br i1 %t144.b, label %then144, label %else144
then144:
  %t139 = load i64, i64* %L0
  %t140 = add i64 0, 1
  %t141 = sub i64 %t139, %t140
  store i64 %t141, i64* %L1
  %t142 = load i64, i64* %L1
  ret i64 %t142
ar142:
  br label %thx144
thx144:
  br label %join144
else144:
  %t143 = add i64 0, 0
  br label %elx144
elx144:
  br label %join144
join144:
  %t144 = add i64 0, %t143
  %t145 = load i64, i64* %L0
  store i64 %t145, i64* %L1
  %t146 = load i64, i64* %L1
  ret i64 %t146
ar146:
  br label %thx148
thx148:
  br label %join148
else148:
  %t147 = add i64 0, 0
  br label %elx148
elx148:
  br label %join148
join148:
  %t148 = add i64 0, %t147
  %t149 = load i64, i64* %L0
  %t150 = add i64 0, 962
  %t151.c = icmp eq i64 %t149, %t150
  %t151 = zext i1 %t151.c to i64
  %t155.b = icmp ne i64 %t151, 0
  br i1 %t155.b, label %then155, label %else155
then155:
  %t152 = add i64 0, 931
  store i64 %t152, i64* %L1
  %t153 = load i64, i64* %L1
  ret i64 %t153
ar153:
  br label %thx155
thx155:
  br label %join155
else155:
  %t154 = add i64 0, 0
  br label %elx155
elx155:
  br label %join155
join155:
  %t155 = add i64 0, %t154
  %t156 = load i64, i64* %L0
  %t157 = add i64 0, 945
  %t158.c = icmp sge i64 %t156, %t157
  %t158 = zext i1 %t158.c to i64
  %t163.b = icmp ne i64 %t158, 0
  br i1 %t163.b, label %then163, label %else163
then163:
  %t159 = load i64, i64* %L0
  %t160 = add i64 0, 969
  %t161.c = icmp sle i64 %t159, %t160
  %t161 = zext i1 %t161.c to i64
  br label %thx163
thx163:
  br label %join163
else163:
  %t162 = add i64 0, 0
  br label %elx163
elx163:
  br label %join163
join163:
  %t163 = phi i64 [ %t161, %thx163 ], [ %t162, %elx163 ]
  %t169.b = icmp ne i64 %t163, 0
  br i1 %t169.b, label %then169, label %else169
then169:
  %t164 = load i64, i64* %L0
  %t165 = add i64 0, 32
  %t166 = sub i64 %t164, %t165
  store i64 %t166, i64* %L1
  %t167 = load i64, i64* %L1
  ret i64 %t167
ar167:
  br label %thx169
thx169:
  br label %join169
else169:
  %t168 = add i64 0, 0
  br label %elx169
elx169:
  br label %join169
join169:
  %t169 = add i64 0, %t168
  %t170 = load i64, i64* %L0
  %t171 = add i64 0, 1072
  %t172.c = icmp sge i64 %t170, %t171
  %t172 = zext i1 %t172.c to i64
  %t177.b = icmp ne i64 %t172, 0
  br i1 %t177.b, label %then177, label %else177
then177:
  %t173 = load i64, i64* %L0
  %t174 = add i64 0, 1103
  %t175.c = icmp sle i64 %t173, %t174
  %t175 = zext i1 %t175.c to i64
  br label %thx177
thx177:
  br label %join177
else177:
  %t176 = add i64 0, 0
  br label %elx177
elx177:
  br label %join177
join177:
  %t177 = phi i64 [ %t175, %thx177 ], [ %t176, %elx177 ]
  %t183.b = icmp ne i64 %t177, 0
  br i1 %t183.b, label %then183, label %else183
then183:
  %t178 = load i64, i64* %L0
  %t179 = add i64 0, 32
  %t180 = sub i64 %t178, %t179
  store i64 %t180, i64* %L1
  %t181 = load i64, i64* %L1
  ret i64 %t181
ar181:
  br label %thx183
thx183:
  br label %join183
else183:
  %t182 = add i64 0, 0
  br label %elx183
elx183:
  br label %join183
join183:
  %t183 = add i64 0, %t182
  %t184 = load i64, i64* %L0
  %t185 = add i64 0, 1104
  %t186.c = icmp sge i64 %t184, %t185
  %t186 = zext i1 %t186.c to i64
  %t191.b = icmp ne i64 %t186, 0
  br i1 %t191.b, label %then191, label %else191
then191:
  %t187 = load i64, i64* %L0
  %t188 = add i64 0, 1119
  %t189.c = icmp sle i64 %t187, %t188
  %t189 = zext i1 %t189.c to i64
  br label %thx191
thx191:
  br label %join191
else191:
  %t190 = add i64 0, 0
  br label %elx191
elx191:
  br label %join191
join191:
  %t191 = phi i64 [ %t189, %thx191 ], [ %t190, %elx191 ]
  %t197.b = icmp ne i64 %t191, 0
  br i1 %t197.b, label %then197, label %else197
then197:
  %t192 = load i64, i64* %L0
  %t193 = add i64 0, 80
  %t194 = sub i64 %t192, %t193
  store i64 %t194, i64* %L1
  %t195 = load i64, i64* %L1
  ret i64 %t195
ar195:
  br label %thx197
thx197:
  br label %join197
else197:
  %t196 = add i64 0, 0
  br label %elx197
elx197:
  br label %join197
join197:
  %t197 = add i64 0, %t196
  %t198 = load i64, i64* %L0
  ret i64 %t198
}
define i64 @ccase_low(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 65
  %t3.c = icmp sge i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = load i64, i64* %L0
  %t5 = add i64 0, 90
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t6, %thx8 ], [ %t7, %elx8 ]
  %t14.b = icmp ne i64 %t8, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t9 = load i64, i64* %L0
  %t10 = add i64 0, 32
  %t11 = add i64 %t9, %t10
  store i64 %t11, i64* %L1
  %t12 = load i64, i64* %L1
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L0
  %t16 = add i64 0, 192
  %t17.c = icmp sge i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = load i64, i64* %L0
  %t19 = add i64 0, 222
  %t20.c = icmp sle i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = phi i64 [ %t20, %thx22 ], [ %t21, %elx22 ]
  %t27.b = icmp ne i64 %t22, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t23 = load i64, i64* %L0
  %t24 = add i64 0, 215
  %t25.c = icmp ne i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  %t33.b = icmp ne i64 %t27, 0
  br i1 %t33.b, label %then33, label %else33
then33:
  %t28 = load i64, i64* %L0
  %t29 = add i64 0, 32
  %t30 = add i64 %t28, %t29
  store i64 %t30, i64* %L1
  %t31 = load i64, i64* %L1
  ret i64 %t31
ar31:
  br label %thx33
thx33:
  br label %join33
else33:
  %t32 = add i64 0, 0
  br label %elx33
elx33:
  br label %join33
join33:
  %t33 = add i64 0, %t32
  %t34 = load i64, i64* %L0
  %t35 = add i64 0, 376
  %t36.c = icmp eq i64 %t34, %t35
  %t36 = zext i1 %t36.c to i64
  %t40.b = icmp ne i64 %t36, 0
  br i1 %t40.b, label %then40, label %else40
then40:
  %t37 = add i64 0, 255
  store i64 %t37, i64* %L1
  %t38 = load i64, i64* %L1
  ret i64 %t38
ar38:
  br label %thx40
thx40:
  br label %join40
else40:
  %t39 = add i64 0, 0
  br label %elx40
elx40:
  br label %join40
join40:
  %t40 = add i64 0, %t39
  %t41 = load i64, i64* %L0
  %t42 = add i64 0, 256
  %t43.c = icmp sge i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  %t48.b = icmp ne i64 %t43, 0
  br i1 %t48.b, label %then48, label %else48
then48:
  %t44 = load i64, i64* %L0
  %t45 = add i64 0, 311
  %t46.c = icmp sle i64 %t44, %t45
  %t46 = zext i1 %t46.c to i64
  br label %thx48
thx48:
  br label %join48
else48:
  %t47 = add i64 0, 0
  br label %elx48
elx48:
  br label %join48
join48:
  %t48 = phi i64 [ %t46, %thx48 ], [ %t47, %elx48 ]
  %t67.b = icmp ne i64 %t48, 0
  br i1 %t67.b, label %then67, label %else67
then67:
  %t49 = load i64, i64* %L0
  %t50 = load i64, i64* %L0
  %t51 = add i64 0, 2
  %t52d = call i64 @__divchk(i64 %t51)
  %t52 = sdiv i64 %t50, %t52d
  %t53 = add i64 0, 2
  %t54 = mul i64 %t52, %t53
  %t55 = sub i64 %t49, %t54
  %t56 = add i64 0, 0
  %t57.c = icmp eq i64 %t55, %t56
  %t57 = zext i1 %t57.c to i64
  %t63.b = icmp ne i64 %t57, 0
  br i1 %t63.b, label %then63, label %else63
then63:
  %t58 = load i64, i64* %L0
  %t59 = add i64 0, 1
  %t60 = add i64 %t58, %t59
  store i64 %t60, i64* %L1
  %t61 = load i64, i64* %L1
  ret i64 %t61
ar61:
  br label %thx63
thx63:
  br label %join63
else63:
  %t62 = add i64 0, 0
  br label %elx63
elx63:
  br label %join63
join63:
  %t63 = add i64 0, %t62
  %t64 = load i64, i64* %L0
  store i64 %t64, i64* %L1
  %t65 = load i64, i64* %L1
  ret i64 %t65
ar65:
  br label %thx67
thx67:
  br label %join67
else67:
  %t66 = add i64 0, 0
  br label %elx67
elx67:
  br label %join67
join67:
  %t67 = add i64 0, %t66
  %t68 = load i64, i64* %L0
  %t69 = add i64 0, 330
  %t70.c = icmp sge i64 %t68, %t69
  %t70 = zext i1 %t70.c to i64
  %t75.b = icmp ne i64 %t70, 0
  br i1 %t75.b, label %then75, label %else75
then75:
  %t71 = load i64, i64* %L0
  %t72 = add i64 0, 375
  %t73.c = icmp sle i64 %t71, %t72
  %t73 = zext i1 %t73.c to i64
  br label %thx75
thx75:
  br label %join75
else75:
  %t74 = add i64 0, 0
  br label %elx75
elx75:
  br label %join75
join75:
  %t75 = phi i64 [ %t73, %thx75 ], [ %t74, %elx75 ]
  %t94.b = icmp ne i64 %t75, 0
  br i1 %t94.b, label %then94, label %else94
then94:
  %t76 = load i64, i64* %L0
  %t77 = load i64, i64* %L0
  %t78 = add i64 0, 2
  %t79d = call i64 @__divchk(i64 %t78)
  %t79 = sdiv i64 %t77, %t79d
  %t80 = add i64 0, 2
  %t81 = mul i64 %t79, %t80
  %t82 = sub i64 %t76, %t81
  %t83 = add i64 0, 0
  %t84.c = icmp eq i64 %t82, %t83
  %t84 = zext i1 %t84.c to i64
  %t90.b = icmp ne i64 %t84, 0
  br i1 %t90.b, label %then90, label %else90
then90:
  %t85 = load i64, i64* %L0
  %t86 = add i64 0, 1
  %t87 = add i64 %t85, %t86
  store i64 %t87, i64* %L1
  %t88 = load i64, i64* %L1
  ret i64 %t88
ar88:
  br label %thx90
thx90:
  br label %join90
else90:
  %t89 = add i64 0, 0
  br label %elx90
elx90:
  br label %join90
join90:
  %t90 = add i64 0, %t89
  %t91 = load i64, i64* %L0
  store i64 %t91, i64* %L1
  %t92 = load i64, i64* %L1
  ret i64 %t92
ar92:
  br label %thx94
thx94:
  br label %join94
else94:
  %t93 = add i64 0, 0
  br label %elx94
elx94:
  br label %join94
join94:
  %t94 = add i64 0, %t93
  %t95 = load i64, i64* %L0
  %t96 = add i64 0, 313
  %t97.c = icmp sge i64 %t95, %t96
  %t97 = zext i1 %t97.c to i64
  %t102.b = icmp ne i64 %t97, 0
  br i1 %t102.b, label %then102, label %else102
then102:
  %t98 = load i64, i64* %L0
  %t99 = add i64 0, 328
  %t100.c = icmp sle i64 %t98, %t99
  %t100 = zext i1 %t100.c to i64
  br label %thx102
thx102:
  br label %join102
else102:
  %t101 = add i64 0, 0
  br label %elx102
elx102:
  br label %join102
join102:
  %t102 = phi i64 [ %t100, %thx102 ], [ %t101, %elx102 ]
  %t121.b = icmp ne i64 %t102, 0
  br i1 %t121.b, label %then121, label %else121
then121:
  %t103 = load i64, i64* %L0
  %t104 = load i64, i64* %L0
  %t105 = add i64 0, 2
  %t106d = call i64 @__divchk(i64 %t105)
  %t106 = sdiv i64 %t104, %t106d
  %t107 = add i64 0, 2
  %t108 = mul i64 %t106, %t107
  %t109 = sub i64 %t103, %t108
  %t110 = add i64 0, 1
  %t111.c = icmp eq i64 %t109, %t110
  %t111 = zext i1 %t111.c to i64
  %t117.b = icmp ne i64 %t111, 0
  br i1 %t117.b, label %then117, label %else117
then117:
  %t112 = load i64, i64* %L0
  %t113 = add i64 0, 1
  %t114 = add i64 %t112, %t113
  store i64 %t114, i64* %L1
  %t115 = load i64, i64* %L1
  ret i64 %t115
ar115:
  br label %thx117
thx117:
  br label %join117
else117:
  %t116 = add i64 0, 0
  br label %elx117
elx117:
  br label %join117
join117:
  %t117 = add i64 0, %t116
  %t118 = load i64, i64* %L0
  store i64 %t118, i64* %L1
  %t119 = load i64, i64* %L1
  ret i64 %t119
ar119:
  br label %thx121
thx121:
  br label %join121
else121:
  %t120 = add i64 0, 0
  br label %elx121
elx121:
  br label %join121
join121:
  %t121 = add i64 0, %t120
  %t122 = load i64, i64* %L0
  %t123 = add i64 0, 377
  %t124.c = icmp sge i64 %t122, %t123
  %t124 = zext i1 %t124.c to i64
  %t129.b = icmp ne i64 %t124, 0
  br i1 %t129.b, label %then129, label %else129
then129:
  %t125 = load i64, i64* %L0
  %t126 = add i64 0, 382
  %t127.c = icmp sle i64 %t125, %t126
  %t127 = zext i1 %t127.c to i64
  br label %thx129
thx129:
  br label %join129
else129:
  %t128 = add i64 0, 0
  br label %elx129
elx129:
  br label %join129
join129:
  %t129 = phi i64 [ %t127, %thx129 ], [ %t128, %elx129 ]
  %t148.b = icmp ne i64 %t129, 0
  br i1 %t148.b, label %then148, label %else148
then148:
  %t130 = load i64, i64* %L0
  %t131 = load i64, i64* %L0
  %t132 = add i64 0, 2
  %t133d = call i64 @__divchk(i64 %t132)
  %t133 = sdiv i64 %t131, %t133d
  %t134 = add i64 0, 2
  %t135 = mul i64 %t133, %t134
  %t136 = sub i64 %t130, %t135
  %t137 = add i64 0, 1
  %t138.c = icmp eq i64 %t136, %t137
  %t138 = zext i1 %t138.c to i64
  %t144.b = icmp ne i64 %t138, 0
  br i1 %t144.b, label %then144, label %else144
then144:
  %t139 = load i64, i64* %L0
  %t140 = add i64 0, 1
  %t141 = add i64 %t139, %t140
  store i64 %t141, i64* %L1
  %t142 = load i64, i64* %L1
  ret i64 %t142
ar142:
  br label %thx144
thx144:
  br label %join144
else144:
  %t143 = add i64 0, 0
  br label %elx144
elx144:
  br label %join144
join144:
  %t144 = add i64 0, %t143
  %t145 = load i64, i64* %L0
  store i64 %t145, i64* %L1
  %t146 = load i64, i64* %L1
  ret i64 %t146
ar146:
  br label %thx148
thx148:
  br label %join148
else148:
  %t147 = add i64 0, 0
  br label %elx148
elx148:
  br label %join148
join148:
  %t148 = add i64 0, %t147
  %t149 = load i64, i64* %L0
  %t150 = add i64 0, 913
  %t151.c = icmp sge i64 %t149, %t150
  %t151 = zext i1 %t151.c to i64
  %t156.b = icmp ne i64 %t151, 0
  br i1 %t156.b, label %then156, label %else156
then156:
  %t152 = load i64, i64* %L0
  %t153 = add i64 0, 937
  %t154.c = icmp sle i64 %t152, %t153
  %t154 = zext i1 %t154.c to i64
  br label %thx156
thx156:
  br label %join156
else156:
  %t155 = add i64 0, 0
  br label %elx156
elx156:
  br label %join156
join156:
  %t156 = phi i64 [ %t154, %thx156 ], [ %t155, %elx156 ]
  %t161.b = icmp ne i64 %t156, 0
  br i1 %t161.b, label %then161, label %else161
then161:
  %t157 = load i64, i64* %L0
  %t158 = add i64 0, 930
  %t159.c = icmp ne i64 %t157, %t158
  %t159 = zext i1 %t159.c to i64
  br label %thx161
thx161:
  br label %join161
else161:
  %t160 = add i64 0, 0
  br label %elx161
elx161:
  br label %join161
join161:
  %t161 = phi i64 [ %t159, %thx161 ], [ %t160, %elx161 ]
  %t167.b = icmp ne i64 %t161, 0
  br i1 %t167.b, label %then167, label %else167
then167:
  %t162 = load i64, i64* %L0
  %t163 = add i64 0, 32
  %t164 = add i64 %t162, %t163
  store i64 %t164, i64* %L1
  %t165 = load i64, i64* %L1
  ret i64 %t165
ar165:
  br label %thx167
thx167:
  br label %join167
else167:
  %t166 = add i64 0, 0
  br label %elx167
elx167:
  br label %join167
join167:
  %t167 = add i64 0, %t166
  %t168 = load i64, i64* %L0
  %t169 = add i64 0, 1040
  %t170.c = icmp sge i64 %t168, %t169
  %t170 = zext i1 %t170.c to i64
  %t175.b = icmp ne i64 %t170, 0
  br i1 %t175.b, label %then175, label %else175
then175:
  %t171 = load i64, i64* %L0
  %t172 = add i64 0, 1071
  %t173.c = icmp sle i64 %t171, %t172
  %t173 = zext i1 %t173.c to i64
  br label %thx175
thx175:
  br label %join175
else175:
  %t174 = add i64 0, 0
  br label %elx175
elx175:
  br label %join175
join175:
  %t175 = phi i64 [ %t173, %thx175 ], [ %t174, %elx175 ]
  %t181.b = icmp ne i64 %t175, 0
  br i1 %t181.b, label %then181, label %else181
then181:
  %t176 = load i64, i64* %L0
  %t177 = add i64 0, 32
  %t178 = add i64 %t176, %t177
  store i64 %t178, i64* %L1
  %t179 = load i64, i64* %L1
  ret i64 %t179
ar179:
  br label %thx181
thx181:
  br label %join181
else181:
  %t180 = add i64 0, 0
  br label %elx181
elx181:
  br label %join181
join181:
  %t181 = add i64 0, %t180
  %t182 = load i64, i64* %L0
  %t183 = add i64 0, 1024
  %t184.c = icmp sge i64 %t182, %t183
  %t184 = zext i1 %t184.c to i64
  %t189.b = icmp ne i64 %t184, 0
  br i1 %t189.b, label %then189, label %else189
then189:
  %t185 = load i64, i64* %L0
  %t186 = add i64 0, 1039
  %t187.c = icmp sle i64 %t185, %t186
  %t187 = zext i1 %t187.c to i64
  br label %thx189
thx189:
  br label %join189
else189:
  %t188 = add i64 0, 0
  br label %elx189
elx189:
  br label %join189
join189:
  %t189 = phi i64 [ %t187, %thx189 ], [ %t188, %elx189 ]
  %t195.b = icmp ne i64 %t189, 0
  br i1 %t195.b, label %then195, label %else195
then195:
  %t190 = load i64, i64* %L0
  %t191 = add i64 0, 80
  %t192 = add i64 %t190, %t191
  store i64 %t192, i64* %L1
  %t193 = load i64, i64* %L1
  ret i64 %t193
ar193:
  br label %thx195
thx195:
  br label %join195
else195:
  %t194 = add i64 0, 0
  br label %elx195
elx195:
  br label %join195
join195:
  %t195 = add i64 0, %t194
  %t196 = load i64, i64* %L0
  ret i64 %t196
}
define i64 @s_nonascii(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  store i64 %t2, i64* %L1
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L2
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L2
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__str_at(i64 %t7, i64 %t8)
  %t10 = add i64 0, 127
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = add i64 0, 1
  store i64 %t12, i64* %L3
  %t13 = load i64, i64* %L3
  ret i64 %t13
ar13:
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = add i64 0, %t14
  %t16 = load i64, i64* %L2
  %t17 = add i64 0, 1
  %t18 = add i64 %t16, %t17
  store i64 %t18, i64* %L2
  br label %lcond6
lend6:
  %t19 = add i64 0, 0
  %t20 = add i64 0, 0
  ret i64 %t20
}
define i64 @ccase(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = add i64 0, 1
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = load i64, i64* %L0
  %t6 = call i64 @ccase_up(i64 %t5)
  store i64 %t6, i64* %L2
  %t7 = load i64, i64* %L2
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = call i64 @ccase_low(i64 %t10)
  ret i64 %t11
}
define i64 @scase8(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @str_chars_vec(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L2
  store i64 %t5, i64* %L4
  %t6 = add i64 0, -1
  store i64 %t6, i64* %L5
  br label %lcond13
lcond13:
  %t7 = load i64, i64* %L5
  %t8 = add i64 0, 1
  %t9 = add i64 %t7, %t8
  store i64 %t9, i64* %L5
  %t10 = load i64, i64* %L5
  %t11 = load i64, i64* %L4
  %t12 = call i64 @__vec_len(i64 %t11)
  %t13.c = icmp slt i64 %t10, %t12
  %t13 = zext i1 %t13.c to i64
  %lc13 = icmp ne i64 %t13, 0
  br i1 %lc13, label %lbody13, label %lend13
lbody13:
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L5
  %t16 = call i64 @__vec_get(i64 %t14, i64 %t15)
  store i64 %t16, i64* %L6
  %t17 = load i64, i64* %L6
  %t18 = load i64, i64* %L1
  %t19 = call i64 @ccase(i64 %t17, i64 %t18)
  store i64 %t19, i64* %L7
  %t20 = load i64, i64* %L3
  %t21 = load i64, i64* %L7
  %t22 = call i64 @__chrstr(i64 %t21)
  %t23 = call i64 @__str_cat(i64 %t20, i64 %t22)
  store i64 %t23, i64* %L3
  br label %lcond13
lend13:
  %t24 = add i64 0, 0
  %t25 = load i64, i64* %L3
  ret i64 %t25
}
define i64 @supper(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @s_nonascii(i64 %t1)
  %t8.b = icmp ne i64 %t2, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t3 = load i64, i64* %L0
  %t4 = add i64 0, 1
  %t5 = call i64 @scase8(i64 %t3, i64 %t4)
  store i64 %t5, i64* %L1
  %t6 = load i64, i64* %L1
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L0
  %t10p = inttoptr i64 %t9 to i8*
  %t10 = call i64 @strlen(i8* %t10p)
  store i64 %t10, i64* %L1
  %t11 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t11, i64* %L2
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L3
  br label %lcond15
lcond15:
  %t13 = load i64, i64* %L3
  %t14 = load i64, i64* %L1
  %t15.c = icmp slt i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  %lc15 = icmp ne i64 %t15, 0
  br i1 %lc15, label %lbody15, label %lend15
lbody15:
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L3
  %t18 = call i64 @__str_at(i64 %t16, i64 %t17)
  store i64 %t18, i64* %L4
  %t19 = load i64, i64* %L4
  %t20 = add i64 0, 97
  %t21.c = icmp sge i64 %t19, %t20
  %t21 = zext i1 %t21.c to i64
  %t26.b = icmp ne i64 %t21, 0
  br i1 %t26.b, label %then26, label %else26
then26:
  %t22 = load i64, i64* %L4
  %t23 = add i64 0, 122
  %t24.c = icmp sle i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  br label %thx26
thx26:
  br label %join26
else26:
  %t25 = add i64 0, 0
  br label %elx26
elx26:
  br label %join26
join26:
  %t26 = phi i64 [ %t24, %thx26 ], [ %t25, %elx26 ]
  %t41.b = icmp ne i64 %t26, 0
  br i1 %t41.b, label %then41, label %else41
then41:
  %t27 = load i64, i64* %L2
  %t28 = ptrtoint [27 x i8]* @.s3e554ab522664be4 to i64
  %t29 = load i64, i64* %L4
  %t30 = add i64 0, 97
  %t31 = sub i64 %t29, %t30
  %t32 = add i64 0, 1
  %t33 = call i64 @__str_sub(i64 %t28, i64 %t31, i64 %t32)
  %t34 = call i64 @__str_cat(i64 %t27, i64 %t33)
  store i64 %t34, i64* %L2
  br label %thx41
thx41:
  br label %join41
else41:
  %t35 = load i64, i64* %L2
  %t36 = load i64, i64* %L0
  %t37 = load i64, i64* %L3
  %t38 = add i64 0, 1
  %t39 = call i64 @__str_sub(i64 %t36, i64 %t37, i64 %t38)
  %t40 = call i64 @__str_cat(i64 %t35, i64 %t39)
  store i64 %t40, i64* %L2
  br label %elx41
elx41:
  br label %join41
join41:
  %t41 = phi i64 [ %t34, %thx41 ], [ %t40, %elx41 ]
  %t42 = load i64, i64* %L3
  %t43 = add i64 0, 1
  %t44 = add i64 %t42, %t43
  store i64 %t44, i64* %L3
  br label %lcond15
lend15:
  %t45 = add i64 0, 0
  %t46 = load i64, i64* %L2
  ret i64 %t46
}
define i64 @slower(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @s_nonascii(i64 %t1)
  %t8.b = icmp ne i64 %t2, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t3 = load i64, i64* %L0
  %t4 = add i64 0, 0
  %t5 = call i64 @scase8(i64 %t3, i64 %t4)
  store i64 %t5, i64* %L1
  %t6 = load i64, i64* %L1
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L0
  %t10p = inttoptr i64 %t9 to i8*
  %t10 = call i64 @strlen(i8* %t10p)
  store i64 %t10, i64* %L1
  %t11 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t11, i64* %L2
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L3
  br label %lcond15
lcond15:
  %t13 = load i64, i64* %L3
  %t14 = load i64, i64* %L1
  %t15.c = icmp slt i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  %lc15 = icmp ne i64 %t15, 0
  br i1 %lc15, label %lbody15, label %lend15
lbody15:
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L3
  %t18 = call i64 @__str_at(i64 %t16, i64 %t17)
  store i64 %t18, i64* %L4
  %t19 = load i64, i64* %L4
  %t20 = add i64 0, 65
  %t21.c = icmp sge i64 %t19, %t20
  %t21 = zext i1 %t21.c to i64
  %t26.b = icmp ne i64 %t21, 0
  br i1 %t26.b, label %then26, label %else26
then26:
  %t22 = load i64, i64* %L4
  %t23 = add i64 0, 90
  %t24.c = icmp sle i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  br label %thx26
thx26:
  br label %join26
else26:
  %t25 = add i64 0, 0
  br label %elx26
elx26:
  br label %join26
join26:
  %t26 = phi i64 [ %t24, %thx26 ], [ %t25, %elx26 ]
  %t41.b = icmp ne i64 %t26, 0
  br i1 %t41.b, label %then41, label %else41
then41:
  %t27 = load i64, i64* %L2
  %t28 = ptrtoint [27 x i8]* @.s3fe03083278eef24 to i64
  %t29 = load i64, i64* %L4
  %t30 = add i64 0, 65
  %t31 = sub i64 %t29, %t30
  %t32 = add i64 0, 1
  %t33 = call i64 @__str_sub(i64 %t28, i64 %t31, i64 %t32)
  %t34 = call i64 @__str_cat(i64 %t27, i64 %t33)
  store i64 %t34, i64* %L2
  br label %thx41
thx41:
  br label %join41
else41:
  %t35 = load i64, i64* %L2
  %t36 = load i64, i64* %L0
  %t37 = load i64, i64* %L3
  %t38 = add i64 0, 1
  %t39 = call i64 @__str_sub(i64 %t36, i64 %t37, i64 %t38)
  %t40 = call i64 @__str_cat(i64 %t35, i64 %t39)
  store i64 %t40, i64* %L2
  br label %elx41
elx41:
  br label %join41
join41:
  %t41 = phi i64 [ %t34, %thx41 ], [ %t40, %elx41 ]
  %t42 = load i64, i64* %L3
  %t43 = add i64 0, 1
  %t44 = add i64 %t42, %t43
  store i64 %t44, i64* %L3
  br label %lcond15
lend15:
  %t45 = add i64 0, 0
  %t46 = load i64, i64* %L2
  ret i64 %t46
}
define i64 @srepeat(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  %t8 = load i64, i64* %L0
  %t9 = call i64 @__str_cat(i64 %t7, i64 %t8)
  store i64 %t9, i64* %L2
  %t10 = load i64, i64* %L3
  %t11 = add i64 0, 1
  %t12 = add i64 %t10, %t11
  store i64 %t12, i64* %L3
  br label %lcond6
lend6:
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L2
  ret i64 %t14
}
define i64 @sreplace(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L1
  %t4p = inttoptr i64 %t3 to i8*
  %t4 = call i64 @strlen(i8* %t4p)
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L3
  %t6 = add i64 0, 0
  %t7.c = icmp eq i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t14.b = icmp ne i64 %t7, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L2
  %t10 = add i64 0, 1099511627776
  %t11 = call i64 @sins_empty(i64 %t8, i64 %t9, i64 %t10)
  store i64 %t11, i64* %L4
  %t12 = load i64, i64* %L4
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t15, i64* %L4
  %t16 = load i64, i64* %L0
  store i64 %t16, i64* %L5
  br label %lcond20
lcond20:
  %t17 = load i64, i64* %L5
  %t18p = inttoptr i64 %t17 to i8*
  %t18 = call i64 @strlen(i8* %t18p)
  %t19 = add i64 0, 0
  %t20.c = icmp sgt i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L5
  %t22 = load i64, i64* %L1
  %t23 = call i64 @str_find(i64 %t21, i64 %t22)
  store i64 %t23, i64* %L6
  %t24 = load i64, i64* %L6
  %t25 = add i64 0, 0
  %t26.c = icmp slt i64 %t24, %t25
  %t26 = zext i1 %t26.c to i64
  %t51.b = icmp ne i64 %t26, 0
  br i1 %t51.b, label %then51, label %else51
then51:
  %t27 = load i64, i64* %L4
  %t28 = load i64, i64* %L5
  %t29 = call i64 @__str_cat(i64 %t27, i64 %t28)
  store i64 %t29, i64* %L4
  %t30 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t30, i64* %L5
  br label %thx51
thx51:
  br label %join51
else51:
  %t31 = load i64, i64* %L4
  %t32 = load i64, i64* %L5
  %t33 = add i64 0, 0
  %t34 = load i64, i64* %L6
  %t35 = call i64 @__str_sub(i64 %t32, i64 %t33, i64 %t34)
  %t36 = call i64 @__str_cat(i64 %t31, i64 %t35)
  store i64 %t36, i64* %L4
  %t37 = load i64, i64* %L4
  %t38 = load i64, i64* %L2
  %t39 = call i64 @__str_cat(i64 %t37, i64 %t38)
  store i64 %t39, i64* %L4
  %t40 = load i64, i64* %L5
  %t41 = load i64, i64* %L6
  %t42 = load i64, i64* %L3
  %t43 = add i64 %t41, %t42
  %t44 = load i64, i64* %L5
  %t45p = inttoptr i64 %t44 to i8*
  %t45 = call i64 @strlen(i8* %t45p)
  %t46 = load i64, i64* %L6
  %t47 = sub i64 %t45, %t46
  %t48 = load i64, i64* %L3
  %t49 = sub i64 %t47, %t48
  %t50 = call i64 @__str_sub(i64 %t40, i64 %t43, i64 %t49)
  store i64 %t50, i64* %L5
  br label %elx51
elx51:
  br label %join51
join51:
  %t51 = phi i64 [ %t30, %thx51 ], [ %t50, %elx51 ]
  br label %lcond20
lend20:
  %t52 = add i64 0, 0
  %t53 = load i64, i64* %L4
  ret i64 %t53
}
define i64 @sreplacen(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L1
  %t5p = inttoptr i64 %t4 to i8*
  %t5 = call i64 @strlen(i8* %t5p)
  store i64 %t5, i64* %L4
  %t6 = load i64, i64* %L4
  %t7 = add i64 0, 0
  %t8.c = icmp eq i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t15.b = icmp ne i64 %t8, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L2
  %t11 = load i64, i64* %L3
  %t12 = call i64 @sins_empty(i64 %t9, i64 %t10, i64 %t11)
  store i64 %t12, i64* %L5
  %t13 = load i64, i64* %L5
  ret i64 %t13
ar13:
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = add i64 0, %t14
  %t16 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t16, i64* %L5
  %t17 = load i64, i64* %L0
  store i64 %t17, i64* %L6
  %t18 = add i64 0, 0
  store i64 %t18, i64* %L7
  br label %lcond27
lcond27:
  %t19 = load i64, i64* %L6
  %t20p = inttoptr i64 %t19 to i8*
  %t20 = call i64 @strlen(i8* %t20p)
  %t21 = add i64 0, 0
  %t22.c = icmp sgt i64 %t20, %t21
  %t22 = zext i1 %t22.c to i64
  %t27.b = icmp ne i64 %t22, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t23 = load i64, i64* %L7
  %t24 = load i64, i64* %L3
  %t25.c = icmp slt i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  %lc27 = icmp ne i64 %t27, 0
  br i1 %lc27, label %lbody27, label %lend27
lbody27:
  %t28 = load i64, i64* %L6
  %t29 = load i64, i64* %L1
  %t30 = call i64 @str_find(i64 %t28, i64 %t29)
  store i64 %t30, i64* %L8
  %t31 = load i64, i64* %L8
  %t32 = add i64 0, 0
  %t33.c = icmp slt i64 %t31, %t32
  %t33 = zext i1 %t33.c to i64
  %t61.b = icmp ne i64 %t33, 0
  br i1 %t61.b, label %then61, label %else61
then61:
  %t34 = load i64, i64* %L5
  %t35 = load i64, i64* %L6
  %t36 = call i64 @__str_cat(i64 %t34, i64 %t35)
  store i64 %t36, i64* %L5
  %t37 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t37, i64* %L6
  br label %thx61
thx61:
  br label %join61
else61:
  %t38 = load i64, i64* %L5
  %t39 = load i64, i64* %L6
  %t40 = add i64 0, 0
  %t41 = load i64, i64* %L8
  %t42 = call i64 @__str_sub(i64 %t39, i64 %t40, i64 %t41)
  %t43 = call i64 @__str_cat(i64 %t38, i64 %t42)
  store i64 %t43, i64* %L5
  %t44 = load i64, i64* %L5
  %t45 = load i64, i64* %L2
  %t46 = call i64 @__str_cat(i64 %t44, i64 %t45)
  store i64 %t46, i64* %L5
  %t47 = load i64, i64* %L6
  %t48 = load i64, i64* %L8
  %t49 = load i64, i64* %L4
  %t50 = add i64 %t48, %t49
  %t51 = load i64, i64* %L6
  %t52p = inttoptr i64 %t51 to i8*
  %t52 = call i64 @strlen(i8* %t52p)
  %t53 = load i64, i64* %L8
  %t54 = sub i64 %t52, %t53
  %t55 = load i64, i64* %L4
  %t56 = sub i64 %t54, %t55
  %t57 = call i64 @__str_sub(i64 %t47, i64 %t50, i64 %t56)
  store i64 %t57, i64* %L6
  %t58 = load i64, i64* %L7
  %t59 = add i64 0, 1
  %t60 = add i64 %t58, %t59
  store i64 %t60, i64* %L7
  br label %elx61
elx61:
  br label %join61
join61:
  %t61 = phi i64 [ %t37, %thx61 ], [ %t60, %elx61 ]
  br label %lcond27
lend27:
  %t62 = add i64 0, 0
  %t63 = load i64, i64* %L5
  %t64 = load i64, i64* %L6
  %t65 = call i64 @__str_cat(i64 %t63, i64 %t64)
  ret i64 %t65
}
define i64 @sins_empty(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4p = inttoptr i64 %t3 to i8*
  %t4 = call i64 @strlen(i8* %t4p)
  store i64 %t4, i64* %L3
  %t5 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 0
  store i64 %t6, i64* %L5
  %t7 = add i64 0, 0
  store i64 %t7, i64* %L6
  br label %lcond10
lcond10:
  %t8 = load i64, i64* %L6
  %t9 = load i64, i64* %L3
  %t10.c = icmp sle i64 %t8, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = load i64, i64* %L5
  %t12 = load i64, i64* %L2
  %t13.c = icmp slt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t21.b = icmp ne i64 %t13, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L1
  %t16 = call i64 @__str_cat(i64 %t14, i64 %t15)
  store i64 %t16, i64* %L4
  %t17 = load i64, i64* %L5
  %t18 = add i64 0, 1
  %t19 = add i64 %t17, %t18
  store i64 %t19, i64* %L5
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  %t22 = load i64, i64* %L6
  %t23 = load i64, i64* %L3
  %t24.c = icmp eq i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  %t61.b = icmp ne i64 %t24, 0
  br i1 %t61.b, label %then61, label %else61
then61:
  %t25 = load i64, i64* %L3
  %t26 = add i64 0, 1
  %t27 = add i64 %t25, %t26
  store i64 %t27, i64* %L6
  br label %thx61
thx61:
  br label %join61
else61:
  %t28 = load i64, i64* %L6
  %t29 = add i64 0, 1
  %t30 = add i64 %t28, %t29
  store i64 %t30, i64* %L7
  br label %lcond47
lcond47:
  %t31 = load i64, i64* %L7
  %t32 = load i64, i64* %L3
  %t33.c = icmp slt i64 %t31, %t32
  %t33 = zext i1 %t33.c to i64
  %t40.b = icmp ne i64 %t33, 0
  br i1 %t40.b, label %then40, label %else40
then40:
  %t34 = load i64, i64* %L0
  %t35 = load i64, i64* %L7
  %t36 = call i64 @__str_at(i64 %t34, i64 %t35)
  %t37 = add i64 0, 128
  %t38.c = icmp sge i64 %t36, %t37
  %t38 = zext i1 %t38.c to i64
  br label %thx40
thx40:
  br label %join40
else40:
  %t39 = add i64 0, 0
  br label %elx40
elx40:
  br label %join40
join40:
  %t40 = phi i64 [ %t38, %thx40 ], [ %t39, %elx40 ]
  %t47.b = icmp ne i64 %t40, 0
  br i1 %t47.b, label %then47, label %else47
then47:
  %t41 = load i64, i64* %L0
  %t42 = load i64, i64* %L7
  %t43 = call i64 @__str_at(i64 %t41, i64 %t42)
  %t44 = add i64 0, 192
  %t45.c = icmp slt i64 %t43, %t44
  %t45 = zext i1 %t45.c to i64
  br label %thx47
thx47:
  br label %join47
else47:
  %t46 = add i64 0, 0
  br label %elx47
elx47:
  br label %join47
join47:
  %t47 = phi i64 [ %t45, %thx47 ], [ %t46, %elx47 ]
  %lc47 = icmp ne i64 %t47, 0
  br i1 %lc47, label %lbody47, label %lend47
lbody47:
  %t48 = load i64, i64* %L7
  %t49 = add i64 0, 1
  %t50 = add i64 %t48, %t49
  store i64 %t50, i64* %L7
  br label %lcond47
lend47:
  %t51 = add i64 0, 0
  %t52 = load i64, i64* %L4
  %t53 = load i64, i64* %L0
  %t54 = load i64, i64* %L6
  %t55 = load i64, i64* %L7
  %t56 = load i64, i64* %L6
  %t57 = sub i64 %t55, %t56
  %t58 = call i64 @__str_sub(i64 %t53, i64 %t54, i64 %t57)
  %t59 = call i64 @__str_cat(i64 %t52, i64 %t58)
  store i64 %t59, i64* %L4
  %t60 = load i64, i64* %L7
  store i64 %t60, i64* %L6
  br label %elx61
elx61:
  br label %join61
join61:
  %t61 = phi i64 [ %t27, %thx61 ], [ %t60, %elx61 ]
  br label %lcond10
lend10:
  %t62 = add i64 0, 0
  %t63 = load i64, i64* %L4
  ret i64 %t63
}
define i64 @vsort_s(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  store i64 %t2, i64* %L1
  %t3 = add i64 0, 1
  store i64 %t3, i64* %L2
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L2
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  store i64 %t7, i64* %L3
  br label %lcond23
lcond23:
  %t8 = load i64, i64* %L3
  %t9 = add i64 0, 0
  %t10.c = icmp sgt i64 %t8, %t9
  %t10 = zext i1 %t10.c to i64
  %t23.b = icmp ne i64 %t10, 0
  br i1 %t23.b, label %then23, label %else23
then23:
  %t11 = load i64, i64* %L0
  %t12 = load i64, i64* %L3
  %t13 = add i64 0, 1
  %t14 = sub i64 %t12, %t13
  %t15 = call i64 @__vec_get(i64 %t11, i64 %t14)
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L3
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = call i64 @scmp(i64 %t15, i64 %t18)
  %t20 = add i64 0, 0
  %t21.c = icmp sgt i64 %t19, %t20
  %t21 = zext i1 %t21.c to i64
  br label %thx23
thx23:
  br label %join23
else23:
  %t22 = add i64 0, 0
  br label %elx23
elx23:
  br label %join23
join23:
  %t23 = phi i64 [ %t21, %thx23 ], [ %t22, %elx23 ]
  %lc23 = icmp ne i64 %t23, 0
  br i1 %lc23, label %lbody23, label %lend23
lbody23:
  %t24 = load i64, i64* %L0
  %t25 = load i64, i64* %L3
  %t26 = call i64 @__vec_get(i64 %t24, i64 %t25)
  store i64 %t26, i64* %L4
  %t27 = load i64, i64* %L0
  %t28 = load i64, i64* %L3
  %t29 = load i64, i64* %L0
  %t30 = load i64, i64* %L3
  %t31 = add i64 0, 1
  %t32 = sub i64 %t30, %t31
  %t33 = call i64 @__vec_get(i64 %t29, i64 %t32)
  %t34 = call i64 @__vec_set(i64 %t27, i64 %t28, i64 %t33)
  %t35 = load i64, i64* %L0
  %t36 = load i64, i64* %L3
  %t37 = add i64 0, 1
  %t38 = sub i64 %t36, %t37
  %t39 = load i64, i64* %L4
  %t40 = call i64 @__vec_set(i64 %t35, i64 %t38, i64 %t39)
  %t41 = load i64, i64* %L3
  %t42 = add i64 0, 1
  %t43 = sub i64 %t41, %t42
  store i64 %t43, i64* %L3
  br label %lcond23
lend23:
  %t44 = add i64 0, 0
  %t45 = load i64, i64* %L2
  %t46 = add i64 0, 1
  %t47 = add i64 %t45, %t46
  store i64 %t47, i64* %L2
  br label %lcond6
lend6:
  %t48 = add i64 0, 0
  %t49 = add i64 0, 0
  ret i64 %t49
}
define i64 @scmp(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L1
  %t5p = inttoptr i64 %t4 to i8*
  %t5 = call i64 @strlen(i8* %t5p)
  store i64 %t5, i64* %L3
  %t6 = add i64 0, 0
  store i64 %t6, i64* %L4
  br label %lcond14
lcond14:
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t14.b = icmp ne i64 %t9, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t10 = load i64, i64* %L4
  %t11 = load i64, i64* %L3
  %t12.c = icmp slt i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = phi i64 [ %t12, %thx14 ], [ %t13, %elx14 ]
  %lc14 = icmp ne i64 %t14, 0
  br i1 %lc14, label %lbody14, label %lend14
lbody14:
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L4
  %t17 = call i64 @__str_at(i64 %t15, i64 %t16)
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L1
  %t19 = load i64, i64* %L4
  %t20 = call i64 @__str_at(i64 %t18, i64 %t19)
  store i64 %t20, i64* %L6
  %t21 = load i64, i64* %L5
  %t22 = load i64, i64* %L6
  %t23.c = icmp slt i64 %t21, %t22
  %t23 = zext i1 %t23.c to i64
  %t29.b = icmp ne i64 %t23, 0
  br i1 %t29.b, label %then29, label %else29
then29:
  %t24 = add i64 0, 0
  %t25 = add i64 0, 1
  %t26 = sub i64 %t24, %t25
  store i64 %t26, i64* %L7
  %t27 = load i64, i64* %L7
  ret i64 %t27
ar27:
  br label %thx29
thx29:
  br label %join29
else29:
  %t28 = add i64 0, 0
  br label %elx29
elx29:
  br label %join29
join29:
  %t29 = add i64 0, %t28
  %t30 = load i64, i64* %L6
  %t31 = load i64, i64* %L5
  %t32.c = icmp slt i64 %t30, %t31
  %t32 = zext i1 %t32.c to i64
  %t36.b = icmp ne i64 %t32, 0
  br i1 %t36.b, label %then36, label %else36
then36:
  %t33 = add i64 0, 1
  store i64 %t33, i64* %L7
  %t34 = load i64, i64* %L7
  ret i64 %t34
ar34:
  br label %thx36
thx36:
  br label %join36
else36:
  %t35 = add i64 0, 0
  br label %elx36
elx36:
  br label %join36
join36:
  %t36 = add i64 0, %t35
  %t37 = load i64, i64* %L4
  %t38 = add i64 0, 1
  %t39 = add i64 %t37, %t38
  store i64 %t39, i64* %L4
  br label %lcond14
lend14:
  %t40 = add i64 0, 0
  %t41 = load i64, i64* %L2
  %t42 = load i64, i64* %L3
  %t43.c = icmp slt i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  %t49.b = icmp ne i64 %t43, 0
  br i1 %t49.b, label %then49, label %else49
then49:
  %t44 = add i64 0, 0
  %t45 = add i64 0, 1
  %t46 = sub i64 %t44, %t45
  store i64 %t46, i64* %L5
  %t47 = load i64, i64* %L5
  ret i64 %t47
ar47:
  br label %thx49
thx49:
  br label %join49
else49:
  %t48 = add i64 0, 0
  br label %elx49
elx49:
  br label %join49
join49:
  %t49 = add i64 0, %t48
  %t50 = load i64, i64* %L3
  %t51 = load i64, i64* %L2
  %t52.c = icmp slt i64 %t50, %t51
  %t52 = zext i1 %t52.c to i64
  %t56.b = icmp ne i64 %t52, 0
  br i1 %t56.b, label %then56, label %else56
then56:
  %t53 = add i64 0, 1
  store i64 %t53, i64* %L5
  %t54 = load i64, i64* %L5
  ret i64 %t54
ar54:
  br label %thx56
thx56:
  br label %join56
else56:
  %t55 = add i64 0, 0
  br label %elx56
elx56:
  br label %join56
join56:
  %t56 = add i64 0, %t55
  %t57 = add i64 0, 0
  ret i64 %t57
}
define i64 @ounwrap(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = load i64, i64* %L2
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  call void @exit(i32 101)
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @ounwrap_or(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t6 = load i64, i64* %L1
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @ois_some(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = add i64 0, 1
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @ois_none(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t5 = add i64 0, 1
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @omap(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 0
  %t7.mb = call i8* @malloc(i64 16)
  %t7.m = bitcast i8* %t7.mb to [2 x i64]*
  %t7.t = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 0
  store i64 1, i64* %t7.t
  %t7.p0 = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 1
  store i64 %t6, i64* %t7.p0
  %t7 = ptrtoint [2 x i64]* %t7.m to i64
  store i64 %t7, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t8 = add i64 0, 0
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 0, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @omap_or(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %t5.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L3
  %t5.bp = inttoptr i64 %t4 to i64*
  %t5.tg = load i64, i64* %t5.bp
  br label %m5a0
m5a0:
  %t5.c0 = icmp eq i64 %t5.tg, 1
  br i1 %t5.c0, label %m5a0b, label %m5a1
m5a0b:
  %t5.g0_0 = getelementptr i64, i64* %t5.bp, i64 1
  %t5.v0_0 = load i64, i64* %t5.g0_0
  store i64 %t5.v0_0, i64* %L4
  %t6 = load i64, i64* %L4
  store i64 %t6, i64* %L5
  %t7 = add i64 0, 0
  store i64 %t7, i64* %t5.slot
  br label %m5end
m5a1:
  %t5.c1 = icmp eq i64 %t5.tg, 0
  br i1 %t5.c1, label %m5a1b, label %m5dflt
m5a1b:
  %t8 = load i64, i64* %L1
  store i64 %t8, i64* %t5.slot
  br label %m5end
m5dflt:
  store i64 0, i64* %t5.slot
  br label %m5end
m5end:
  %t5 = load i64, i64* %t5.slot
  ret i64 %t5
}
define i64 @icount_ones(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L3
  %t9 = ashr i64 %t7, %t8
  %t10 = add i64 0, 1
  %t11 = and i64 %t9, %t10
  store i64 %t11, i64* %L4
  %t12 = load i64, i64* %L2
  %t13 = load i64, i64* %L4
  %t14 = add i64 %t12, %t13
  store i64 %t14, i64* %L2
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  br label %lcond6
lend6:
  %t18 = add i64 0, 0
  %t19 = load i64, i64* %L2
  ret i64 %t19
}
define i64 @ileading_zeros(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 1
  %t5 = sub i64 %t3, %t4
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 0
  %t8.c = icmp sge i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L3
  %t11 = ashr i64 %t9, %t10
  %t12 = add i64 0, 1
  %t13 = and i64 %t11, %t12
  store i64 %t13, i64* %L4
  %t14 = load i64, i64* %L4
  %t15 = add i64 0, 1
  %t16.c = icmp eq i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = load i64, i64* %L2
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L2
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L2
  %t24 = load i64, i64* %L3
  %t25 = add i64 0, 1
  %t26 = sub i64 %t24, %t25
  store i64 %t26, i64* %L3
  br label %lcond8
lend8:
  %t27 = add i64 0, 0
  %t28 = load i64, i64* %L2
  ret i64 %t28
}
define i64 @itrailing_zeros(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L3
  %t9 = ashr i64 %t7, %t8
  %t10 = add i64 0, 1
  %t11 = and i64 %t9, %t10
  store i64 %t11, i64* %L4
  %t12 = load i64, i64* %L4
  %t13 = add i64 0, 1
  %t14.c = icmp eq i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t18.b = icmp ne i64 %t14, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t15 = load i64, i64* %L2
  store i64 %t15, i64* %L5
  %t16 = load i64, i64* %L5
  ret i64 %t16
ar16:
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = add i64 0, %t17
  %t19 = load i64, i64* %L2
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L2
  %t22 = load i64, i64* %L3
  %t23 = add i64 0, 1
  %t24 = add i64 %t22, %t23
  store i64 %t24, i64* %L3
  br label %lcond6
lend6:
  %t25 = add i64 0, 0
  %t26 = load i64, i64* %L2
  ret i64 %t26
}
define i64 @isat_add(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t13.b = icmp ne i64 %t9, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t10 = load i64, i64* %L2
  store i64 %t10, i64* %L5
  %t11 = load i64, i64* %L5
  ret i64 %t11
ar11:
  br label %thx13
thx13:
  br label %join13
else13:
  %t12 = add i64 0, 0
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = add i64 0, %t12
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L3
  %t16.c = icmp sgt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = load i64, i64* %L3
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L4
  ret i64 %t21
}
define i64 @isat_sub(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = sub i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t13.b = icmp ne i64 %t9, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t10 = load i64, i64* %L2
  store i64 %t10, i64* %L5
  %t11 = load i64, i64* %L5
  ret i64 %t11
ar11:
  br label %thx13
thx13:
  br label %join13
else13:
  %t12 = add i64 0, 0
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = add i64 0, %t12
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L3
  %t16.c = icmp sgt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = load i64, i64* %L3
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L4
  ret i64 %t21
}
define i64 @isat_mul(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = mul i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t13.b = icmp ne i64 %t9, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t10 = load i64, i64* %L2
  store i64 %t10, i64* %L5
  %t11 = load i64, i64* %L5
  ret i64 %t11
ar11:
  br label %thx13
thx13:
  br label %join13
else13:
  %t12 = add i64 0, 0
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = add i64 0, %t12
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L3
  %t16.c = icmp sgt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = load i64, i64* %L3
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L4
  ret i64 %t21
}
define i64 @ichk_add(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t14.b = icmp ne i64 %t9, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t10 = add i64 0, 0
  %t11.mb = call i8* @malloc(i64 16)
  %t11.m = bitcast i8* %t11.mb to [2 x i64]*
  %t11.t = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 0
  store i64 0, i64* %t11.t
  %t11.p0 = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 1
  store i64 %t10, i64* %t11.p0
  %t11 = ptrtoint [2 x i64]* %t11.m to i64
  store i64 %t11, i64* %L5
  %t12 = load i64, i64* %L5
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L4
  %t16 = load i64, i64* %L3
  %t17.c = icmp sgt i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = add i64 0, 0
  %t19.mb = call i8* @malloc(i64 16)
  %t19.m = bitcast i8* %t19.mb to [2 x i64]*
  %t19.t = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 0
  store i64 0, i64* %t19.t
  %t19.p0 = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 1
  store i64 %t18, i64* %t19.p0
  %t19 = ptrtoint [2 x i64]* %t19.m to i64
  store i64 %t19, i64* %L5
  %t20 = load i64, i64* %L5
  ret i64 %t20
ar20:
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = add i64 0, %t21
  %t23 = load i64, i64* %L4
  %t24.mb = call i8* @malloc(i64 16)
  %t24.m = bitcast i8* %t24.mb to [2 x i64]*
  %t24.t = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  store i64 1, i64* %t24.t
  %t24.p0 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 1
  store i64 %t23, i64* %t24.p0
  %t24 = ptrtoint [2 x i64]* %t24.m to i64
  ret i64 %t24
}
define i64 @ichk_sub(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = sub i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t14.b = icmp ne i64 %t9, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t10 = add i64 0, 0
  %t11.mb = call i8* @malloc(i64 16)
  %t11.m = bitcast i8* %t11.mb to [2 x i64]*
  %t11.t = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 0
  store i64 0, i64* %t11.t
  %t11.p0 = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 1
  store i64 %t10, i64* %t11.p0
  %t11 = ptrtoint [2 x i64]* %t11.m to i64
  store i64 %t11, i64* %L5
  %t12 = load i64, i64* %L5
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L4
  %t16 = load i64, i64* %L3
  %t17.c = icmp sgt i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = add i64 0, 0
  %t19.mb = call i8* @malloc(i64 16)
  %t19.m = bitcast i8* %t19.mb to [2 x i64]*
  %t19.t = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 0
  store i64 0, i64* %t19.t
  %t19.p0 = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 1
  store i64 %t18, i64* %t19.p0
  %t19 = ptrtoint [2 x i64]* %t19.m to i64
  store i64 %t19, i64* %L5
  %t20 = load i64, i64* %L5
  ret i64 %t20
ar20:
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = add i64 0, %t21
  %t23 = load i64, i64* %L4
  %t24.mb = call i8* @malloc(i64 16)
  %t24.m = bitcast i8* %t24.mb to [2 x i64]*
  %t24.t = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  store i64 1, i64* %t24.t
  %t24.p0 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 1
  store i64 %t23, i64* %t24.p0
  %t24 = ptrtoint [2 x i64]* %t24.m to i64
  ret i64 %t24
}
define i64 @ichk_mul(i64 %p0, i64 %p1, i64 %p2, i64 %p3) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = add i64 0, %p3
  store i64 %t3, i64* %L3
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = mul i64 %t4, %t5
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L2
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t14.b = icmp ne i64 %t9, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t10 = add i64 0, 0
  %t11.mb = call i8* @malloc(i64 16)
  %t11.m = bitcast i8* %t11.mb to [2 x i64]*
  %t11.t = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 0
  store i64 0, i64* %t11.t
  %t11.p0 = getelementptr [2 x i64], [2 x i64]* %t11.m, i64 0, i64 1
  store i64 %t10, i64* %t11.p0
  %t11 = ptrtoint [2 x i64]* %t11.m to i64
  store i64 %t11, i64* %L5
  %t12 = load i64, i64* %L5
  ret i64 %t12
ar12:
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = add i64 0, %t13
  %t15 = load i64, i64* %L4
  %t16 = load i64, i64* %L3
  %t17.c = icmp sgt i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t22.b = icmp ne i64 %t17, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t18 = add i64 0, 0
  %t19.mb = call i8* @malloc(i64 16)
  %t19.m = bitcast i8* %t19.mb to [2 x i64]*
  %t19.t = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 0
  store i64 0, i64* %t19.t
  %t19.p0 = getelementptr [2 x i64], [2 x i64]* %t19.m, i64 0, i64 1
  store i64 %t18, i64* %t19.p0
  %t19 = ptrtoint [2 x i64]* %t19.m to i64
  store i64 %t19, i64* %L5
  %t20 = load i64, i64* %L5
  ret i64 %t20
ar20:
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = add i64 0, %t21
  %t23 = load i64, i64* %L4
  %t24.mb = call i8* @malloc(i64 16)
  %t24.m = bitcast i8* %t24.mb to [2 x i64]*
  %t24.t = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  store i64 1, i64* %t24.t
  %t24.p0 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 1
  store i64 %t23, i64* %t24.p0
  %t24 = ptrtoint [2 x i64]* %t24.m to i64
  ret i64 %t24
}
define i64 @ichecked_div(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L2
  %t7 = load i64, i64* %L2
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L1
  %t12d = call i64 @__divchk(i64 %t11)
  %t12 = sdiv i64 %t10, %t12d
  %t13.mb = call i8* @malloc(i64 16)
  %t13.m = bitcast i8* %t13.mb to [2 x i64]*
  %t13.t = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 0
  store i64 1, i64* %t13.t
  %t13.p0 = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 1
  store i64 %t12, i64* %t13.p0
  %t13 = ptrtoint [2 x i64]* %t13.m to i64
  ret i64 %t13
}
define i64 @ounwrap_or_else(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t6 = add i64 0, 0
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @oand_then(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 0
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t7 = add i64 0, 0
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 0, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @ipow(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 1
  store i64 %t2, i64* %L2
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  %t8 = load i64, i64* %L0
  %t9 = mul i64 %t7, %t8
  store i64 %t9, i64* %L2
  %t10 = load i64, i64* %L3
  %t11 = add i64 0, 1
  %t12 = add i64 %t10, %t11
  store i64 %t12, i64* %L3
  br label %lcond6
lend6:
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L2
  ret i64 %t14
}
define i64 @iabs(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3.c = icmp slt i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t9.b = icmp ne i64 %t3, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t4 = add i64 0, 0
  %t5 = load i64, i64* %L0
  %t6 = sub i64 %t4, %t5
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  ret i64 %t10
}
define i64 @imax(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4.c = icmp sgt i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t7.b = icmp ne i64 %t4, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t5 = load i64, i64* %L0
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = load i64, i64* %L1
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = phi i64 [ %t5, %thx7 ], [ %t6, %elx7 ]
  ret i64 %t7
}
define i64 @imin(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4.c = icmp slt i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t7.b = icmp ne i64 %t4, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t5 = load i64, i64* %L0
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = load i64, i64* %L1
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = phi i64 [ %t5, %thx7 ], [ %t6, %elx7 ]
  ret i64 %t7
}
define i64 @ook_or(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t7 = load i64, i64* %L1
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 1, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @runwrap_or(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t4.g1_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v1_0 = load i64, i64* %t4.g1_0
  store i64 %t4.v1_0, i64* %L3
  %t6 = load i64, i64* %L1
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @rok_opt(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = load i64, i64* %L2
  %t5.mb = call i8* @malloc(i64 16)
  %t5.m = bitcast i8* %t5.mb to [2 x i64]*
  %t5.t = getelementptr [2 x i64], [2 x i64]* %t5.m, i64 0, i64 0
  store i64 1, i64* %t5.t
  %t5.p0 = getelementptr [2 x i64], [2 x i64]* %t5.m, i64 0, i64 1
  store i64 %t4, i64* %t5.p0
  %t5 = ptrtoint [2 x i64]* %t5.m to i64
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t3.g1_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v1_0 = load i64, i64* %t3.g1_0
  store i64 %t3.v1_0, i64* %L2
  %t6 = add i64 0, 0
  %t7.mb = call i8* @malloc(i64 16)
  %t7.m = bitcast i8* %t7.mb to [2 x i64]*
  %t7.t = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 0
  store i64 0, i64* %t7.t
  %t7.p0 = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 1
  store i64 %t6, i64* %t7.p0
  %t7 = ptrtoint [2 x i64]* %t7.m to i64
  store i64 %t7, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @ofilter(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 0
  %t7 = add i64 0, 1
  %t8.c = icmp eq i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t13.b = icmp ne i64 %t8, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t9 = load i64, i64* %L3
  %t10.mb = call i8* @malloc(i64 16)
  %t10.m = bitcast i8* %t10.mb to [2 x i64]*
  %t10.t = getelementptr [2 x i64], [2 x i64]* %t10.m, i64 0, i64 0
  store i64 1, i64* %t10.t
  %t10.p0 = getelementptr [2 x i64], [2 x i64]* %t10.m, i64 0, i64 1
  store i64 %t9, i64* %t10.p0
  %t10 = ptrtoint [2 x i64]* %t10.m to i64
  br label %thx13
thx13:
  br label %join13
else13:
  %t11 = add i64 0, 0
  %t12.mb = call i8* @malloc(i64 16)
  %t12.m = bitcast i8* %t12.mb to [2 x i64]*
  %t12.t = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 0
  store i64 0, i64* %t12.t
  %t12.p0 = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 1
  store i64 %t11, i64* %t12.p0
  %t12 = ptrtoint [2 x i64]* %t12.m to i64
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = phi i64 [ %t10, %thx13 ], [ %t12, %elx13 ]
  store i64 %t13, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t14 = add i64 0, 0
  %t15.mb = call i8* @malloc(i64 16)
  %t15.m = bitcast i8* %t15.mb to [2 x i64]*
  %t15.t = getelementptr [2 x i64], [2 x i64]* %t15.m, i64 0, i64 0
  store i64 0, i64* %t15.t
  %t15.p0 = getelementptr [2 x i64], [2 x i64]* %t15.m, i64 0, i64 1
  store i64 %t14, i64* %t15.p0
  %t15 = ptrtoint [2 x i64]* %t15.m to i64
  store i64 %t15, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @cis_digit(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 48
  %t3.c = icmp sge i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = load i64, i64* %L0
  %t5 = add i64 0, 57
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t6, %thx8 ], [ %t7, %elx8 ]
  ret i64 %t8
}
define i64 @cis_alpha(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 65
  %t3.c = icmp sge i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = load i64, i64* %L0
  %t5 = add i64 0, 90
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t6, %thx8 ], [ %t7, %elx8 ]
  %t18.b = icmp ne i64 %t8, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t9 = add i64 0, 1
  br label %thx18
thx18:
  br label %join18
else18:
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 97
  %t12.c = icmp sge i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t17.b = icmp ne i64 %t12, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  %t13 = load i64, i64* %L0
  %t14 = add i64 0, 122
  %t15.c = icmp sle i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  br label %thx17
thx17:
  br label %join17
else17:
  %t16 = add i64 0, 0
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = phi i64 [ %t15, %thx17 ], [ %t16, %elx17 ]
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t9, %thx18 ], [ %t17, %elx18 ]
  %t38.b = icmp ne i64 %t18, 0
  br i1 %t38.b, label %then38, label %else38
then38:
  %t19 = add i64 0, 1
  br label %thx38
thx38:
  br label %join38
else38:
  %t20 = load i64, i64* %L0
  %t21 = add i64 0, 192
  %t22.c = icmp sge i64 %t20, %t21
  %t22 = zext i1 %t22.c to i64
  %t27.b = icmp ne i64 %t22, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t23 = load i64, i64* %L0
  %t24 = add i64 0, 591
  %t25.c = icmp sle i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  br label %thx27
thx27:
  br label %join27
else27:
  %t26 = add i64 0, 0
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t25, %thx27 ], [ %t26, %elx27 ]
  %t32.b = icmp ne i64 %t27, 0
  br i1 %t32.b, label %then32, label %else32
then32:
  %t28 = load i64, i64* %L0
  %t29 = add i64 0, 215
  %t30.c = icmp ne i64 %t28, %t29
  %t30 = zext i1 %t30.c to i64
  br label %thx32
thx32:
  br label %join32
else32:
  %t31 = add i64 0, 0
  br label %elx32
elx32:
  br label %join32
join32:
  %t32 = phi i64 [ %t30, %thx32 ], [ %t31, %elx32 ]
  %t37.b = icmp ne i64 %t32, 0
  br i1 %t37.b, label %then37, label %else37
then37:
  %t33 = load i64, i64* %L0
  %t34 = add i64 0, 247
  %t35.c = icmp ne i64 %t33, %t34
  %t35 = zext i1 %t35.c to i64
  br label %thx37
thx37:
  br label %join37
else37:
  %t36 = add i64 0, 0
  br label %elx37
elx37:
  br label %join37
join37:
  %t37 = phi i64 [ %t35, %thx37 ], [ %t36, %elx37 ]
  br label %elx38
elx38:
  br label %join38
join38:
  %t38 = phi i64 [ %t19, %thx38 ], [ %t37, %elx38 ]
  %t58.b = icmp ne i64 %t38, 0
  br i1 %t58.b, label %then58, label %else58
then58:
  %t39 = add i64 0, 1
  br label %thx58
thx58:
  br label %join58
else58:
  %t40 = load i64, i64* %L0
  %t41 = add i64 0, 880
  %t42.c = icmp sge i64 %t40, %t41
  %t42 = zext i1 %t42.c to i64
  %t47.b = icmp ne i64 %t42, 0
  br i1 %t47.b, label %then47, label %else47
then47:
  %t43 = load i64, i64* %L0
  %t44 = add i64 0, 1327
  %t45.c = icmp sle i64 %t43, %t44
  %t45 = zext i1 %t45.c to i64
  br label %thx47
thx47:
  br label %join47
else47:
  %t46 = add i64 0, 0
  br label %elx47
elx47:
  br label %join47
join47:
  %t47 = phi i64 [ %t45, %thx47 ], [ %t46, %elx47 ]
  %t52.b = icmp ne i64 %t47, 0
  br i1 %t52.b, label %then52, label %else52
then52:
  %t48 = load i64, i64* %L0
  %t49 = add i64 0, 894
  %t50.c = icmp ne i64 %t48, %t49
  %t50 = zext i1 %t50.c to i64
  br label %thx52
thx52:
  br label %join52
else52:
  %t51 = add i64 0, 0
  br label %elx52
elx52:
  br label %join52
join52:
  %t52 = phi i64 [ %t50, %thx52 ], [ %t51, %elx52 ]
  %t57.b = icmp ne i64 %t52, 0
  br i1 %t57.b, label %then57, label %else57
then57:
  %t53 = load i64, i64* %L0
  %t54 = add i64 0, 903
  %t55.c = icmp ne i64 %t53, %t54
  %t55 = zext i1 %t55.c to i64
  br label %thx57
thx57:
  br label %join57
else57:
  %t56 = add i64 0, 0
  br label %elx57
elx57:
  br label %join57
join57:
  %t57 = phi i64 [ %t55, %thx57 ], [ %t56, %elx57 ]
  br label %elx58
elx58:
  br label %join58
join58:
  %t58 = phi i64 [ %t39, %thx58 ], [ %t57, %elx58 ]
  ret i64 %t58
}
define i64 @cis_alnum(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @cis_digit(i64 %t1)
  %t6.b = icmp ne i64 %t2, 0
  br i1 %t6.b, label %then6, label %else6
then6:
  %t3 = add i64 0, 1
  br label %thx6
thx6:
  br label %join6
else6:
  %t4 = load i64, i64* %L0
  %t5 = call i64 @cis_alpha(i64 %t4)
  br label %elx6
elx6:
  br label %join6
join6:
  %t6 = phi i64 [ %t3, %thx6 ], [ %t5, %elx6 ]
  ret i64 %t6
}
define i64 @cis_ws(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 32
  %t3.c = icmp eq i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = add i64 0, 1
  br label %thx8
thx8:
  br label %join8
else8:
  %t5 = load i64, i64* %L0
  %t6 = add i64 0, 9
  %t7.c = icmp eq i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t4, %thx8 ], [ %t7, %elx8 ]
  %t13.b = icmp ne i64 %t8, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t9 = add i64 0, 1
  br label %thx13
thx13:
  br label %join13
else13:
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 10
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = phi i64 [ %t9, %thx13 ], [ %t12, %elx13 ]
  %t18.b = icmp ne i64 %t13, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t14 = add i64 0, 1
  br label %thx18
thx18:
  br label %join18
else18:
  %t15 = load i64, i64* %L0
  %t16 = add i64 0, 13
  %t17.c = icmp eq i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t14, %thx18 ], [ %t17, %elx18 ]
  ret i64 %t18
}
define i64 @rmap(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 0
  %t7.mb = call i8* @malloc(i64 16)
  %t7.m = bitcast i8* %t7.mb to [2 x i64]*
  %t7.t = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 0
  store i64 0, i64* %t7.t
  %t7.p0 = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 1
  store i64 %t6, i64* %t7.p0
  %t7 = ptrtoint [2 x i64]* %t7.m to i64
  store i64 %t7, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t4.g1_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v1_0 = load i64, i64* %t4.g1_0
  store i64 %t4.v1_0, i64* %L4
  %t8 = load i64, i64* %L4
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 1, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @rmaperr(i64 %p0, i64 %p1) {
entry:
  %t4.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4.bp = inttoptr i64 %t3 to i64*
  %t4.tg = load i64, i64* %t4.bp
  br label %m4a0
m4a0:
  %t4.c0 = icmp eq i64 %t4.tg, 0
  br i1 %t4.c0, label %m4a0b, label %m4a1
m4a0b:
  %t4.g0_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v0_0 = load i64, i64* %t4.g0_0
  store i64 %t4.v0_0, i64* %L3
  %t5 = load i64, i64* %L3
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %t4.slot
  br label %m4end
m4a1:
  %t4.c1 = icmp eq i64 %t4.tg, 1
  br i1 %t4.c1, label %m4a1b, label %m4dflt
m4a1b:
  %t4.g1_0 = getelementptr i64, i64* %t4.bp, i64 1
  %t4.v1_0 = load i64, i64* %t4.g1_0
  store i64 %t4.v1_0, i64* %L3
  %t7 = load i64, i64* %L3
  store i64 %t7, i64* %L4
  %t8 = add i64 0, 0
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 1, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %t4.slot
  br label %m4end
m4dflt:
  store i64 0, i64* %t4.slot
  br label %m4end
m4end:
  %t4 = load i64, i64* %t4.slot
  ret i64 %t4
}
define i64 @bnext(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 0
  %t12 = call i64 @__str_at(i64 %t10, i64 %t11)
  %t13.mb = call i8* @malloc(i64 16)
  %t13.m = bitcast i8* %t13.mb to [2 x i64]*
  %t13.t = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 0
  store i64 1, i64* %t13.t
  %t13.p0 = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 1
  store i64 %t12, i64* %t13.p0
  %t13 = ptrtoint [2 x i64]* %t13.m to i64
  ret i64 %t13
}
define i64 @eargs() {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = call i64 @__vec_new()
  store i64 %t0, i64* %L0
  %t1 = call i64 @__argc_get()
  store i64 %t1, i64* %L1
  %t2 = add i64 0, 0
  store i64 %t2, i64* %L2
  br label %lcond5
lcond5:
  %t3 = load i64, i64* %L2
  %t4 = load i64, i64* %L1
  %t5.c = icmp slt i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %lc5 = icmp ne i64 %t5, 0
  br i1 %lc5, label %lbody5, label %lend5
lbody5:
  %t6 = load i64, i64* %L0
  %t7 = load i64, i64* %L2
  %t8 = call i64 @__argv_at(i64 %t7)
  %t9 = call i64 @__vec_push(i64 %t6, i64 %t8)
  %t10 = load i64, i64* %L2
  %t11 = add i64 0, 1
  %t12 = add i64 %t10, %t11
  store i64 %t12, i64* %L2
  br label %lcond5
lend5:
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L0
  ret i64 %t14
}
define i64 @freadok(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__fileok(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 1, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = call i64 @__readfile(i64 %t10)
  %t12.mb = call i8* @malloc(i64 16)
  %t12.m = bitcast i8* %t12.mb to [2 x i64]*
  %t12.t = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 0
  store i64 0, i64* %t12.t
  %t12.p0 = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 1
  store i64 %t11, i64* %t12.p0
  %t12 = ptrtoint [2 x i64]* %t12.m to i64
  ret i64 %t12
}
define i64 @vremove(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = call i64 @__vec_get(i64 %t4, i64 %t5)
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L1
  store i64 %t7, i64* %L4
  br label %lcond12
lcond12:
  %t8 = load i64, i64* %L4
  %t9 = add i64 0, 1
  %t10 = add i64 %t8, %t9
  %t11 = load i64, i64* %L2
  %t12.c = icmp slt i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %lc12 = icmp ne i64 %t12, 0
  br i1 %lc12, label %lbody12, label %lend12
lbody12:
  %t13 = load i64, i64* %L0
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L4
  %t17 = add i64 0, 1
  %t18 = add i64 %t16, %t17
  %t19 = call i64 @__vec_get(i64 %t15, i64 %t18)
  %t20 = call i64 @__vec_set(i64 %t13, i64 %t14, i64 %t19)
  %t21 = load i64, i64* %L4
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L4
  br label %lcond12
lend12:
  %t24 = add i64 0, 0
  %t25 = load i64, i64* %L0
  %t26 = call i64 @vpop(i64 %t25)
  store i64 %t26, i64* %L5
  %t27 = load i64, i64* %L3
  ret i64 %t27
}
define i64 @vinsert(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L0
  %t6 = load i64, i64* %L2
  %t7 = call i64 @__vec_push(i64 %t5, i64 %t6)
  %t8 = load i64, i64* %L3
  store i64 %t8, i64* %L4
  br label %lcond11
lcond11:
  %t9 = load i64, i64* %L4
  %t10 = load i64, i64* %L1
  %t11.c = icmp sgt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L0
  %t13 = load i64, i64* %L4
  %t14 = load i64, i64* %L0
  %t15 = load i64, i64* %L4
  %t16 = add i64 0, 1
  %t17 = sub i64 %t15, %t16
  %t18 = call i64 @__vec_get(i64 %t14, i64 %t17)
  %t19 = call i64 @__vec_set(i64 %t12, i64 %t13, i64 %t18)
  %t20 = load i64, i64* %L4
  %t21 = add i64 0, 1
  %t22 = sub i64 %t20, %t21
  store i64 %t22, i64* %L4
  br label %lcond11
lend11:
  %t23 = add i64 0, 0
  %t24 = load i64, i64* %L0
  %t25 = load i64, i64* %L1
  %t26 = load i64, i64* %L2
  %t27 = call i64 @__vec_set(i64 %t24, i64 %t25, i64 %t26)
  %t28 = add i64 0, 0
  ret i64 %t28
}
define i64 @vpop(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = call i64 @__vec_popraw(i64 %t10)
  %t12.mb = call i8* @malloc(i64 16)
  %t12.m = bitcast i8* %t12.mb to [2 x i64]*
  %t12.t = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 0
  store i64 1, i64* %t12.t
  %t12.p0 = getelementptr [2 x i64], [2 x i64]* %t12.m, i64 0, i64 1
  store i64 %t11, i64* %t12.p0
  %t12 = ptrtoint [2 x i64]* %t12.m to i64
  ret i64 %t12
}
define i64 @rexit(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t3.g1_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v1_0 = load i64, i64* %t3.g1_0
  store i64 %t3.v1_0, i64* %L2
  %t5 = add i64 0, 1
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @runwrap(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = load i64, i64* %L2
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t3.g1_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v1_0 = load i64, i64* %t3.g1_0
  store i64 %t3.v1_0, i64* %L2
  call void @exit(i32 101)
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @rok(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = add i64 0, 1
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t3.g1_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v1_0 = load i64, i64* %t3.g1_0
  store i64 %t3.v1_0, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @rerr(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t3.g1_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v1_0 = load i64, i64* %t3.g1_0
  store i64 %t3.v1_0, i64* %L2
  %t5 = add i64 0, 1
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @sparse(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2p = inttoptr i64 %t1 to i8*
  %t2 = call i64 @strlen(i8* %t2p)
  store i64 %t2, i64* %L1
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 0
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t10.b = icmp ne i64 %t5, 0
  br i1 %t10.b, label %then10, label %else10
then10:
  %t6 = add i64 0, 0
  %t7.mb = call i8* @malloc(i64 16)
  %t7.m = bitcast i8* %t7.mb to [2 x i64]*
  %t7.t = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 0
  store i64 1, i64* %t7.t
  %t7.p0 = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 1
  store i64 %t6, i64* %t7.p0
  %t7 = ptrtoint [2 x i64]* %t7.m to i64
  store i64 %t7, i64* %L2
  %t8 = load i64, i64* %L2
  ret i64 %t8
ar8:
  br label %thx10
thx10:
  br label %join10
else10:
  %t9 = add i64 0, 0
  br label %elx10
elx10:
  br label %join10
join10:
  %t10 = add i64 0, %t9
  %t11 = add i64 0, 0
  store i64 %t11, i64* %L2
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L3
  %t13 = load i64, i64* %L0
  %t14 = add i64 0, 0
  %t15 = call i64 @__str_at(i64 %t13, i64 %t14)
  %t16 = add i64 0, 45
  %t17.c = icmp eq i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t21.b = icmp ne i64 %t17, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t18 = add i64 0, 1
  store i64 %t18, i64* %L3
  %t19 = add i64 0, 1
  store i64 %t19, i64* %L2
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  %t22 = load i64, i64* %L2
  %t23 = load i64, i64* %L1
  %t24.c = icmp sge i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  %t29.b = icmp ne i64 %t24, 0
  br i1 %t29.b, label %then29, label %else29
then29:
  %t25 = add i64 0, 0
  %t26.mb = call i8* @malloc(i64 16)
  %t26.m = bitcast i8* %t26.mb to [2 x i64]*
  %t26.t = getelementptr [2 x i64], [2 x i64]* %t26.m, i64 0, i64 0
  store i64 1, i64* %t26.t
  %t26.p0 = getelementptr [2 x i64], [2 x i64]* %t26.m, i64 0, i64 1
  store i64 %t25, i64* %t26.p0
  %t26 = ptrtoint [2 x i64]* %t26.m to i64
  store i64 %t26, i64* %L4
  %t27 = load i64, i64* %L4
  ret i64 %t27
ar27:
  br label %thx29
thx29:
  br label %join29
else29:
  %t28 = add i64 0, 0
  br label %elx29
elx29:
  br label %join29
join29:
  %t29 = add i64 0, %t28
  %t30 = add i64 0, 0
  store i64 %t30, i64* %L4
  br label %lcond33
lcond33:
  %t31 = load i64, i64* %L2
  %t32 = load i64, i64* %L1
  %t33.c = icmp slt i64 %t31, %t32
  %t33 = zext i1 %t33.c to i64
  %lc33 = icmp ne i64 %t33, 0
  br i1 %lc33, label %lbody33, label %lend33
lbody33:
  %t34 = load i64, i64* %L0
  %t35 = load i64, i64* %L2
  %t36 = call i64 @__str_at(i64 %t34, i64 %t35)
  store i64 %t36, i64* %L5
  %t37 = load i64, i64* %L5
  %t38 = add i64 0, 48
  %t39.c = icmp slt i64 %t37, %t38
  %t39 = zext i1 %t39.c to i64
  %t44.b = icmp ne i64 %t39, 0
  br i1 %t44.b, label %then44, label %else44
then44:
  %t40 = add i64 0, 1
  br label %thx44
thx44:
  br label %join44
else44:
  %t41 = load i64, i64* %L5
  %t42 = add i64 0, 57
  %t43.c = icmp sgt i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  br label %elx44
elx44:
  br label %join44
join44:
  %t44 = phi i64 [ %t40, %thx44 ], [ %t43, %elx44 ]
  %t49.b = icmp ne i64 %t44, 0
  br i1 %t49.b, label %then49, label %else49
then49:
  %t45 = add i64 0, 0
  %t46.mb = call i8* @malloc(i64 16)
  %t46.m = bitcast i8* %t46.mb to [2 x i64]*
  %t46.t = getelementptr [2 x i64], [2 x i64]* %t46.m, i64 0, i64 0
  store i64 1, i64* %t46.t
  %t46.p0 = getelementptr [2 x i64], [2 x i64]* %t46.m, i64 0, i64 1
  store i64 %t45, i64* %t46.p0
  %t46 = ptrtoint [2 x i64]* %t46.m to i64
  store i64 %t46, i64* %L6
  %t47 = load i64, i64* %L6
  ret i64 %t47
ar47:
  br label %thx49
thx49:
  br label %join49
else49:
  %t48 = add i64 0, 0
  br label %elx49
elx49:
  br label %join49
join49:
  %t49 = add i64 0, %t48
  %t50 = load i64, i64* %L4
  %t51 = add i64 0, 10
  %t52 = mul i64 %t50, %t51
  %t53 = load i64, i64* %L5
  %t54 = add i64 0, 48
  %t55 = sub i64 %t53, %t54
  %t56 = add i64 %t52, %t55
  store i64 %t56, i64* %L4
  %t57 = load i64, i64* %L2
  %t58 = add i64 0, 1
  %t59 = add i64 %t57, %t58
  store i64 %t59, i64* %L2
  br label %lcond33
lend33:
  %t60 = add i64 0, 0
  %t61 = load i64, i64* %L3
  %t62 = add i64 0, 1
  %t63.c = icmp eq i64 %t61, %t62
  %t63 = zext i1 %t63.c to i64
  %t70.b = icmp ne i64 %t63, 0
  br i1 %t70.b, label %then70, label %else70
then70:
  %t64 = add i64 0, 0
  %t65 = load i64, i64* %L4
  %t66 = sub i64 %t64, %t65
  %t67.mb = call i8* @malloc(i64 16)
  %t67.m = bitcast i8* %t67.mb to [2 x i64]*
  %t67.t = getelementptr [2 x i64], [2 x i64]* %t67.m, i64 0, i64 0
  store i64 0, i64* %t67.t
  %t67.p0 = getelementptr [2 x i64], [2 x i64]* %t67.m, i64 0, i64 1
  store i64 %t66, i64* %t67.p0
  %t67 = ptrtoint [2 x i64]* %t67.m to i64
  store i64 %t67, i64* %L5
  %t68 = load i64, i64* %L5
  ret i64 %t68
ar68:
  br label %thx70
thx70:
  br label %join70
else70:
  %t69 = add i64 0, 0
  br label %elx70
elx70:
  br label %join70
join70:
  %t70 = add i64 0, %t69
  %t71 = load i64, i64* %L4
  %t72.mb = call i8* @malloc(i64 16)
  %t72.m = bitcast i8* %t72.mb to [2 x i64]*
  %t72.t = getelementptr [2 x i64], [2 x i64]* %t72.m, i64 0, i64 0
  store i64 0, i64* %t72.t
  %t72.p0 = getelementptr [2 x i64], [2 x i64]* %t72.m, i64 0, i64 1
  store i64 %t71, i64* %t72.p0
  %t72 = ptrtoint [2 x i64]* %t72.m to i64
  ret i64 %t72
}
define i64 @omax(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 0
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  store i64 %t12, i64* %L1
  %t13 = load i64, i64* %L0
  store i64 %t13, i64* %L2
  %t14 = add i64 0, -1
  store i64 %t14, i64* %L3
  br label %lcond21
lcond21:
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  %t18 = load i64, i64* %L3
  %t19 = load i64, i64* %L2
  %t20 = call i64 @__vec_len(i64 %t19)
  %t21.c = icmp slt i64 %t18, %t20
  %t21 = zext i1 %t21.c to i64
  %lc21 = icmp ne i64 %t21, 0
  br i1 %lc21, label %lbody21, label %lend21
lbody21:
  %t22 = load i64, i64* %L2
  %t23 = load i64, i64* %L3
  %t24 = call i64 @__vec_get(i64 %t22, i64 %t23)
  store i64 %t24, i64* %L4
  %t25 = load i64, i64* %L1
  %t26 = load i64, i64* %L4
  %t27.c = icmp slt i64 %t25, %t26
  %t27 = zext i1 %t27.c to i64
  %t30.b = icmp ne i64 %t27, 0
  br i1 %t30.b, label %then30, label %else30
then30:
  %t28 = load i64, i64* %L4
  store i64 %t28, i64* %L1
  br label %thx30
thx30:
  br label %join30
else30:
  %t29 = add i64 0, 0
  br label %elx30
elx30:
  br label %join30
join30:
  %t30 = phi i64 [ %t28, %thx30 ], [ %t29, %elx30 ]
  br label %lcond21
lend21:
  %t31 = add i64 0, 0
  %t32 = load i64, i64* %L1
  %t33.mb = call i8* @malloc(i64 16)
  %t33.m = bitcast i8* %t33.mb to [2 x i64]*
  %t33.t = getelementptr [2 x i64], [2 x i64]* %t33.m, i64 0, i64 0
  store i64 1, i64* %t33.t
  %t33.p0 = getelementptr [2 x i64], [2 x i64]* %t33.m, i64 0, i64 1
  store i64 %t32, i64* %t33.p0
  %t33 = ptrtoint [2 x i64]* %t33.m to i64
  ret i64 %t33
}
define i64 @omin(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 0
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  store i64 %t12, i64* %L1
  %t13 = load i64, i64* %L0
  store i64 %t13, i64* %L2
  %t14 = add i64 0, -1
  store i64 %t14, i64* %L3
  br label %lcond21
lcond21:
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  %t18 = load i64, i64* %L3
  %t19 = load i64, i64* %L2
  %t20 = call i64 @__vec_len(i64 %t19)
  %t21.c = icmp slt i64 %t18, %t20
  %t21 = zext i1 %t21.c to i64
  %lc21 = icmp ne i64 %t21, 0
  br i1 %lc21, label %lbody21, label %lend21
lbody21:
  %t22 = load i64, i64* %L2
  %t23 = load i64, i64* %L3
  %t24 = call i64 @__vec_get(i64 %t22, i64 %t23)
  store i64 %t24, i64* %L4
  %t25 = load i64, i64* %L4
  %t26 = load i64, i64* %L1
  %t27.c = icmp slt i64 %t25, %t26
  %t27 = zext i1 %t27.c to i64
  %t30.b = icmp ne i64 %t27, 0
  br i1 %t30.b, label %then30, label %else30
then30:
  %t28 = load i64, i64* %L4
  store i64 %t28, i64* %L1
  br label %thx30
thx30:
  br label %join30
else30:
  %t29 = add i64 0, 0
  br label %elx30
elx30:
  br label %join30
join30:
  %t30 = phi i64 [ %t28, %thx30 ], [ %t29, %elx30 ]
  br label %lcond21
lend21:
  %t31 = add i64 0, 0
  %t32 = load i64, i64* %L1
  %t33.mb = call i8* @malloc(i64 16)
  %t33.m = bitcast i8* %t33.mb to [2 x i64]*
  %t33.t = getelementptr [2 x i64], [2 x i64]* %t33.m, i64 0, i64 0
  store i64 1, i64* %t33.t
  %t33.p0 = getelementptr [2 x i64], [2 x i64]* %t33.m, i64 0, i64 1
  store i64 %t32, i64* %t33.p0
  %t33 = ptrtoint [2 x i64]* %t33.m to i64
  ret i64 %t33
}
define i64 @ofirst(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 0
  %t6.mb = call i8* @malloc(i64 16)
  %t6.m = bitcast i8* %t6.mb to [2 x i64]*
  %t6.t = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 0
  store i64 0, i64* %t6.t
  %t6.p0 = getelementptr [2 x i64], [2 x i64]* %t6.m, i64 0, i64 1
  store i64 %t5, i64* %t6.p0
  %t6 = ptrtoint [2 x i64]* %t6.m to i64
  store i64 %t6, i64* %L1
  %t7 = load i64, i64* %L1
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 0
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13.mb = call i8* @malloc(i64 16)
  %t13.m = bitcast i8* %t13.mb to [2 x i64]*
  %t13.t = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 0
  store i64 1, i64* %t13.t
  %t13.p0 = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 1
  store i64 %t12, i64* %t13.p0
  %t13 = ptrtoint [2 x i64]* %t13.m to i64
  ret i64 %t13
}
define i64 @olast(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  store i64 %t2, i64* %L1
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 0
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t10.b = icmp ne i64 %t5, 0
  br i1 %t10.b, label %then10, label %else10
then10:
  %t6 = add i64 0, 0
  %t7.mb = call i8* @malloc(i64 16)
  %t7.m = bitcast i8* %t7.mb to [2 x i64]*
  %t7.t = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 0
  store i64 0, i64* %t7.t
  %t7.p0 = getelementptr [2 x i64], [2 x i64]* %t7.m, i64 0, i64 1
  store i64 %t6, i64* %t7.p0
  %t7 = ptrtoint [2 x i64]* %t7.m to i64
  store i64 %t7, i64* %L2
  %t8 = load i64, i64* %L2
  ret i64 %t8
ar8:
  br label %thx10
thx10:
  br label %join10
else10:
  %t9 = add i64 0, 0
  br label %elx10
elx10:
  br label %join10
join10:
  %t10 = add i64 0, %t9
  %t11 = load i64, i64* %L0
  %t12 = load i64, i64* %L1
  %t13 = add i64 0, 1
  %t14 = sub i64 %t12, %t13
  %t15 = call i64 @__vec_get(i64 %t11, i64 %t14)
  %t16.mb = call i8* @malloc(i64 16)
  %t16.m = bitcast i8* %t16.mb to [2 x i64]*
  %t16.t = getelementptr [2 x i64], [2 x i64]* %t16.m, i64 0, i64 0
  store i64 1, i64* %t16.t
  %t16.p0 = getelementptr [2 x i64], [2 x i64]* %t16.m, i64 0, i64 1
  store i64 %t15, i64* %t16.p0
  %t16 = ptrtoint [2 x i64]* %t16.m to i64
  ret i64 %t16
}
define i64 @vbsearch(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 0
  %t6.c = icmp eq i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t11.b = icmp ne i64 %t6, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t7 = add i64 0, 0
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 1, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L3
  br label %lcond15
lcond15:
  %t13 = load i64, i64* %L2
  %t14 = add i64 0, 1
  %t15.c = icmp sgt i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  %lc15 = icmp ne i64 %t15, 0
  br i1 %lc15, label %lbody15, label %lend15
lbody15:
  %t16 = load i64, i64* %L2
  %t17 = add i64 0, 2
  %t18d = call i64 @__divchk(i64 %t17)
  %t18 = sdiv i64 %t16, %t18d
  store i64 %t18, i64* %L4
  %t19 = load i64, i64* %L3
  %t20 = load i64, i64* %L4
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L5
  %t22 = load i64, i64* %L0
  %t23 = load i64, i64* %L5
  %t24 = call i64 @__vec_get(i64 %t22, i64 %t23)
  %t25 = load i64, i64* %L1
  %t26.c = icmp sgt i64 %t24, %t25
  %t26 = zext i1 %t26.c to i64
  %t29.b = icmp ne i64 %t26, 0
  br i1 %t29.b, label %then29, label %else29
then29:
  %t27 = load i64, i64* %L3
  store i64 %t27, i64* %L3
  br label %thx29
thx29:
  br label %join29
else29:
  %t28 = load i64, i64* %L5
  store i64 %t28, i64* %L3
  br label %elx29
elx29:
  br label %join29
join29:
  %t29 = phi i64 [ %t27, %thx29 ], [ %t28, %elx29 ]
  %t30 = load i64, i64* %L2
  %t31 = load i64, i64* %L4
  %t32 = sub i64 %t30, %t31
  store i64 %t32, i64* %L2
  br label %lcond15
lend15:
  %t33 = add i64 0, 0
  %t34 = load i64, i64* %L0
  %t35 = load i64, i64* %L3
  %t36 = call i64 @__vec_get(i64 %t34, i64 %t35)
  store i64 %t36, i64* %L4
  %t37 = load i64, i64* %L4
  %t38 = load i64, i64* %L1
  %t39.c = icmp eq i64 %t37, %t38
  %t39 = zext i1 %t39.c to i64
  %t44.b = icmp ne i64 %t39, 0
  br i1 %t44.b, label %then44, label %else44
then44:
  %t40 = load i64, i64* %L3
  %t41.mb = call i8* @malloc(i64 16)
  %t41.m = bitcast i8* %t41.mb to [2 x i64]*
  %t41.t = getelementptr [2 x i64], [2 x i64]* %t41.m, i64 0, i64 0
  store i64 0, i64* %t41.t
  %t41.p0 = getelementptr [2 x i64], [2 x i64]* %t41.m, i64 0, i64 1
  store i64 %t40, i64* %t41.p0
  %t41 = ptrtoint [2 x i64]* %t41.m to i64
  store i64 %t41, i64* %L5
  %t42 = load i64, i64* %L5
  ret i64 %t42
ar42:
  br label %thx44
thx44:
  br label %join44
else44:
  %t43 = add i64 0, 0
  br label %elx44
elx44:
  br label %join44
join44:
  %t44 = add i64 0, %t43
  %t45 = load i64, i64* %L4
  %t46 = load i64, i64* %L1
  %t47.c = icmp slt i64 %t45, %t46
  %t47 = zext i1 %t47.c to i64
  %t54.b = icmp ne i64 %t47, 0
  br i1 %t54.b, label %then54, label %else54
then54:
  %t48 = load i64, i64* %L3
  %t49 = add i64 0, 1
  %t50 = add i64 %t48, %t49
  %t51.mb = call i8* @malloc(i64 16)
  %t51.m = bitcast i8* %t51.mb to [2 x i64]*
  %t51.t = getelementptr [2 x i64], [2 x i64]* %t51.m, i64 0, i64 0
  store i64 1, i64* %t51.t
  %t51.p0 = getelementptr [2 x i64], [2 x i64]* %t51.m, i64 0, i64 1
  store i64 %t50, i64* %t51.p0
  %t51 = ptrtoint [2 x i64]* %t51.m to i64
  store i64 %t51, i64* %L5
  %t52 = load i64, i64* %L5
  ret i64 %t52
ar52:
  br label %thx54
thx54:
  br label %join54
else54:
  %t53 = add i64 0, 0
  br label %elx54
elx54:
  br label %join54
join54:
  %t54 = add i64 0, %t53
  %t55 = load i64, i64* %L3
  %t56.mb = call i8* @malloc(i64 16)
  %t56.m = bitcast i8* %t56.mb to [2 x i64]*
  %t56.t = getelementptr [2 x i64], [2 x i64]* %t56.m, i64 0, i64 0
  store i64 1, i64* %t56.t
  %t56.p0 = getelementptr [2 x i64], [2 x i64]* %t56.m, i64 0, i64 1
  store i64 %t55, i64* %t56.p0
  %t56 = ptrtoint [2 x i64]* %t56.m to i64
  ret i64 %t56
}
define i64 @opos(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = add i64 0, 0
  %t12 = add i64 0, 1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t18.b = icmp ne i64 %t13, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t14 = load i64, i64* %L3
  %t15.mb = call i8* @malloc(i64 16)
  %t15.m = bitcast i8* %t15.mb to [2 x i64]*
  %t15.t = getelementptr [2 x i64], [2 x i64]* %t15.m, i64 0, i64 0
  store i64 1, i64* %t15.t
  %t15.p0 = getelementptr [2 x i64], [2 x i64]* %t15.m, i64 0, i64 1
  store i64 %t14, i64* %t15.p0
  %t15 = ptrtoint [2 x i64]* %t15.m to i64
  store i64 %t15, i64* %L5
  %t16 = load i64, i64* %L5
  ret i64 %t16
ar16:
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = add i64 0, %t17
  %t19 = load i64, i64* %L3
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L3
  br label %lcond7
lend7:
  %t22 = add i64 0, 0
  %t23 = add i64 0, 0
  %t24.mb = call i8* @malloc(i64 16)
  %t24.m = bitcast i8* %t24.mb to [2 x i64]*
  %t24.t = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  store i64 0, i64* %t24.t
  %t24.p0 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 1
  store i64 %t23, i64* %t24.p0
  %t24 = ptrtoint [2 x i64]* %t24.m to i64
  ret i64 %t24
}
define i64 @vclone(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12 = call i64 @__vec_push(i64 %t8, i64 %t11)
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @ssplit_ws(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  br label %lcond18
lcond18:
  %t8 = load i64, i64* %L3
  %t9 = load i64, i64* %L2
  %t10.c = icmp slt i64 %t8, %t9
  %t10 = zext i1 %t10.c to i64
  %t18.b = icmp ne i64 %t10, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t11 = load i64, i64* %L0
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__str_at(i64 %t11, i64 %t12)
  %t14 = call i64 @sws(i64 %t13)
  %t15 = add i64 0, 1
  %t16.c = icmp eq i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t16, %thx18 ], [ %t17, %elx18 ]
  %lc18 = icmp ne i64 %t18, 0
  br i1 %lc18, label %lbody18, label %lend18
lbody18:
  %t19 = load i64, i64* %L3
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L3
  br label %lcond18
lend18:
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L3
  store i64 %t23, i64* %L4
  br label %lcond34
lcond34:
  %t24 = load i64, i64* %L3
  %t25 = load i64, i64* %L2
  %t26.c = icmp slt i64 %t24, %t25
  %t26 = zext i1 %t26.c to i64
  %t34.b = icmp ne i64 %t26, 0
  br i1 %t34.b, label %then34, label %else34
then34:
  %t27 = load i64, i64* %L0
  %t28 = load i64, i64* %L3
  %t29 = call i64 @__str_at(i64 %t27, i64 %t28)
  %t30 = call i64 @sws(i64 %t29)
  %t31 = add i64 0, 0
  %t32.c = icmp eq i64 %t30, %t31
  %t32 = zext i1 %t32.c to i64
  br label %thx34
thx34:
  br label %join34
else34:
  %t33 = add i64 0, 0
  br label %elx34
elx34:
  br label %join34
join34:
  %t34 = phi i64 [ %t32, %thx34 ], [ %t33, %elx34 ]
  %lc34 = icmp ne i64 %t34, 0
  br i1 %lc34, label %lbody34, label %lend34
lbody34:
  %t35 = load i64, i64* %L3
  %t36 = add i64 0, 1
  %t37 = add i64 %t35, %t36
  store i64 %t37, i64* %L3
  br label %lcond34
lend34:
  %t38 = add i64 0, 0
  %t39 = load i64, i64* %L3
  %t40 = load i64, i64* %L4
  %t41.c = icmp sgt i64 %t39, %t40
  %t41 = zext i1 %t41.c to i64
  %t51.b = icmp ne i64 %t41, 0
  br i1 %t51.b, label %then51, label %else51
then51:
  %t42 = load i64, i64* %L1
  %t43 = load i64, i64* %L0
  %t44 = load i64, i64* %L4
  %t45 = load i64, i64* %L3
  %t46 = load i64, i64* %L4
  %t47 = sub i64 %t45, %t46
  %t48 = call i64 @__str_sub(i64 %t43, i64 %t44, i64 %t47)
  %t49 = call i64 @__vec_push(i64 %t42, i64 %t48)
  br label %thx51
thx51:
  br label %join51
else51:
  %t50 = add i64 0, 0
  br label %elx51
elx51:
  br label %join51
join51:
  %t51 = phi i64 [ %t49, %thx51 ], [ %t50, %elx51 ]
  br label %lcond7
lend7:
  %t52 = add i64 0, 0
  %t53 = load i64, i64* %L1
  ret i64 %t53
}
define i64 @arr_vec(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__arr_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__arr_get(i64 %t9, i64 %t10)
  %t12 = call i64 @__vec_push(i64 %t8, i64 %t11)
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @strim_endc(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  br label %lcond15
lcond15:
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 0
  %t6.c = icmp sgt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t15.b = icmp ne i64 %t6, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L2
  %t9 = add i64 0, 1
  %t10 = sub i64 %t8, %t9
  %t11 = call i64 @__str_at(i64 %t7, i64 %t10)
  %t12 = load i64, i64* %L1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t13, %thx15 ], [ %t14, %elx15 ]
  %lc15 = icmp ne i64 %t15, 0
  br i1 %lc15, label %lbody15, label %lend15
lbody15:
  %t16 = load i64, i64* %L2
  %t17 = add i64 0, 1
  %t18 = sub i64 %t16, %t17
  store i64 %t18, i64* %L2
  br label %lcond15
lend15:
  %t19 = add i64 0, 0
  %t20 = load i64, i64* %L0
  %t21 = add i64 0, 0
  %t22 = load i64, i64* %L2
  %t23 = call i64 @__str_sub(i64 %t20, i64 %t21, i64 %t22)
  ret i64 %t23
}
define i64 @strim_startc(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond14
lcond14:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t14.b = icmp ne i64 %t7, 0
  br i1 %t14.b, label %then14, label %else14
then14:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__str_at(i64 %t8, i64 %t9)
  %t11 = load i64, i64* %L1
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  br label %thx14
thx14:
  br label %join14
else14:
  %t13 = add i64 0, 0
  br label %elx14
elx14:
  br label %join14
join14:
  %t14 = phi i64 [ %t12, %thx14 ], [ %t13, %elx14 ]
  %lc14 = icmp ne i64 %t14, 0
  br i1 %lc14, label %lbody14, label %lend14
lbody14:
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  br label %lcond14
lend14:
  %t18 = add i64 0, 0
  %t19 = load i64, i64* %L0
  %t20 = load i64, i64* %L3
  %t21 = load i64, i64* %L2
  %t22 = load i64, i64* %L3
  %t23 = sub i64 %t21, %t22
  %t24 = call i64 @__str_sub(i64 %t19, i64 %t20, i64 %t23)
  ret i64 %t24
}
define i64 @strim_c(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L1
  %t4 = call i64 @strim_endc(i64 %t2, i64 %t3)
  %t5 = load i64, i64* %L1
  %t6 = call i64 @strim_startc(i64 %t4, i64 %t5)
  ret i64 %t6
}
define i64 @str_char_idx(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__str_at(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L4
  %t12 = add i64 0, 128
  %t13.c = icmp slt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t120.b = icmp ne i64 %t13, 0
  br i1 %t120.b, label %then120, label %else120
then120:
  %t14 = load i64, i64* %L1
  %t15 = load i64, i64* %L3
  %t16 = load i64, i64* %L4
  %t17.mb = call i8* @malloc(i64 16)
  %t17.m = bitcast i8* %t17.mb to [2 x i64]*
  %t17.f0 = getelementptr [2 x i64], [2 x i64]* %t17.m, i64 0, i64 0
  store i64 %t15, i64* %t17.f0
  %t17.f1 = getelementptr [2 x i64], [2 x i64]* %t17.m, i64 0, i64 1
  store i64 %t16, i64* %t17.f1
  %t17.p = getelementptr [2 x i64], [2 x i64]* %t17.m, i64 0, i64 0
  %t17 = ptrtoint i64* %t17.p to i64   ; struct size=16
  %t18 = call i64 @__vec_push(i64 %t14, i64 %t17)
  %t19 = load i64, i64* %L3
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L3
  br label %thx120
thx120:
  br label %join120
else120:
  %t22 = load i64, i64* %L4
  %t23 = add i64 0, 224
  %t24.c = icmp slt i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  %t119.b = icmp ne i64 %t24, 0
  br i1 %t119.b, label %then119, label %else119
then119:
  %t25 = load i64, i64* %L1
  %t26 = load i64, i64* %L3
  %t27 = load i64, i64* %L4
  %t28 = add i64 0, 192
  %t29 = sub i64 %t27, %t28
  %t30 = add i64 0, 64
  %t31 = mul i64 %t29, %t30
  %t32 = load i64, i64* %L0
  %t33 = load i64, i64* %L3
  %t34 = add i64 0, 1
  %t35 = add i64 %t33, %t34
  %t36 = call i64 @__str_at(i64 %t32, i64 %t35)
  %t37 = add i64 0, 128
  %t38 = sub i64 %t36, %t37
  %t39 = add i64 %t31, %t38
  %t40.mb = call i8* @malloc(i64 16)
  %t40.m = bitcast i8* %t40.mb to [2 x i64]*
  %t40.f0 = getelementptr [2 x i64], [2 x i64]* %t40.m, i64 0, i64 0
  store i64 %t26, i64* %t40.f0
  %t40.f1 = getelementptr [2 x i64], [2 x i64]* %t40.m, i64 0, i64 1
  store i64 %t39, i64* %t40.f1
  %t40.p = getelementptr [2 x i64], [2 x i64]* %t40.m, i64 0, i64 0
  %t40 = ptrtoint i64* %t40.p to i64   ; struct size=16
  %t41 = call i64 @__vec_push(i64 %t25, i64 %t40)
  %t42 = load i64, i64* %L3
  %t43 = add i64 0, 2
  %t44 = add i64 %t42, %t43
  store i64 %t44, i64* %L3
  br label %thx119
thx119:
  br label %join119
else119:
  %t45 = load i64, i64* %L4
  %t46 = add i64 0, 240
  %t47.c = icmp slt i64 %t45, %t46
  %t47 = zext i1 %t47.c to i64
  %t118.b = icmp ne i64 %t47, 0
  br i1 %t118.b, label %then118, label %else118
then118:
  %t48 = load i64, i64* %L1
  %t49 = load i64, i64* %L3
  %t50 = load i64, i64* %L4
  %t51 = add i64 0, 224
  %t52 = sub i64 %t50, %t51
  %t53 = add i64 0, 64
  %t54 = mul i64 %t52, %t53
  %t55 = load i64, i64* %L0
  %t56 = load i64, i64* %L3
  %t57 = add i64 0, 1
  %t58 = add i64 %t56, %t57
  %t59 = call i64 @__str_at(i64 %t55, i64 %t58)
  %t60 = add i64 0, 128
  %t61 = sub i64 %t59, %t60
  %t62 = add i64 %t54, %t61
  %t63 = add i64 0, 64
  %t64 = mul i64 %t62, %t63
  %t65 = load i64, i64* %L0
  %t66 = load i64, i64* %L3
  %t67 = add i64 0, 2
  %t68 = add i64 %t66, %t67
  %t69 = call i64 @__str_at(i64 %t65, i64 %t68)
  %t70 = add i64 0, 128
  %t71 = sub i64 %t69, %t70
  %t72 = add i64 %t64, %t71
  %t73.mb = call i8* @malloc(i64 16)
  %t73.m = bitcast i8* %t73.mb to [2 x i64]*
  %t73.f0 = getelementptr [2 x i64], [2 x i64]* %t73.m, i64 0, i64 0
  store i64 %t49, i64* %t73.f0
  %t73.f1 = getelementptr [2 x i64], [2 x i64]* %t73.m, i64 0, i64 1
  store i64 %t72, i64* %t73.f1
  %t73.p = getelementptr [2 x i64], [2 x i64]* %t73.m, i64 0, i64 0
  %t73 = ptrtoint i64* %t73.p to i64   ; struct size=16
  %t74 = call i64 @__vec_push(i64 %t48, i64 %t73)
  %t75 = load i64, i64* %L3
  %t76 = add i64 0, 3
  %t77 = add i64 %t75, %t76
  store i64 %t77, i64* %L3
  br label %thx118
thx118:
  br label %join118
else118:
  %t78 = load i64, i64* %L1
  %t79 = load i64, i64* %L3
  %t80 = load i64, i64* %L4
  %t81 = add i64 0, 240
  %t82 = sub i64 %t80, %t81
  %t83 = add i64 0, 64
  %t84 = mul i64 %t82, %t83
  %t85 = load i64, i64* %L0
  %t86 = load i64, i64* %L3
  %t87 = add i64 0, 1
  %t88 = add i64 %t86, %t87
  %t89 = call i64 @__str_at(i64 %t85, i64 %t88)
  %t90 = add i64 0, 128
  %t91 = sub i64 %t89, %t90
  %t92 = add i64 %t84, %t91
  %t93 = add i64 0, 64
  %t94 = mul i64 %t92, %t93
  %t95 = load i64, i64* %L0
  %t96 = load i64, i64* %L3
  %t97 = add i64 0, 2
  %t98 = add i64 %t96, %t97
  %t99 = call i64 @__str_at(i64 %t95, i64 %t98)
  %t100 = add i64 0, 128
  %t101 = sub i64 %t99, %t100
  %t102 = add i64 %t94, %t101
  %t103 = add i64 0, 64
  %t104 = mul i64 %t102, %t103
  %t105 = load i64, i64* %L0
  %t106 = load i64, i64* %L3
  %t107 = add i64 0, 3
  %t108 = add i64 %t106, %t107
  %t109 = call i64 @__str_at(i64 %t105, i64 %t108)
  %t110 = add i64 0, 128
  %t111 = sub i64 %t109, %t110
  %t112 = add i64 %t104, %t111
  %t113.mb = call i8* @malloc(i64 16)
  %t113.m = bitcast i8* %t113.mb to [2 x i64]*
  %t113.f0 = getelementptr [2 x i64], [2 x i64]* %t113.m, i64 0, i64 0
  store i64 %t79, i64* %t113.f0
  %t113.f1 = getelementptr [2 x i64], [2 x i64]* %t113.m, i64 0, i64 1
  store i64 %t112, i64* %t113.f1
  %t113.p = getelementptr [2 x i64], [2 x i64]* %t113.m, i64 0, i64 0
  %t113 = ptrtoint i64* %t113.p to i64   ; struct size=16
  %t114 = call i64 @__vec_push(i64 %t78, i64 %t113)
  %t115 = load i64, i64* %L3
  %t116 = add i64 0, 4
  %t117 = add i64 %t115, %t116
  store i64 %t117, i64* %L3
  br label %elx118
elx118:
  br label %join118
join118:
  %t118 = phi i64 [ %t77, %thx118 ], [ %t117, %elx118 ]
  br label %elx119
elx119:
  br label %join119
join119:
  %t119 = phi i64 [ %t44, %thx119 ], [ %t118, %elx119 ]
  br label %elx120
elx120:
  br label %join120
join120:
  %t120 = phi i64 [ %t21, %thx120 ], [ %t119, %elx120 ]
  br label %lcond7
lend7:
  %t121 = add i64 0, 0
  %t122 = load i64, i64* %L1
  ret i64 %t122
}
define i64 @vwindows(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond10
lcond10:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L1
  %t8 = add i64 %t6, %t7
  %t9 = load i64, i64* %L3
  %t10.c = icmp sle i64 %t8, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = call i64 @__vec_new()
  store i64 %t11, i64* %L5
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L6
  br label %lcond15
lcond15:
  %t13 = load i64, i64* %L6
  %t14 = load i64, i64* %L1
  %t15.c = icmp slt i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  %lc15 = icmp ne i64 %t15, 0
  br i1 %lc15, label %lbody15, label %lend15
lbody15:
  %t16 = load i64, i64* %L5
  %t17 = load i64, i64* %L0
  %t18 = load i64, i64* %L4
  %t19 = load i64, i64* %L6
  %t20 = add i64 %t18, %t19
  %t21 = call i64 @__vec_get(i64 %t17, i64 %t20)
  %t22 = call i64 @__vec_push(i64 %t16, i64 %t21)
  %t23 = load i64, i64* %L6
  %t24 = add i64 0, 1
  %t25 = add i64 %t23, %t24
  store i64 %t25, i64* %L6
  br label %lcond15
lend15:
  %t26 = add i64 0, 0
  %t27 = load i64, i64* %L2
  %t28 = load i64, i64* %L5
  %t29 = call i64 @__vec_push(i64 %t27, i64 %t28)
  %t30 = load i64, i64* %L4
  %t31 = add i64 0, 1
  %t32 = add i64 %t30, %t31
  store i64 %t32, i64* %L4
  br label %lcond10
lend10:
  %t33 = add i64 0, 0
  %t34 = load i64, i64* %L2
  ret i64 %t34
}
define i64 @vchunks(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = call i64 @__vec_new()
  store i64 %t9, i64* %L5
  %t10 = add i64 0, 0
  store i64 %t10, i64* %L6
  br label %lcond20
lcond20:
  %t11 = load i64, i64* %L6
  %t12 = load i64, i64* %L1
  %t13.c = icmp slt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t20.b = icmp ne i64 %t13, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L6
  %t16 = add i64 %t14, %t15
  %t17 = load i64, i64* %L3
  %t18.c = icmp slt i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = phi i64 [ %t18, %thx20 ], [ %t19, %elx20 ]
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L5
  %t22 = load i64, i64* %L0
  %t23 = load i64, i64* %L4
  %t24 = load i64, i64* %L6
  %t25 = add i64 %t23, %t24
  %t26 = call i64 @__vec_get(i64 %t22, i64 %t25)
  %t27 = call i64 @__vec_push(i64 %t21, i64 %t26)
  %t28 = load i64, i64* %L6
  %t29 = add i64 0, 1
  %t30 = add i64 %t28, %t29
  store i64 %t30, i64* %L6
  br label %lcond20
lend20:
  %t31 = add i64 0, 0
  %t32 = load i64, i64* %L2
  %t33 = load i64, i64* %L5
  %t34 = call i64 @__vec_push(i64 %t32, i64 %t33)
  %t35 = load i64, i64* %L4
  %t36 = load i64, i64* %L1
  %t37 = add i64 %t35, %t36
  store i64 %t37, i64* %L4
  br label %lcond8
lend8:
  %t38 = add i64 0, 0
  %t39 = load i64, i64* %L2
  ret i64 %t39
}
define i64 @vsum_any(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, 0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12 = add i64 %t8, %t11
  store i64 %t12, i64* %L1
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @vsum_f(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2d = call double @strtod(i8* %t2p, i8** null)
  %t2 = bitcast double %t2d to i64
  store i64 %t2, i64* %L1
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L3
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13a = bitcast i64 %t9 to double
  %t13b = bitcast i64 %t12 to double
  %t13r = fadd double %t13a, %t13b
  %t13 = bitcast double %t13r to i64
  store i64 %t13, i64* %L1
  %t14 = load i64, i64* %L3
  %t15 = add i64 0, 1
  %t16 = add i64 %t14, %t15
  store i64 %t16, i64* %L3
  br label %lcond8
lend8:
  %t17 = add i64 0, 0
  %t18 = load i64, i64* %L1
  ret i64 %t18
}
define i64 @vproduct_f(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t2p = inttoptr i64 %t1 to i8*
  %t2d = call double @strtod(i8* %t2p, i8** null)
  %t2 = bitcast double %t2d to i64
  store i64 %t2, i64* %L1
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L3
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13a = bitcast i64 %t9 to double
  %t13b = bitcast i64 %t12 to double
  %t13r = fmul double %t13a, %t13b
  %t13 = bitcast double %t13r to i64
  store i64 %t13, i64* %L1
  %t14 = load i64, i64* %L3
  %t15 = add i64 0, 1
  %t16 = add i64 %t14, %t15
  store i64 %t16, i64* %L3
  br label %lcond8
lend8:
  %t17 = add i64 0, 0
  %t18 = load i64, i64* %L1
  ret i64 %t18
}
define i64 @hm_pairs(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3.bp = inttoptr i64 %t2 to i8*
  %t3.fp8 = getelementptr i8, i8* %t3.bp, i64 0
  %t3.fp = bitcast i8* %t3.fp8 to i64*
  %t3 = load i64, i64* %t3.fp
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L2
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L3
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L3
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L0
  %t11.bp = inttoptr i64 %t10 to i8*
  %t11.fp8 = getelementptr i8, i8* %t11.bp, i64 0
  %t11.fp = bitcast i8* %t11.fp8 to i64*
  %t11 = load i64, i64* %t11.fp
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  %t14 = load i64, i64* %L0
  %t15.bp = inttoptr i64 %t14 to i8*
  %t15.fp8 = getelementptr i8, i8* %t15.bp, i64 8
  %t15.fp = bitcast i8* %t15.fp8 to i64*
  %t15 = load i64, i64* %t15.fp
  %t16 = load i64, i64* %L3
  %t17 = call i64 @__vec_get(i64 %t15, i64 %t16)
  %t18.mb = call i8* @malloc(i64 16)
  %t18.m = bitcast i8* %t18.mb to [2 x i64]*
  %t18.f0 = getelementptr [2 x i64], [2 x i64]* %t18.m, i64 0, i64 0
  store i64 %t13, i64* %t18.f0
  %t18.f1 = getelementptr [2 x i64], [2 x i64]* %t18.m, i64 0, i64 1
  store i64 %t17, i64* %t18.f1
  %t18.p = getelementptr [2 x i64], [2 x i64]* %t18.m, i64 0, i64 0
  %t18 = ptrtoint i64* %t18.p to i64   ; struct size=16
  %t19 = call i64 @__vec_push(i64 %t9, i64 %t18)
  %t20 = load i64, i64* %L3
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L3
  br label %lcond8
lend8:
  %t23 = add i64 0, 0
  %t24 = load i64, i64* %L1
  ret i64 %t24
}
define i64 @t2_lt(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L2
  %t4 = add i64 0, 1
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t28.b = icmp ne i64 %t5, 0
  br i1 %t28.b, label %then28, label %else28
then28:
  %t6 = load i64, i64* %L0
  %t7.bp = inttoptr i64 %t6 to i8*
  %t7.fp8 = getelementptr i8, i8* %t7.bp, i64 0
  %t7.fp = bitcast i8* %t7.fp8 to i64*
  %t7 = load i64, i64* %t7.fp
  %t8 = load i64, i64* %L1
  %t9.bp = inttoptr i64 %t8 to i8*
  %t9.fp8 = getelementptr i8, i8* %t9.bp, i64 0
  %t9.fp = bitcast i8* %t9.fp8 to i64*
  %t9 = load i64, i64* %t9.fp
  %t10 = call i64 @scmp(i64 %t7, i64 %t9)
  br label %thx28
thx28:
  br label %join28
else28:
  %t11 = load i64, i64* %L0
  %t12.bp = inttoptr i64 %t11 to i8*
  %t12.fp8 = getelementptr i8, i8* %t12.bp, i64 0
  %t12.fp = bitcast i8* %t12.fp8 to i64*
  %t12 = load i64, i64* %t12.fp
  %t13 = load i64, i64* %L1
  %t14.bp = inttoptr i64 %t13 to i8*
  %t14.fp8 = getelementptr i8, i8* %t14.bp, i64 0
  %t14.fp = bitcast i8* %t14.fp8 to i64*
  %t14 = load i64, i64* %t14.fp
  %t15.c = icmp slt i64 %t12, %t14
  %t15 = zext i1 %t15.c to i64
  %t27.b = icmp ne i64 %t15, 0
  br i1 %t27.b, label %then27, label %else27
then27:
  %t16 = add i64 0, 0
  %t17 = add i64 0, 1
  %t18 = sub i64 %t16, %t17
  br label %thx27
thx27:
  br label %join27
else27:
  %t19 = load i64, i64* %L1
  %t20.bp = inttoptr i64 %t19 to i8*
  %t20.fp8 = getelementptr i8, i8* %t20.bp, i64 0
  %t20.fp = bitcast i8* %t20.fp8 to i64*
  %t20 = load i64, i64* %t20.fp
  %t21 = load i64, i64* %L0
  %t22.bp = inttoptr i64 %t21 to i8*
  %t22.fp8 = getelementptr i8, i8* %t22.bp, i64 0
  %t22.fp = bitcast i8* %t22.fp8 to i64*
  %t22 = load i64, i64* %t22.fp
  %t23.c = icmp slt i64 %t20, %t22
  %t23 = zext i1 %t23.c to i64
  %t26.b = icmp ne i64 %t23, 0
  br i1 %t26.b, label %then26, label %else26
then26:
  %t24 = add i64 0, 1
  br label %thx26
thx26:
  br label %join26
else26:
  %t25 = add i64 0, 0
  br label %elx26
elx26:
  br label %join26
join26:
  %t26 = phi i64 [ %t24, %thx26 ], [ %t25, %elx26 ]
  br label %elx27
elx27:
  br label %join27
join27:
  %t27 = phi i64 [ %t18, %thx27 ], [ %t26, %elx27 ]
  br label %elx28
elx28:
  br label %join28
join28:
  %t28 = phi i64 [ %t10, %thx28 ], [ %t27, %elx28 ]
  store i64 %t28, i64* %L3
  %t29 = load i64, i64* %L3
  %t30 = add i64 0, 0
  %t31.c = icmp ne i64 %t29, %t30
  %t31 = zext i1 %t31.c to i64
  %t37.b = icmp ne i64 %t31, 0
  br i1 %t37.b, label %then37, label %else37
then37:
  %t32 = load i64, i64* %L3
  %t33 = add i64 0, 0
  %t34.c = icmp slt i64 %t32, %t33
  %t34 = zext i1 %t34.c to i64
  store i64 %t34, i64* %L4
  %t35 = load i64, i64* %L4
  ret i64 %t35
ar35:
  br label %thx37
thx37:
  br label %join37
else37:
  %t36 = add i64 0, 0
  br label %elx37
elx37:
  br label %join37
join37:
  %t37 = add i64 0, %t36
  %t38 = load i64, i64* %L0
  %t39.bp = inttoptr i64 %t38 to i8*
  %t39.fp8 = getelementptr i8, i8* %t39.bp, i64 8
  %t39.fp = bitcast i8* %t39.fp8 to i64*
  %t39 = load i64, i64* %t39.fp
  %t40 = load i64, i64* %L1
  %t41.bp = inttoptr i64 %t40 to i8*
  %t41.fp8 = getelementptr i8, i8* %t41.bp, i64 8
  %t41.fp = bitcast i8* %t41.fp8 to i64*
  %t41 = load i64, i64* %t41.fp
  %t42.c = icmp slt i64 %t39, %t41
  %t42 = zext i1 %t42.c to i64
  ret i64 %t42
}
define i64 @vsort_t2(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 1
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L3
  store i64 %t11, i64* %L5
  br label %lcond24
lcond24:
  %t12 = load i64, i64* %L5
  %t13 = add i64 0, 0
  %t14.c = icmp sgt i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t24.b = icmp ne i64 %t14, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t15 = load i64, i64* %L4
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L5
  %t18 = add i64 0, 1
  %t19 = sub i64 %t17, %t18
  %t20 = call i64 @__vec_get(i64 %t16, i64 %t19)
  %t21 = load i64, i64* %L1
  %t22 = call i64 @t2_lt(i64 %t15, i64 %t20, i64 %t21)
  br label %thx24
thx24:
  br label %join24
else24:
  %t23 = add i64 0, 0
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t22, %thx24 ], [ %t23, %elx24 ]
  %lc24 = icmp ne i64 %t24, 0
  br i1 %lc24, label %lbody24, label %lend24
lbody24:
  %t25 = load i64, i64* %L0
  %t26 = load i64, i64* %L5
  %t27 = load i64, i64* %L0
  %t28 = load i64, i64* %L5
  %t29 = add i64 0, 1
  %t30 = sub i64 %t28, %t29
  %t31 = call i64 @__vec_get(i64 %t27, i64 %t30)
  %t32 = call i64 @__vec_set(i64 %t25, i64 %t26, i64 %t31)
  %t33 = load i64, i64* %L5
  %t34 = add i64 0, 1
  %t35 = sub i64 %t33, %t34
  store i64 %t35, i64* %L5
  br label %lcond24
lend24:
  %t36 = add i64 0, 0
  %t37 = load i64, i64* %L0
  %t38 = load i64, i64* %L5
  %t39 = load i64, i64* %L4
  %t40 = call i64 @__vec_set(i64 %t37, i64 %t38, i64 %t39)
  %t41 = load i64, i64* %L3
  %t42 = add i64 0, 1
  %t43 = add i64 %t41, %t42
  store i64 %t43, i64* %L3
  br label %lcond7
lend7:
  %t44 = add i64 0, 0
  %t45 = add i64 0, 0
  ret i64 %t45
}
define i64 @vsort_t2s(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 1
  %t3 = call i64 @vsort_t2(i64 %t1, i64 %t2)
  ret i64 %t3
}
define i64 @vsort_t2i(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3 = call i64 @vsort_t2(i64 %t1, i64 %t2)
  ret i64 %t3
}
define i64 @dbgq_s(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [2 x i8]* @.s0000000000151a27 to i64
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__str_at(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L4
  %t12 = add i64 0, 34
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t44.b = icmp ne i64 %t13, 0
  br i1 %t44.b, label %then44, label %else44
then44:
  %t14 = load i64, i64* %L1
  %t15 = ptrtoint [3 x i8]* @.s00000000152f7b83 to i64
  %t16 = call i64 @__str_cat(i64 %t14, i64 %t15)
  store i64 %t16, i64* %L1
  br label %thx44
thx44:
  br label %join44
else44:
  %t17 = load i64, i64* %L4
  %t18 = add i64 0, 92
  %t19.c = icmp eq i64 %t17, %t18
  %t19 = zext i1 %t19.c to i64
  %t43.b = icmp ne i64 %t19, 0
  br i1 %t43.b, label %then43, label %else43
then43:
  %t20 = load i64, i64* %L1
  %t21 = ptrtoint [3 x i8]* @.s00000000152f7bbd to i64
  %t22 = call i64 @__str_cat(i64 %t20, i64 %t21)
  store i64 %t22, i64* %L1
  br label %thx43
thx43:
  br label %join43
else43:
  %t23 = load i64, i64* %L4
  %t24 = add i64 0, 10
  %t25.c = icmp eq i64 %t23, %t24
  %t25 = zext i1 %t25.c to i64
  %t42.b = icmp ne i64 %t25, 0
  br i1 %t42.b, label %then42, label %else42
then42:
  %t26 = load i64, i64* %L1
  %t27 = ptrtoint [3 x i8]* @.s00000000152f7bcf to i64
  %t28 = call i64 @__str_cat(i64 %t26, i64 %t27)
  store i64 %t28, i64* %L1
  br label %thx42
thx42:
  br label %join42
else42:
  %t29 = load i64, i64* %L4
  %t30 = add i64 0, 9
  %t31.c = icmp eq i64 %t29, %t30
  %t31 = zext i1 %t31.c to i64
  %t41.b = icmp ne i64 %t31, 0
  br i1 %t41.b, label %then41, label %else41
then41:
  %t32 = load i64, i64* %L1
  %t33 = ptrtoint [3 x i8]* @.s00000000152f7bd5 to i64
  %t34 = call i64 @__str_cat(i64 %t32, i64 %t33)
  store i64 %t34, i64* %L1
  br label %thx41
thx41:
  br label %join41
else41:
  %t35 = load i64, i64* %L1
  %t36 = load i64, i64* %L0
  %t37 = load i64, i64* %L3
  %t38 = add i64 0, 1
  %t39 = call i64 @__str_sub(i64 %t36, i64 %t37, i64 %t38)
  %t40 = call i64 @__str_cat(i64 %t35, i64 %t39)
  store i64 %t40, i64* %L1
  br label %elx41
elx41:
  br label %join41
join41:
  %t41 = phi i64 [ %t34, %thx41 ], [ %t40, %elx41 ]
  br label %elx42
elx42:
  br label %join42
join42:
  %t42 = phi i64 [ %t28, %thx42 ], [ %t41, %elx42 ]
  br label %elx43
elx43:
  br label %join43
join43:
  %t43 = phi i64 [ %t22, %thx43 ], [ %t42, %elx43 ]
  br label %elx44
elx44:
  br label %join44
join44:
  %t44 = phi i64 [ %t16, %thx44 ], [ %t43, %elx44 ]
  %t45 = load i64, i64* %L3
  %t46 = add i64 0, 1
  %t47 = add i64 %t45, %t46
  store i64 %t47, i64* %L3
  br label %lcond7
lend7:
  %t48 = add i64 0, 0
  %t49 = load i64, i64* %L1
  %t50 = ptrtoint [2 x i8]* @.s0000000000151a27 to i64
  %t51 = call i64 @__str_cat(i64 %t49, i64 %t50)
  ret i64 %t51
}
define i64 @dbgc_s(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = add i64 0, 39
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t18.b = icmp ne i64 %t4, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t5 = ptrtoint [3 x i8]* @.s00000000152f7b88 to i64
  br label %thx18
thx18:
  br label %join18
else18:
  %t6 = load i64, i64* %L0
  %t7 = add i64 0, 92
  %t8.c = icmp eq i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t17.b = icmp ne i64 %t8, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  %t9 = ptrtoint [3 x i8]* @.s00000000152f7bbd to i64
  br label %thx17
thx17:
  br label %join17
else17:
  %t10 = load i64, i64* %L0
  %t11 = add i64 0, 10
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t16.b = icmp ne i64 %t12, 0
  br i1 %t16.b, label %then16, label %else16
then16:
  %t13 = ptrtoint [3 x i8]* @.s00000000152f7bcf to i64
  br label %thx16
thx16:
  br label %join16
else16:
  %t14 = load i64, i64* %L1
  %t15 = call i64 @__chrstr(i64 %t14)
  br label %elx16
elx16:
  br label %join16
join16:
  %t16 = phi i64 [ %t13, %thx16 ], [ %t15, %elx16 ]
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = phi i64 [ %t9, %thx17 ], [ %t16, %elx17 ]
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t5, %thx18 ], [ %t17, %elx18 ]
  store i64 %t18, i64* %L2
  %t19 = ptrtoint [2 x i8]* @.s0000000000151a2c to i64
  %t20 = load i64, i64* %L2
  %t21 = call i64 @__str_cat(i64 %t19, i64 %t20)
  %t22 = ptrtoint [2 x i8]* @.s0000000000151a2c to i64
  %t23 = call i64 @__str_cat(i64 %t21, i64 %t22)
  ret i64 %t23
}
define i64 @str_bytes_vec(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__str_at(i64 %t9, i64 %t10)
  %t12 = call i64 @__vec_push(i64 %t8, i64 %t11)
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @ofind(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = add i64 0, 0
  %t12 = add i64 0, 1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t20.b = icmp ne i64 %t13, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t14 = load i64, i64* %L0
  %t15 = load i64, i64* %L3
  %t16 = call i64 @__vec_get(i64 %t14, i64 %t15)
  %t17.mb = call i8* @malloc(i64 16)
  %t17.m = bitcast i8* %t17.mb to [2 x i64]*
  %t17.t = getelementptr [2 x i64], [2 x i64]* %t17.m, i64 0, i64 0
  store i64 1, i64* %t17.t
  %t17.p0 = getelementptr [2 x i64], [2 x i64]* %t17.m, i64 0, i64 1
  store i64 %t16, i64* %t17.p0
  %t17 = ptrtoint [2 x i64]* %t17.m to i64
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  %t21 = load i64, i64* %L3
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L3
  br label %lcond7
lend7:
  %t24 = add i64 0, 0
  %t25 = add i64 0, 0
  %t26.mb = call i8* @malloc(i64 16)
  %t26.m = bitcast i8* %t26.mb to [2 x i64]*
  %t26.t = getelementptr [2 x i64], [2 x i64]* %t26.m, i64 0, i64 0
  store i64 0, i64* %t26.t
  %t26.p0 = getelementptr [2 x i64], [2 x i64]* %t26.m, i64 0, i64 1
  store i64 %t25, i64* %t26.p0
  %t26 = ptrtoint [2 x i64]* %t26.m to i64
  ret i64 %t26
}
define i64 @opart_t(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  store i64 %t3, i64* %L3
  %t4 = add i64 0, -1
  store i64 %t4, i64* %L4
  br label %lcond11
lcond11:
  %t5 = load i64, i64* %L4
  %t6 = add i64 0, 1
  %t7 = add i64 %t5, %t6
  store i64 %t7, i64* %L4
  %t8 = load i64, i64* %L4
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_len(i64 %t9)
  %t11.c = icmp slt i64 %t8, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L3
  %t13 = load i64, i64* %L4
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L5
  %t15 = load i64, i64* %L5
  store i64 %t15, i64* %L6
  %t16 = add i64 0, 0
  %t17 = add i64 0, 1
  %t18.c = icmp eq i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %t23.b = icmp ne i64 %t18, 0
  br i1 %t23.b, label %then23, label %else23
then23:
  %t19 = load i64, i64* %L2
  %t20 = load i64, i64* %L5
  %t21 = call i64 @__vec_push(i64 %t19, i64 %t20)
  br label %thx23
thx23:
  br label %join23
else23:
  %t22 = add i64 0, 0
  br label %elx23
elx23:
  br label %join23
join23:
  %t23 = phi i64 [ %t21, %thx23 ], [ %t22, %elx23 ]
  br label %lcond11
lend11:
  %t24 = add i64 0, 0
  %t25 = load i64, i64* %L2
  ret i64 %t25
}
define i64 @opart_g(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  store i64 %t3, i64* %L3
  %t4 = add i64 0, -1
  store i64 %t4, i64* %L4
  br label %lcond11
lcond11:
  %t5 = load i64, i64* %L4
  %t6 = add i64 0, 1
  %t7 = add i64 %t5, %t6
  store i64 %t7, i64* %L4
  %t8 = load i64, i64* %L4
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_len(i64 %t9)
  %t11.c = icmp slt i64 %t8, %t10
  %t11 = zext i1 %t11.c to i64
  %lc11 = icmp ne i64 %t11, 0
  br i1 %lc11, label %lbody11, label %lend11
lbody11:
  %t12 = load i64, i64* %L3
  %t13 = load i64, i64* %L4
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L5
  %t15 = load i64, i64* %L5
  store i64 %t15, i64* %L6
  %t16 = add i64 0, 0
  %t17 = add i64 0, 1
  %t18.c = icmp eq i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %t23.b = icmp ne i64 %t18, 0
  br i1 %t23.b, label %then23, label %else23
then23:
  %t19 = add i64 0, 0
  br label %thx23
thx23:
  br label %join23
else23:
  %t20 = load i64, i64* %L2
  %t21 = load i64, i64* %L5
  %t22 = call i64 @__vec_push(i64 %t20, i64 %t21)
  br label %elx23
elx23:
  br label %join23
join23:
  %t23 = phi i64 [ %t19, %thx23 ], [ %t22, %elx23 ]
  br label %lcond11
lend11:
  %t24 = add i64 0, 0
  %t25 = load i64, i64* %L2
  ret i64 %t25
}
define i64 @vstepby(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L4
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13 = call i64 @__vec_push(i64 %t9, i64 %t12)
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L1
  %t16 = add i64 %t14, %t15
  store i64 %t16, i64* %L4
  br label %lcond8
lend8:
  %t17 = add i64 0, 0
  %t18 = load i64, i64* %L2
  ret i64 %t18
}
define i64 @vretain(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L2
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L4
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  store i64 %t11, i64* %L5
  %t12 = add i64 0, 0
  %t13 = add i64 0, 1
  %t14.c = icmp eq i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t25.b = icmp ne i64 %t14, 0
  br i1 %t25.b, label %then25, label %else25
then25:
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L3
  %t17 = load i64, i64* %L0
  %t18 = load i64, i64* %L4
  %t19 = call i64 @__vec_get(i64 %t17, i64 %t18)
  %t20 = call i64 @__vec_set(i64 %t15, i64 %t16, i64 %t19)
  %t21 = load i64, i64* %L3
  %t22 = add i64 0, 1
  %t23 = add i64 %t21, %t22
  store i64 %t23, i64* %L3
  br label %thx25
thx25:
  br label %join25
else25:
  %t24 = add i64 0, 0
  br label %elx25
elx25:
  br label %join25
join25:
  %t25 = phi i64 [ %t23, %thx25 ], [ %t24, %elx25 ]
  %t26 = load i64, i64* %L4
  %t27 = add i64 0, 1
  %t28 = add i64 %t26, %t27
  store i64 %t28, i64* %L4
  br label %lcond8
lend8:
  %t29 = add i64 0, 0
  br label %lcond33
lcond33:
  %t30 = load i64, i64* %L0
  %t31 = call i64 @__vec_len(i64 %t30)
  %t32 = load i64, i64* %L3
  %t33.c = icmp sgt i64 %t31, %t32
  %t33 = zext i1 %t33.c to i64
  %lc33 = icmp ne i64 %t33, 0
  br i1 %lc33, label %lbody33, label %lend33
lbody33:
  %t34 = load i64, i64* %L0
  %t35 = call i64 @vpop(i64 %t34)
  store i64 %t35, i64* %L6
  %t36 = add i64 0, 0
  br label %lcond33
lend33:
  %t37 = add i64 0, 0
  %t38 = add i64 0, 0
  ret i64 %t38
}
define i64 @vsort(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  store i64 %t2, i64* %L1
  %t3 = add i64 0, 1
  store i64 %t3, i64* %L2
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L2
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__vec_get(i64 %t7, i64 %t8)
  store i64 %t9, i64* %L3
  %t10 = load i64, i64* %L2
  store i64 %t10, i64* %L4
  br label %lcond22
lcond22:
  %t11 = load i64, i64* %L4
  %t12 = add i64 0, 0
  %t13.c = icmp sgt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t22.b = icmp ne i64 %t13, 0
  br i1 %t22.b, label %then22, label %else22
then22:
  %t14 = load i64, i64* %L0
  %t15 = load i64, i64* %L4
  %t16 = add i64 0, 1
  %t17 = sub i64 %t15, %t16
  %t18 = call i64 @__vec_get(i64 %t14, i64 %t17)
  %t19 = load i64, i64* %L3
  %t20.c = icmp sgt i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  br label %thx22
thx22:
  br label %join22
else22:
  %t21 = add i64 0, 0
  br label %elx22
elx22:
  br label %join22
join22:
  %t22 = phi i64 [ %t20, %thx22 ], [ %t21, %elx22 ]
  %lc22 = icmp ne i64 %t22, 0
  br i1 %lc22, label %lbody22, label %lend22
lbody22:
  %t23 = load i64, i64* %L0
  %t24 = load i64, i64* %L4
  %t25 = load i64, i64* %L0
  %t26 = load i64, i64* %L4
  %t27 = add i64 0, 1
  %t28 = sub i64 %t26, %t27
  %t29 = call i64 @__vec_get(i64 %t25, i64 %t28)
  %t30 = call i64 @__vec_set(i64 %t23, i64 %t24, i64 %t29)
  %t31 = load i64, i64* %L4
  %t32 = add i64 0, 1
  %t33 = sub i64 %t31, %t32
  store i64 %t33, i64* %L4
  br label %lcond22
lend22:
  %t34 = add i64 0, 0
  %t35 = load i64, i64* %L0
  %t36 = load i64, i64* %L4
  %t37 = load i64, i64* %L3
  %t38 = call i64 @__vec_set(i64 %t35, i64 %t36, i64 %t37)
  %t39 = load i64, i64* %L2
  %t40 = add i64 0, 1
  %t41 = add i64 %t39, %t40
  store i64 %t41, i64* %L2
  br label %lcond6
lend6:
  %t42 = add i64 0, 0
  %t43 = add i64 0, 0
  ret i64 %t43
}
define i64 @vreverse(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  store i64 %t2, i64* %L1
  %t3 = add i64 0, 0
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L1
  store i64 %t4, i64* %L3
  br label %lcond9
lcond9:
  %t5 = load i64, i64* %L2
  %t6 = add i64 0, 1
  %t7 = add i64 %t5, %t6
  %t8 = load i64, i64* %L3
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %lc9 = icmp ne i64 %t9, 0
  br i1 %lc9, label %lbody9, label %lend9
lbody9:
  %t10 = load i64, i64* %L3
  %t11 = add i64 0, 1
  %t12 = sub i64 %t10, %t11
  store i64 %t12, i64* %L3
  %t13 = load i64, i64* %L0
  %t14 = load i64, i64* %L2
  %t15 = call i64 @__vec_get(i64 %t13, i64 %t14)
  store i64 %t15, i64* %L4
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L2
  %t18 = load i64, i64* %L0
  %t19 = load i64, i64* %L3
  %t20 = call i64 @__vec_get(i64 %t18, i64 %t19)
  %t21 = call i64 @__vec_set(i64 %t16, i64 %t17, i64 %t20)
  %t22 = load i64, i64* %L0
  %t23 = load i64, i64* %L3
  %t24 = load i64, i64* %L4
  %t25 = call i64 @__vec_set(i64 %t22, i64 %t23, i64 %t24)
  %t26 = load i64, i64* %L2
  %t27 = add i64 0, 1
  %t28 = add i64 %t26, %t27
  store i64 %t28, i64* %L2
  br label %lcond9
lend9:
  %t29 = add i64 0, 0
  %t30 = add i64 0, 0
  ret i64 %t30
}
define i64 @vswap(i64 %p0, i64 %p1, i64 %p2) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = add i64 0, %p2
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = load i64, i64* %L1
  %t5 = call i64 @__vec_get(i64 %t3, i64 %t4)
  store i64 %t5, i64* %L3
  %t6 = load i64, i64* %L0
  %t7 = load i64, i64* %L1
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L2
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  %t11 = call i64 @__vec_set(i64 %t6, i64 %t7, i64 %t10)
  %t12 = load i64, i64* %L0
  %t13 = load i64, i64* %L2
  %t14 = load i64, i64* %L3
  %t15 = call i64 @__vec_set(i64 %t12, i64 %t13, i64 %t14)
  %t16 = add i64 0, 0
  ret i64 %t16
}
define i64 @vtruncate(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  br label %lcond5
lcond5:
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  %t4 = load i64, i64* %L1
  %t5.c = icmp sgt i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %lc5 = icmp ne i64 %t5, 0
  br i1 %lc5, label %lbody5, label %lend5
lbody5:
  %t6 = load i64, i64* %L0
  %t7 = call i64 @vpop(i64 %t6)
  store i64 %t7, i64* %L2
  %t8 = add i64 0, 0
  br label %lcond5
lend5:
  %t9 = add i64 0, 0
  %t10 = add i64 0, 0
  ret i64 %t10
}
define i64 @vdedup(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  store i64 %t2, i64* %L1
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 0
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t9.b = icmp ne i64 %t5, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t6 = add i64 0, 0
  store i64 %t6, i64* %L2
  %t7 = load i64, i64* %L2
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = add i64 0, 1
  store i64 %t10, i64* %L2
  %t11 = add i64 0, 1
  store i64 %t11, i64* %L3
  br label %lcond14
lcond14:
  %t12 = load i64, i64* %L3
  %t13 = load i64, i64* %L1
  %t14.c = icmp slt i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %lc14 = icmp ne i64 %t14, 0
  br i1 %lc14, label %lbody14, label %lend14
lbody14:
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L3
  %t17 = call i64 @__vec_get(i64 %t15, i64 %t16)
  %t18 = load i64, i64* %L0
  %t19 = load i64, i64* %L2
  %t20 = add i64 0, 1
  %t21 = sub i64 %t19, %t20
  %t22 = call i64 @__vec_get(i64 %t18, i64 %t21)
  %t23.c = icmp ne i64 %t17, %t22
  %t23 = zext i1 %t23.c to i64
  %t34.b = icmp ne i64 %t23, 0
  br i1 %t34.b, label %then34, label %else34
then34:
  %t24 = load i64, i64* %L0
  %t25 = load i64, i64* %L2
  %t26 = load i64, i64* %L0
  %t27 = load i64, i64* %L3
  %t28 = call i64 @__vec_get(i64 %t26, i64 %t27)
  %t29 = call i64 @__vec_set(i64 %t24, i64 %t25, i64 %t28)
  %t30 = load i64, i64* %L2
  %t31 = add i64 0, 1
  %t32 = add i64 %t30, %t31
  store i64 %t32, i64* %L2
  br label %thx34
thx34:
  br label %join34
else34:
  %t33 = add i64 0, 0
  br label %elx34
elx34:
  br label %join34
join34:
  %t34 = phi i64 [ %t32, %thx34 ], [ %t33, %elx34 ]
  %t35 = load i64, i64* %L3
  %t36 = add i64 0, 1
  %t37 = add i64 %t35, %t36
  store i64 %t37, i64* %L3
  br label %lcond14
lend14:
  %t38 = add i64 0, 0
  br label %lcond42
lcond42:
  %t39 = load i64, i64* %L0
  %t40 = call i64 @__vec_len(i64 %t39)
  %t41 = load i64, i64* %L2
  %t42.c = icmp sgt i64 %t40, %t41
  %t42 = zext i1 %t42.c to i64
  %lc42 = icmp ne i64 %t42, 0
  br i1 %lc42, label %lbody42, label %lend42
lbody42:
  %t43 = load i64, i64* %L0
  %t44 = call i64 @vpop(i64 %t43)
  store i64 %t44, i64* %L4
  %t45 = add i64 0, 0
  br label %lcond42
lend42:
  %t46 = add i64 0, 0
  %t47 = add i64 0, 0
  ret i64 %t47
}
define i64 @vappend(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12 = call i64 @__vec_push(i64 %t8, i64 %t11)
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  br label %lcond20
lcond20:
  %t17 = load i64, i64* %L1
  %t18 = call i64 @__vec_len(i64 %t17)
  %t19 = add i64 0, 0
  %t20.c = icmp sgt i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L1
  %t22 = call i64 @vpop(i64 %t21)
  store i64 %t22, i64* %L4
  %t23 = add i64 0, 0
  br label %lcond20
lend20:
  %t24 = add i64 0, 0
  %t25 = add i64 0, 0
  ret i64 %t25
}
define i64 @vsortbykey(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 1
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L4
  store i64 %t11, i64* %L5
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L6
  %t13 = load i64, i64* %L3
  store i64 %t13, i64* %L7
  br label %lcond26
lcond26:
  %t14 = load i64, i64* %L7
  %t15 = add i64 0, 0
  %t16.c = icmp sgt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t26.b = icmp ne i64 %t16, 0
  br i1 %t26.b, label %then26, label %else26
then26:
  %t17 = load i64, i64* %L0
  %t18 = load i64, i64* %L7
  %t19 = add i64 0, 1
  %t20 = sub i64 %t18, %t19
  %t21 = call i64 @__vec_get(i64 %t17, i64 %t20)
  store i64 %t21, i64* %L8
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L6
  %t24.c = icmp sgt i64 %t22, %t23
  %t24 = zext i1 %t24.c to i64
  br label %thx26
thx26:
  br label %join26
else26:
  %t25 = add i64 0, 0
  br label %elx26
elx26:
  br label %join26
join26:
  %t26 = phi i64 [ %t24, %thx26 ], [ %t25, %elx26 ]
  %lc26 = icmp ne i64 %t26, 0
  br i1 %lc26, label %lbody26, label %lend26
lbody26:
  %t27 = load i64, i64* %L0
  %t28 = load i64, i64* %L7
  %t29 = load i64, i64* %L0
  %t30 = load i64, i64* %L7
  %t31 = add i64 0, 1
  %t32 = sub i64 %t30, %t31
  %t33 = call i64 @__vec_get(i64 %t29, i64 %t32)
  %t34 = call i64 @__vec_set(i64 %t27, i64 %t28, i64 %t33)
  %t35 = load i64, i64* %L7
  %t36 = add i64 0, 1
  %t37 = sub i64 %t35, %t36
  store i64 %t37, i64* %L7
  br label %lcond26
lend26:
  %t38 = add i64 0, 0
  %t39 = load i64, i64* %L0
  %t40 = load i64, i64* %L7
  %t41 = load i64, i64* %L4
  %t42 = call i64 @__vec_set(i64 %t39, i64 %t40, i64 %t41)
  %t43 = load i64, i64* %L3
  %t44 = add i64 0, 1
  %t45 = add i64 %t43, %t44
  store i64 %t45, i64* %L3
  br label %lcond7
lend7:
  %t46 = add i64 0, 0
  %t47 = add i64 0, 0
  ret i64 %t47
}
define i64 @vmaxbykey(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %L9 = alloca i64
  store i64 0, i64* %L9
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 0
  %t6.c = icmp eq i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t11.b = icmp ne i64 %t6, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t7 = add i64 0, 0
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 0, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = load i64, i64* %L0
  %t13 = add i64 0, 0
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L3
  %t15 = load i64, i64* %L3
  store i64 %t15, i64* %L4
  %t16 = add i64 0, 0
  store i64 %t16, i64* %L5
  %t17 = add i64 0, 1
  store i64 %t17, i64* %L6
  br label %lcond20
lcond20:
  %t18 = load i64, i64* %L6
  %t19 = load i64, i64* %L2
  %t20.c = icmp slt i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L0
  %t22 = load i64, i64* %L6
  %t23 = call i64 @__vec_get(i64 %t21, i64 %t22)
  store i64 %t23, i64* %L7
  %t24 = load i64, i64* %L7
  store i64 %t24, i64* %L8
  %t25 = add i64 0, 0
  store i64 %t25, i64* %L9
  %t26 = load i64, i64* %L9
  %t27 = load i64, i64* %L5
  %t28.c = icmp sge i64 %t26, %t27
  %t28 = zext i1 %t28.c to i64
  %t32.b = icmp ne i64 %t28, 0
  br i1 %t32.b, label %then32, label %else32
then32:
  %t29 = load i64, i64* %L9
  store i64 %t29, i64* %L5
  %t30 = load i64, i64* %L7
  store i64 %t30, i64* %L3
  br label %thx32
thx32:
  br label %join32
else32:
  %t31 = add i64 0, 0
  br label %elx32
elx32:
  br label %join32
join32:
  %t32 = phi i64 [ %t30, %thx32 ], [ %t31, %elx32 ]
  %t33 = load i64, i64* %L6
  %t34 = add i64 0, 1
  %t35 = add i64 %t33, %t34
  store i64 %t35, i64* %L6
  br label %lcond20
lend20:
  %t36 = add i64 0, 0
  %t37 = load i64, i64* %L3
  %t38.mb = call i8* @malloc(i64 16)
  %t38.m = bitcast i8* %t38.mb to [2 x i64]*
  %t38.t = getelementptr [2 x i64], [2 x i64]* %t38.m, i64 0, i64 0
  store i64 1, i64* %t38.t
  %t38.p0 = getelementptr [2 x i64], [2 x i64]* %t38.m, i64 0, i64 1
  store i64 %t37, i64* %t38.p0
  %t38 = ptrtoint [2 x i64]* %t38.m to i64
  ret i64 %t38
}
define i64 @vminbykey(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %L9 = alloca i64
  store i64 0, i64* %L9
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 0
  %t6.c = icmp eq i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t11.b = icmp ne i64 %t6, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t7 = add i64 0, 0
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 0, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = load i64, i64* %L0
  %t13 = add i64 0, 0
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L3
  %t15 = load i64, i64* %L3
  store i64 %t15, i64* %L4
  %t16 = add i64 0, 0
  store i64 %t16, i64* %L5
  %t17 = add i64 0, 1
  store i64 %t17, i64* %L6
  br label %lcond20
lcond20:
  %t18 = load i64, i64* %L6
  %t19 = load i64, i64* %L2
  %t20.c = icmp slt i64 %t18, %t19
  %t20 = zext i1 %t20.c to i64
  %lc20 = icmp ne i64 %t20, 0
  br i1 %lc20, label %lbody20, label %lend20
lbody20:
  %t21 = load i64, i64* %L0
  %t22 = load i64, i64* %L6
  %t23 = call i64 @__vec_get(i64 %t21, i64 %t22)
  store i64 %t23, i64* %L7
  %t24 = load i64, i64* %L7
  store i64 %t24, i64* %L8
  %t25 = add i64 0, 0
  store i64 %t25, i64* %L9
  %t26 = load i64, i64* %L9
  %t27 = load i64, i64* %L5
  %t28.c = icmp slt i64 %t26, %t27
  %t28 = zext i1 %t28.c to i64
  %t32.b = icmp ne i64 %t28, 0
  br i1 %t32.b, label %then32, label %else32
then32:
  %t29 = load i64, i64* %L9
  store i64 %t29, i64* %L5
  %t30 = load i64, i64* %L7
  store i64 %t30, i64* %L3
  br label %thx32
thx32:
  br label %join32
else32:
  %t31 = add i64 0, 0
  br label %elx32
elx32:
  br label %join32
join32:
  %t32 = phi i64 [ %t30, %thx32 ], [ %t31, %elx32 ]
  %t33 = load i64, i64* %L6
  %t34 = add i64 0, 1
  %t35 = add i64 %t33, %t34
  store i64 %t35, i64* %L6
  br label %lcond20
lend20:
  %t36 = add i64 0, 0
  %t37 = load i64, i64* %L3
  %t38.mb = call i8* @malloc(i64 16)
  %t38.m = bitcast i8* %t38.mb to [2 x i64]*
  %t38.t = getelementptr [2 x i64], [2 x i64]* %t38.m, i64 0, i64 0
  store i64 1, i64* %t38.t
  %t38.p0 = getelementptr [2 x i64], [2 x i64]* %t38.m, i64 0, i64 1
  store i64 %t37, i64* %t38.p0
  %t38 = ptrtoint [2 x i64]* %t38.m to i64
  ret i64 %t38
}
define i64 @vextend(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L3
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  %t12 = call i64 @__vec_push(i64 %t8, i64 %t11)
  %t13 = load i64, i64* %L3
  %t14 = add i64 0, 1
  %t15 = add i64 %t13, %t14
  store i64 %t15, i64* %L3
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = add i64 0, 0
  ret i64 %t17
}
define i64 @ounwrap_or_default(i64 %p0) {
entry:
  %t3.slot = alloca i64
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3.bp = inttoptr i64 %t2 to i64*
  %t3.tg = load i64, i64* %t3.bp
  br label %m3a0
m3a0:
  %t3.c0 = icmp eq i64 %t3.tg, 1
  br i1 %t3.c0, label %m3a0b, label %m3a1
m3a0b:
  %t3.g0_0 = getelementptr i64, i64* %t3.bp, i64 1
  %t3.v0_0 = load i64, i64* %t3.g0_0
  store i64 %t3.v0_0, i64* %L2
  %t4 = load i64, i64* %L2
  store i64 %t4, i64* %t3.slot
  br label %m3end
m3a1:
  %t3.c1 = icmp eq i64 %t3.tg, 0
  br i1 %t3.c1, label %m3a1b, label %m3dflt
m3a1b:
  %t5 = add i64 0, 0
  store i64 %t5, i64* %t3.slot
  br label %m3end
m3dflt:
  store i64 0, i64* %t3.slot
  br label %m3end
m3end:
  %t3 = load i64, i64* %t3.slot
  ret i64 %t3
}
define i64 @vclear(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  br label %lcond4
lcond4:
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp sgt i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %lc4 = icmp ne i64 %t4, 0
  br i1 %lc4, label %lbody4, label %lend4
lbody4:
  %t5 = load i64, i64* %L0
  %t6 = call i64 @vpop(i64 %t5)
  store i64 %t6, i64* %L1
  %t7 = add i64 0, 0
  br label %lcond4
lend4:
  %t8 = add i64 0, 0
  %t9 = add i64 0, 0
  ret i64 %t9
}
define i64 @vswapremove(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L0
  %t5 = load i64, i64* %L1
  %t6 = call i64 @__vec_get(i64 %t4, i64 %t5)
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L0
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L2
  %t11 = add i64 0, 1
  %t12 = sub i64 %t10, %t11
  %t13 = call i64 @__vec_get(i64 %t9, i64 %t12)
  %t14 = call i64 @__vec_set(i64 %t7, i64 %t8, i64 %t13)
  %t15 = load i64, i64* %L0
  %t16 = call i64 @vpop(i64 %t15)
  store i64 %t16, i64* %L4
  %t17 = load i64, i64* %L3
  ret i64 %t17
}
define i64 @vnth(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  %t5.c = icmp slt i64 %t2, %t4
  %t5 = zext i1 %t5.c to i64
  %t12.b = icmp ne i64 %t5, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t6 = load i64, i64* %L0
  %t7 = load i64, i64* %L1
  %t8 = call i64 @__vec_get(i64 %t6, i64 %t7)
  %t9.mb = call i8* @malloc(i64 16)
  %t9.m = bitcast i8* %t9.mb to [2 x i64]*
  %t9.t = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 0
  store i64 1, i64* %t9.t
  %t9.p0 = getelementptr [2 x i64], [2 x i64]* %t9.m, i64 0, i64 1
  store i64 %t8, i64* %t9.p0
  %t9 = ptrtoint [2 x i64]* %t9.m to i64
  store i64 %t9, i64* %L2
  %t10 = load i64, i64* %L2
  ret i64 %t10
ar10:
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = add i64 0, %t11
  %t13 = add i64 0, 0
  %t14.mb = call i8* @malloc(i64 16)
  %t14.m = bitcast i8* %t14.mb to [2 x i64]*
  %t14.t = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 0
  store i64 0, i64* %t14.t
  %t14.p0 = getelementptr [2 x i64], [2 x i64]* %t14.m, i64 0, i64 1
  store i64 %t13, i64* %t14.p0
  %t14 = ptrtoint [2 x i64]* %t14.m to i64
  ret i64 %t14
}
define i64 @vreduce(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  %t5 = add i64 0, 0
  %t6.c = icmp eq i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t11.b = icmp ne i64 %t6, 0
  br i1 %t11.b, label %then11, label %else11
then11:
  %t7 = add i64 0, 0
  %t8.mb = call i8* @malloc(i64 16)
  %t8.m = bitcast i8* %t8.mb to [2 x i64]*
  %t8.t = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 0
  store i64 0, i64* %t8.t
  %t8.p0 = getelementptr [2 x i64], [2 x i64]* %t8.m, i64 0, i64 1
  store i64 %t7, i64* %t8.p0
  %t8 = ptrtoint [2 x i64]* %t8.m to i64
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  ret i64 %t9
ar9:
  br label %thx11
thx11:
  br label %join11
else11:
  %t10 = add i64 0, 0
  br label %elx11
elx11:
  br label %join11
join11:
  %t11 = add i64 0, %t10
  %t12 = load i64, i64* %L0
  %t13 = add i64 0, 0
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  store i64 %t14, i64* %L3
  %t15 = add i64 0, 1
  store i64 %t15, i64* %L4
  br label %lcond18
lcond18:
  %t16 = load i64, i64* %L4
  %t17 = load i64, i64* %L2
  %t18.c = icmp slt i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %lc18 = icmp ne i64 %t18, 0
  br i1 %lc18, label %lbody18, label %lend18
lbody18:
  %t19 = load i64, i64* %L3
  store i64 %t19, i64* %L5
  %t20 = load i64, i64* %L0
  %t21 = load i64, i64* %L4
  %t22 = call i64 @__vec_get(i64 %t20, i64 %t21)
  store i64 %t22, i64* %L6
  %t23 = add i64 0, 0
  store i64 %t23, i64* %L3
  %t24 = load i64, i64* %L4
  %t25 = add i64 0, 1
  %t26 = add i64 %t24, %t25
  store i64 %t26, i64* %L4
  br label %lcond18
lend18:
  %t27 = add i64 0, 0
  %t28 = load i64, i64* %L3
  %t29.mb = call i8* @malloc(i64 16)
  %t29.m = bitcast i8* %t29.mb to [2 x i64]*
  %t29.t = getelementptr [2 x i64], [2 x i64]* %t29.m, i64 0, i64 0
  store i64 1, i64* %t29.t
  %t29.p0 = getelementptr [2 x i64], [2 x i64]* %t29.m, i64 0, i64 1
  store i64 %t28, i64* %t29.p0
  %t29 = ptrtoint [2 x i64]* %t29.m to i64
  ret i64 %t29
}
define i64 @fpowi(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L1
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L3
  %t6 = add i64 0, 0
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %t12.b = icmp ne i64 %t7, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t8 = add i64 0, 0
  %t9 = load i64, i64* %L3
  %t10 = sub i64 %t8, %t9
  store i64 %t10, i64* %L3
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = add i64 0, 0
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = phi i64 [ %t10, %thx12 ], [ %t11, %elx12 ]
  %t13 = add i64 0, 0
  store i64 %t13, i64* %L4
  br label %lcond16
lcond16:
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L3
  %t16.c = icmp slt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %lc16 = icmp ne i64 %t16, 0
  br i1 %lc16, label %lbody16, label %lend16
lbody16:
  %t17 = load i64, i64* %L2
  %t18 = load i64, i64* %L0
  %t19a = bitcast i64 %t17 to double
  %t19b = bitcast i64 %t18 to double
  %t19r = fmul double %t19a, %t19b
  %t19 = bitcast double %t19r to i64
  store i64 %t19, i64* %L2
  %t20 = load i64, i64* %L4
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L4
  br label %lcond16
lend16:
  %t23 = add i64 0, 0
  %t24 = load i64, i64* %L1
  %t25 = add i64 0, 0
  %t26.c = icmp slt i64 %t24, %t25
  %t26 = zext i1 %t26.c to i64
  %t32.b = icmp ne i64 %t26, 0
  br i1 %t32.b, label %then32, label %else32
then32:
  %t27 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t28p = inttoptr i64 %t27 to i8*
  %t28d = call double @strtod(i8* %t28p, i8** null)
  %t28 = bitcast double %t28d to i64
  %t29 = load i64, i64* %L2
  %t30a = bitcast i64 %t28 to double
  %t30b = bitcast i64 %t29 to double
  %t30r = fdiv double %t30a, %t30b
  %t30 = bitcast double %t30r to i64
  br label %thx32
thx32:
  br label %join32
else32:
  %t31 = load i64, i64* %L2
  br label %elx32
elx32:
  br label %join32
join32:
  %t32 = phi i64 [ %t30, %thx32 ], [ %t31, %elx32 ]
  ret i64 %t32
}
define i64 @fmax(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L0
  %t4a = bitcast i64 %t2 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp une double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t8.b = icmp ne i64 %t4, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t5 = load i64, i64* %L1
  store i64 %t5, i64* %L2
  %t6 = load i64, i64* %L2
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L1
  %t11a = bitcast i64 %t9 to double
  %t11b = bitcast i64 %t10 to double
  %t11c = fcmp une double %t11a, %t11b
  %t11 = zext i1 %t11c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = load i64, i64* %L0
  store i64 %t12, i64* %L2
  %t13 = load i64, i64* %L2
  ret i64 %t13
ar13:
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = add i64 0, %t14
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L1
  %t18a = bitcast i64 %t16 to double
  %t18b = bitcast i64 %t17 to double
  %t18c = fcmp olt double %t18a, %t18b
  %t18 = zext i1 %t18c to i64
  %t21.b = icmp ne i64 %t18, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t19 = load i64, i64* %L1
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = load i64, i64* %L0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  ret i64 %t21
}
define i64 @fmin(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = load i64, i64* %L0
  %t4a = bitcast i64 %t2 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp une double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t8.b = icmp ne i64 %t4, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t5 = load i64, i64* %L1
  store i64 %t5, i64* %L2
  %t6 = load i64, i64* %L2
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L1
  %t10 = load i64, i64* %L1
  %t11a = bitcast i64 %t9 to double
  %t11b = bitcast i64 %t10 to double
  %t11c = fcmp une double %t11a, %t11b
  %t11 = zext i1 %t11c to i64
  %t15.b = icmp ne i64 %t11, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t12 = load i64, i64* %L0
  store i64 %t12, i64* %L2
  %t13 = load i64, i64* %L2
  ret i64 %t13
ar13:
  br label %thx15
thx15:
  br label %join15
else15:
  %t14 = add i64 0, 0
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = add i64 0, %t14
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L1
  %t18a = bitcast i64 %t16 to double
  %t18b = bitcast i64 %t17 to double
  %t18c = fcmp olt double %t18a, %t18b
  %t18 = zext i1 %t18c to i64
  %t21.b = icmp ne i64 %t18, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t19 = load i64, i64* %L0
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = load i64, i64* %L1
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  ret i64 %t21
}
define i64 @fabs(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  %t4a = bitcast i64 %t1 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp olt double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t10.b = icmp ne i64 %t4, 0
  br i1 %t10.b, label %then10, label %else10
then10:
  %t5 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t6p = inttoptr i64 %t5 to i8*
  %t6d = call double @strtod(i8* %t6p, i8** null)
  %t6 = bitcast double %t6d to i64
  %t7 = load i64, i64* %L0
  %t8a = bitcast i64 %t6 to double
  %t8b = bitcast i64 %t7 to double
  %t8r = fsub double %t8a, %t8b
  %t8 = bitcast double %t8r to i64
  br label %thx10
thx10:
  br label %join10
else10:
  %t9 = load i64, i64* %L0
  br label %elx10
elx10:
  br label %join10
join10:
  %t10 = phi i64 [ %t8, %thx10 ], [ %t9, %elx10 ]
  ret i64 %t10
}
define i64 @ffloor(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [19 x i8]* @.s4e85d907520ae4b0 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  %t4a = bitcast i64 %t1 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp ogt double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t8.b = icmp ne i64 %t4, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t5 = load i64, i64* %L0
  store i64 %t5, i64* %L1
  %t6 = load i64, i64* %L1
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L0
  %t10 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t11p = inttoptr i64 %t10 to i8*
  %t11d = call double @strtod(i8* %t11p, i8** null)
  %t11 = bitcast double %t11d to i64
  %t12 = ptrtoint [19 x i8]* @.s4e85d907520ae4b0 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13d = call double @strtod(i8* %t13p, i8** null)
  %t13 = bitcast double %t13d to i64
  %t14a = bitcast i64 %t11 to double
  %t14b = bitcast i64 %t13 to double
  %t14r = fsub double %t14a, %t14b
  %t14 = bitcast double %t14r to i64
  %t15a = bitcast i64 %t9 to double
  %t15b = bitcast i64 %t14 to double
  %t15c = fcmp olt double %t15a, %t15b
  %t15 = zext i1 %t15c to i64
  %t19.b = icmp ne i64 %t15, 0
  br i1 %t19.b, label %then19, label %else19
then19:
  %t16 = load i64, i64* %L0
  store i64 %t16, i64* %L1
  %t17 = load i64, i64* %L1
  ret i64 %t17
ar17:
  br label %thx19
thx19:
  br label %join19
else19:
  %t18 = add i64 0, 0
  br label %elx19
elx19:
  br label %join19
join19:
  %t19 = add i64 0, %t18
  %t20 = load i64, i64* %L0
  %t21d = bitcast i64 %t20 to double
  %t21n = call i64 @llvm.fptosi.sat.i64.f64(double %t21d)
  %t21 = add i64 0, %t21n
  %t22f = sitofp i64 %t21 to double
  %t22 = bitcast double %t22f to i64
  store i64 %t22, i64* %L1
  %t23 = load i64, i64* %L1
  %t24 = load i64, i64* %L0
  %t25a = bitcast i64 %t23 to double
  %t25b = bitcast i64 %t24 to double
  %t25c = fcmp ogt double %t25a, %t25b
  %t25 = zext i1 %t25c to i64
  %t31.b = icmp ne i64 %t25, 0
  br i1 %t31.b, label %then31, label %else31
then31:
  %t26 = load i64, i64* %L1
  %t27 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t28p = inttoptr i64 %t27 to i8*
  %t28d = call double @strtod(i8* %t28p, i8** null)
  %t28 = bitcast double %t28d to i64
  %t29a = bitcast i64 %t26 to double
  %t29b = bitcast i64 %t28 to double
  %t29r = fsub double %t29a, %t29b
  %t29 = bitcast double %t29r to i64
  br label %thx31
thx31:
  br label %join31
else31:
  %t30 = load i64, i64* %L1
  br label %elx31
elx31:
  br label %join31
join31:
  %t31 = phi i64 [ %t29, %thx31 ], [ %t30, %elx31 ]
  ret i64 %t31
}
define i64 @fceil(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [19 x i8]* @.s4e85d907520ae4b0 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  %t4a = bitcast i64 %t1 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp ogt double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t8.b = icmp ne i64 %t4, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t5 = load i64, i64* %L0
  store i64 %t5, i64* %L1
  %t6 = load i64, i64* %L1
  ret i64 %t6
ar6:
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = add i64 0, %t7
  %t9 = load i64, i64* %L0
  %t10 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t11p = inttoptr i64 %t10 to i8*
  %t11d = call double @strtod(i8* %t11p, i8** null)
  %t11 = bitcast double %t11d to i64
  %t12 = ptrtoint [19 x i8]* @.s4e85d907520ae4b0 to i64
  %t13p = inttoptr i64 %t12 to i8*
  %t13d = call double @strtod(i8* %t13p, i8** null)
  %t13 = bitcast double %t13d to i64
  %t14a = bitcast i64 %t11 to double
  %t14b = bitcast i64 %t13 to double
  %t14r = fsub double %t14a, %t14b
  %t14 = bitcast double %t14r to i64
  %t15a = bitcast i64 %t9 to double
  %t15b = bitcast i64 %t14 to double
  %t15c = fcmp olt double %t15a, %t15b
  %t15 = zext i1 %t15c to i64
  %t19.b = icmp ne i64 %t15, 0
  br i1 %t19.b, label %then19, label %else19
then19:
  %t16 = load i64, i64* %L0
  store i64 %t16, i64* %L1
  %t17 = load i64, i64* %L1
  ret i64 %t17
ar17:
  br label %thx19
thx19:
  br label %join19
else19:
  %t18 = add i64 0, 0
  br label %elx19
elx19:
  br label %join19
join19:
  %t19 = add i64 0, %t18
  %t20 = load i64, i64* %L0
  %t21d = bitcast i64 %t20 to double
  %t21n = call i64 @llvm.fptosi.sat.i64.f64(double %t21d)
  %t21 = add i64 0, %t21n
  %t22f = sitofp i64 %t21 to double
  %t22 = bitcast double %t22f to i64
  store i64 %t22, i64* %L1
  %t23 = load i64, i64* %L1
  %t24 = load i64, i64* %L0
  %t25a = bitcast i64 %t23 to double
  %t25b = bitcast i64 %t24 to double
  %t25c = fcmp olt double %t25a, %t25b
  %t25 = zext i1 %t25c to i64
  %t31.b = icmp ne i64 %t25, 0
  br i1 %t31.b, label %then31, label %else31
then31:
  %t26 = load i64, i64* %L1
  %t27 = ptrtoint [4 x i8]* @.s00000015447fb494 to i64
  %t28p = inttoptr i64 %t27 to i8*
  %t28d = call double @strtod(i8* %t28p, i8** null)
  %t28 = bitcast double %t28d to i64
  %t29a = bitcast i64 %t26 to double
  %t29b = bitcast i64 %t28 to double
  %t29r = fadd double %t29a, %t29b
  %t29 = bitcast double %t29r to i64
  br label %thx31
thx31:
  br label %join31
else31:
  %t30 = load i64, i64* %L1
  br label %elx31
elx31:
  br label %join31
join31:
  %t31 = phi i64 [ %t29, %thx31 ], [ %t30, %elx31 ]
  ret i64 %t31
}
define i64 @fround(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [4 x i8]* @.s00000015447eb293 to i64
  %t3p = inttoptr i64 %t2 to i8*
  %t3d = call double @strtod(i8* %t3p, i8** null)
  %t3 = bitcast double %t3d to i64
  %t4a = bitcast i64 %t1 to double
  %t4b = bitcast i64 %t3 to double
  %t4c = fcmp oge double %t4a, %t4b
  %t4 = zext i1 %t4c to i64
  %t15.b = icmp ne i64 %t4, 0
  br i1 %t15.b, label %then15, label %else15
then15:
  %t5 = load i64, i64* %L0
  %t6 = ptrtoint [4 x i8]* @.s00000015447eb298 to i64
  %t7p = inttoptr i64 %t6 to i8*
  %t7d = call double @strtod(i8* %t7p, i8** null)
  %t7 = bitcast double %t7d to i64
  %t8a = bitcast i64 %t5 to double
  %t8b = bitcast i64 %t7 to double
  %t8r = fadd double %t8a, %t8b
  %t8 = bitcast double %t8r to i64
  %t9 = call i64 @ffloor(i64 %t8)
  br label %thx15
thx15:
  br label %join15
else15:
  %t10 = load i64, i64* %L0
  %t11 = ptrtoint [4 x i8]* @.s00000015447eb298 to i64
  %t12p = inttoptr i64 %t11 to i8*
  %t12d = call double @strtod(i8* %t12p, i8** null)
  %t12 = bitcast double %t12d to i64
  %t13a = bitcast i64 %t10 to double
  %t13b = bitcast i64 %t12 to double
  %t13r = fsub double %t13a, %t13b
  %t13 = bitcast double %t13r to i64
  %t14 = call i64 @fceil(i64 %t13)
  br label %elx15
elx15:
  br label %join15
join15:
  %t15 = phi i64 [ %t9, %thx15 ], [ %t14, %elx15 ]
  ret i64 %t15
}
define i64 @vflatmap(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %L8 = alloca i64
  store i64 0, i64* %L8
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L4
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  store i64 %t11, i64* %L5
  %t12 = add i64 0, 0
  store i64 %t12, i64* %L6
  %t13 = load i64, i64* %L6
  %t14 = call i64 @__vec_len(i64 %t13)
  store i64 %t14, i64* %L7
  %t15 = add i64 0, 0
  store i64 %t15, i64* %L8
  br label %lcond18
lcond18:
  %t16 = load i64, i64* %L8
  %t17 = load i64, i64* %L7
  %t18.c = icmp slt i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %lc18 = icmp ne i64 %t18, 0
  br i1 %lc18, label %lbody18, label %lend18
lbody18:
  %t19 = load i64, i64* %L2
  %t20 = load i64, i64* %L6
  %t21 = load i64, i64* %L8
  %t22 = call i64 @__vec_get(i64 %t20, i64 %t21)
  %t23 = call i64 @__vec_push(i64 %t19, i64 %t22)
  %t24 = load i64, i64* %L8
  %t25 = add i64 0, 1
  %t26 = add i64 %t24, %t25
  store i64 %t26, i64* %L8
  br label %lcond18
lend18:
  %t27 = add i64 0, 0
  %t28 = load i64, i64* %L4
  %t29 = add i64 0, 1
  %t30 = add i64 %t28, %t29
  store i64 %t30, i64* %L4
  br label %lcond8
lend8:
  %t31 = add i64 0, 0
  %t32 = load i64, i64* %L2
  ret i64 %t32
}
define i64 @vchain(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L4
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13 = call i64 @__vec_push(i64 %t9, i64 %t12)
  %t14 = load i64, i64* %L4
  %t15 = add i64 0, 1
  %t16 = add i64 %t14, %t15
  store i64 %t16, i64* %L4
  br label %lcond8
lend8:
  %t17 = add i64 0, 0
  %t18 = load i64, i64* %L1
  %t19 = call i64 @__vec_len(i64 %t18)
  store i64 %t19, i64* %L5
  %t20 = add i64 0, 0
  store i64 %t20, i64* %L6
  br label %lcond23
lcond23:
  %t21 = load i64, i64* %L6
  %t22 = load i64, i64* %L5
  %t23.c = icmp slt i64 %t21, %t22
  %t23 = zext i1 %t23.c to i64
  %lc23 = icmp ne i64 %t23, 0
  br i1 %lc23, label %lbody23, label %lend23
lbody23:
  %t24 = load i64, i64* %L2
  %t25 = load i64, i64* %L1
  %t26 = load i64, i64* %L6
  %t27 = call i64 @__vec_get(i64 %t25, i64 %t26)
  %t28 = call i64 @__vec_push(i64 %t24, i64 %t27)
  %t29 = load i64, i64* %L6
  %t30 = add i64 0, 1
  %t31 = add i64 %t29, %t30
  store i64 %t31, i64* %L6
  br label %lcond23
lend23:
  %t32 = add i64 0, 0
  %t33 = load i64, i64* %L2
  ret i64 %t33
}
define i64 @venum(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L1
  %t9 = load i64, i64* %L3
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L3
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13.mb = call i8* @malloc(i64 16)
  %t13.m = bitcast i8* %t13.mb to [2 x i64]*
  %t13.f0 = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 0
  store i64 %t9, i64* %t13.f0
  %t13.f1 = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 1
  store i64 %t12, i64* %t13.f1
  %t13.p = getelementptr [2 x i64], [2 x i64]* %t13.m, i64 0, i64 0
  %t13 = ptrtoint i64* %t13.p to i64   ; struct size=16
  %t14 = call i64 @__vec_push(i64 %t8, i64 %t13)
  %t15 = load i64, i64* %L3
  %t16 = add i64 0, 1
  %t17 = add i64 %t15, %t16
  store i64 %t17, i64* %L3
  br label %lcond7
lend7:
  %t18 = add i64 0, 0
  %t19 = load i64, i64* %L1
  ret i64 %t19
}
define i64 @vzip(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L1
  %t6 = call i64 @__vec_len(i64 %t5)
  store i64 %t6, i64* %L4
  %t7 = load i64, i64* %L3
  %t8 = load i64, i64* %L4
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %t12.b = icmp ne i64 %t9, 0
  br i1 %t12.b, label %then12, label %else12
then12:
  %t10 = load i64, i64* %L3
  br label %thx12
thx12:
  br label %join12
else12:
  %t11 = load i64, i64* %L4
  br label %elx12
elx12:
  br label %join12
join12:
  %t12 = phi i64 [ %t10, %thx12 ], [ %t11, %elx12 ]
  store i64 %t12, i64* %L5
  %t13 = add i64 0, 0
  store i64 %t13, i64* %L6
  br label %lcond16
lcond16:
  %t14 = load i64, i64* %L6
  %t15 = load i64, i64* %L5
  %t16.c = icmp slt i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %lc16 = icmp ne i64 %t16, 0
  br i1 %lc16, label %lbody16, label %lend16
lbody16:
  %t17 = load i64, i64* %L2
  %t18 = load i64, i64* %L0
  %t19 = load i64, i64* %L6
  %t20 = call i64 @__vec_get(i64 %t18, i64 %t19)
  %t21 = load i64, i64* %L1
  %t22 = load i64, i64* %L6
  %t23 = call i64 @__vec_get(i64 %t21, i64 %t22)
  %t24.mb = call i8* @malloc(i64 16)
  %t24.m = bitcast i8* %t24.mb to [2 x i64]*
  %t24.f0 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  store i64 %t20, i64* %t24.f0
  %t24.f1 = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 1
  store i64 %t23, i64* %t24.f1
  %t24.p = getelementptr [2 x i64], [2 x i64]* %t24.m, i64 0, i64 0
  %t24 = ptrtoint i64* %t24.p to i64   ; struct size=16
  %t25 = call i64 @__vec_push(i64 %t17, i64 %t24)
  %t26 = load i64, i64* %L6
  %t27 = add i64 0, 1
  %t28 = add i64 %t26, %t27
  store i64 %t28, i64* %L6
  br label %lcond16
lend16:
  %t29 = add i64 0, 0
  %t30 = load i64, i64* %L2
  ret i64 %t30
}
define i64 @range2(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  store i64 %t3, i64* %L3
  br label %lcond6
lcond6:
  %t4 = load i64, i64* %L3
  %t5 = load i64, i64* %L1
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %lc6 = icmp ne i64 %t6, 0
  br i1 %lc6, label %lbody6, label %lend6
lbody6:
  %t7 = load i64, i64* %L2
  %t8 = load i64, i64* %L3
  %t9 = call i64 @__vec_push(i64 %t7, i64 %t8)
  %t10 = load i64, i64* %L3
  %t11 = add i64 0, 1
  %t12 = add i64 %t10, %t11
  store i64 %t12, i64* %L3
  br label %lcond6
lend6:
  %t13 = add i64 0, 0
  %t14 = load i64, i64* %L2
  ret i64 %t14
}
define i64 @vmin(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3 = call i64 @__vec_get(i64 %t1, i64 %t2)
  store i64 %t3, i64* %L1
  %t4 = load i64, i64* %L0
  store i64 %t4, i64* %L2
  %t5 = add i64 0, -1
  store i64 %t5, i64* %L3
  br label %lcond12
lcond12:
  %t6 = load i64, i64* %L3
  %t7 = add i64 0, 1
  %t8 = add i64 %t6, %t7
  store i64 %t8, i64* %L3
  %t9 = load i64, i64* %L3
  %t10 = load i64, i64* %L2
  %t11 = call i64 @__vec_len(i64 %t10)
  %t12.c = icmp slt i64 %t9, %t11
  %t12 = zext i1 %t12.c to i64
  %lc12 = icmp ne i64 %t12, 0
  br i1 %lc12, label %lbody12, label %lend12
lbody12:
  %t13 = load i64, i64* %L2
  %t14 = load i64, i64* %L3
  %t15 = call i64 @__vec_get(i64 %t13, i64 %t14)
  store i64 %t15, i64* %L4
  %t16 = load i64, i64* %L4
  %t17 = load i64, i64* %L1
  %t18.c = icmp slt i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  %t21.b = icmp ne i64 %t18, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t19 = load i64, i64* %L4
  store i64 %t19, i64* %L1
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = phi i64 [ %t19, %thx21 ], [ %t20, %elx21 ]
  br label %lcond12
lend12:
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L1
  ret i64 %t23
}
define i64 @vrev(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = load i64, i64* %L2
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = add i64 0, 0
  %t7.c = icmp sgt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L3
  %t9 = add i64 0, 1
  %t10 = sub i64 %t8, %t9
  store i64 %t10, i64* %L3
  %t11 = load i64, i64* %L1
  %t12 = load i64, i64* %L0
  %t13 = load i64, i64* %L3
  %t14 = call i64 @__vec_get(i64 %t12, i64 %t13)
  %t15 = call i64 @__vec_push(i64 %t11, i64 %t14)
  br label %lcond7
lend7:
  %t16 = add i64 0, 0
  %t17 = load i64, i64* %L1
  ret i64 %t17
}
define i64 @vlast(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  %t4 = add i64 0, 1
  %t5 = sub i64 %t3, %t4
  %t6 = call i64 @__vec_get(i64 %t1, i64 %t5)
  ret i64 %t6
}
define i64 @vempty(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @__vec_len(i64 %t1)
  %t3 = add i64 0, 0
  %t4.c = icmp eq i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t7.b = icmp ne i64 %t4, 0
  br i1 %t7.b, label %then7, label %else7
then7:
  %t5 = add i64 0, 1
  br label %thx7
thx7:
  br label %join7
else7:
  %t6 = add i64 0, 0
  br label %elx7
elx7:
  br label %join7
join7:
  %t7 = phi i64 [ %t5, %thx7 ], [ %t6, %elx7 ]
  ret i64 %t7
}
define i64 @vany(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, -1
  store i64 %t3, i64* %L3
  br label %lcond10
lcond10:
  %t4 = load i64, i64* %L3
  %t5 = add i64 0, 1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L3
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__vec_len(i64 %t8)
  %t10.c = icmp slt i64 %t7, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = load i64, i64* %L2
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  store i64 %t13, i64* %L4
  %t14 = load i64, i64* %L4
  store i64 %t14, i64* %L5
  %t15 = add i64 0, 0
  %t16 = add i64 0, 1
  %t17.c = icmp eq i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t21.b = icmp ne i64 %t17, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t18 = add i64 0, 1
  store i64 %t18, i64* %L6
  %t19 = load i64, i64* %L6
  ret i64 %t19
ar19:
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = add i64 0, %t20
  br label %lcond10
lend10:
  %t22 = add i64 0, 0
  %t23 = add i64 0, 0
  ret i64 %t23
}
define i64 @vall(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, -1
  store i64 %t3, i64* %L3
  br label %lcond10
lcond10:
  %t4 = load i64, i64* %L3
  %t5 = add i64 0, 1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L3
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__vec_len(i64 %t8)
  %t10.c = icmp slt i64 %t7, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = load i64, i64* %L2
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  store i64 %t13, i64* %L4
  %t14 = load i64, i64* %L4
  store i64 %t14, i64* %L5
  %t15 = add i64 0, 0
  %t16 = add i64 0, 0
  %t17.c = icmp eq i64 %t15, %t16
  %t17 = zext i1 %t17.c to i64
  %t21.b = icmp ne i64 %t17, 0
  br i1 %t21.b, label %then21, label %else21
then21:
  %t18 = add i64 0, 0
  store i64 %t18, i64* %L6
  %t19 = load i64, i64* %L6
  ret i64 %t19
ar19:
  br label %thx21
thx21:
  br label %join21
else21:
  %t20 = add i64 0, 0
  br label %elx21
elx21:
  br label %join21
join21:
  %t21 = add i64 0, %t20
  br label %lcond10
lend10:
  %t22 = add i64 0, 0
  %t23 = add i64 0, 1
  ret i64 %t23
}
define i64 @vpos(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3 = call i64 @__vec_len(i64 %t2)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__vec_get(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = add i64 0, 0
  %t12 = add i64 0, 1
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t17.b = icmp ne i64 %t13, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  %t14 = load i64, i64* %L3
  store i64 %t14, i64* %L5
  %t15 = load i64, i64* %L5
  ret i64 %t15
ar15:
  br label %thx17
thx17:
  br label %join17
else17:
  %t16 = add i64 0, 0
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = add i64 0, %t16
  %t18 = load i64, i64* %L3
  %t19 = add i64 0, 1
  %t20 = add i64 %t18, %t19
  store i64 %t20, i64* %L3
  br label %lcond7
lend7:
  %t21 = add i64 0, 0
  %t22 = add i64 0, 0
  %t23 = add i64 0, 1
  %t24 = sub i64 %t22, %t23
  ret i64 %t24
}
define i64 @vtake(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond13
lcond13:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %t13.b = icmp ne i64 %t8, 0
  br i1 %t13.b, label %then13, label %else13
then13:
  %t9 = load i64, i64* %L4
  %t10 = load i64, i64* %L1
  %t11.c = icmp slt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  br label %thx13
thx13:
  br label %join13
else13:
  %t12 = add i64 0, 0
  br label %elx13
elx13:
  br label %join13
join13:
  %t13 = phi i64 [ %t11, %thx13 ], [ %t12, %elx13 ]
  %lc13 = icmp ne i64 %t13, 0
  br i1 %lc13, label %lbody13, label %lend13
lbody13:
  %t14 = load i64, i64* %L2
  %t15 = load i64, i64* %L0
  %t16 = load i64, i64* %L4
  %t17 = call i64 @__vec_get(i64 %t15, i64 %t16)
  %t18 = call i64 @__vec_push(i64 %t14, i64 %t17)
  %t19 = load i64, i64* %L4
  %t20 = add i64 0, 1
  %t21 = add i64 %t19, %t20
  store i64 %t21, i64* %L4
  br label %lcond13
lend13:
  %t22 = add i64 0, 0
  %t23 = load i64, i64* %L2
  ret i64 %t23
}
define i64 @vtakewhile(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L0
  %t10 = load i64, i64* %L4
  %t11 = call i64 @__vec_get(i64 %t9, i64 %t10)
  store i64 %t11, i64* %L5
  %t12 = add i64 0, 0
  %t13 = add i64 0, 1
  %t14.c = icmp eq i64 %t12, %t13
  %t14 = zext i1 %t14.c to i64
  %t24.b = icmp ne i64 %t14, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t15 = load i64, i64* %L2
  %t16 = load i64, i64* %L0
  %t17 = load i64, i64* %L4
  %t18 = call i64 @__vec_get(i64 %t16, i64 %t17)
  %t19 = call i64 @__vec_push(i64 %t15, i64 %t18)
  %t20 = load i64, i64* %L4
  %t21 = add i64 0, 1
  %t22 = add i64 %t20, %t21
  store i64 %t22, i64* %L4
  br label %thx24
thx24:
  br label %join24
else24:
  %t23 = load i64, i64* %L3
  store i64 %t23, i64* %L4
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t22, %thx24 ], [ %t23, %elx24 ]
  br label %lcond8
lend8:
  %t25 = add i64 0, 0
  %t26 = load i64, i64* %L2
  ret i64 %t26
}
define i64 @vskipwhile(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = add i64 0, 0
  store i64 %t5, i64* %L4
  %t6 = add i64 0, 1
  store i64 %t6, i64* %L5
  br label %lcond9
lcond9:
  %t7 = load i64, i64* %L4
  %t8 = load i64, i64* %L3
  %t9.c = icmp slt i64 %t7, %t8
  %t9 = zext i1 %t9.c to i64
  %lc9 = icmp ne i64 %t9, 0
  br i1 %lc9, label %lbody9, label %lend9
lbody9:
  %t10 = load i64, i64* %L5
  %t11 = add i64 0, 1
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t20.b = icmp ne i64 %t12, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t13 = load i64, i64* %L0
  %t14 = load i64, i64* %L4
  %t15 = call i64 @__vec_get(i64 %t13, i64 %t14)
  store i64 %t15, i64* %L6
  %t16 = add i64 0, 0
  %t17 = add i64 0, 1
  %t18.c = icmp eq i64 %t16, %t17
  %t18 = zext i1 %t18.c to i64
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = phi i64 [ %t18, %thx20 ], [ %t19, %elx20 ]
  %t28.b = icmp ne i64 %t20, 0
  br i1 %t28.b, label %then28, label %else28
then28:
  %t21 = add i64 0, 0
  br label %thx28
thx28:
  br label %join28
else28:
  %t22 = add i64 0, 0
  store i64 %t22, i64* %L5
  %t23 = load i64, i64* %L2
  %t24 = load i64, i64* %L0
  %t25 = load i64, i64* %L4
  %t26 = call i64 @__vec_get(i64 %t24, i64 %t25)
  %t27 = call i64 @__vec_push(i64 %t23, i64 %t26)
  br label %elx28
elx28:
  br label %join28
join28:
  %t28 = phi i64 [ %t21, %thx28 ], [ %t27, %elx28 ]
  %t29 = load i64, i64* %L4
  %t30 = add i64 0, 1
  %t31 = add i64 %t29, %t30
  store i64 %t31, i64* %L4
  br label %lcond9
lend9:
  %t32 = add i64 0, 0
  %t33 = load i64, i64* %L2
  ret i64 %t33
}
define i64 @vskip(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = call i64 @__vec_new()
  store i64 %t2, i64* %L2
  %t3 = load i64, i64* %L0
  %t4 = call i64 @__vec_len(i64 %t3)
  store i64 %t4, i64* %L3
  %t5 = load i64, i64* %L1
  store i64 %t5, i64* %L4
  br label %lcond8
lcond8:
  %t6 = load i64, i64* %L4
  %t7 = load i64, i64* %L3
  %t8.c = icmp slt i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  %lc8 = icmp ne i64 %t8, 0
  br i1 %lc8, label %lbody8, label %lend8
lbody8:
  %t9 = load i64, i64* %L2
  %t10 = load i64, i64* %L0
  %t11 = load i64, i64* %L4
  %t12 = call i64 @__vec_get(i64 %t10, i64 %t11)
  %t13 = call i64 @__vec_push(i64 %t9, i64 %t12)
  %t14 = load i64, i64* %L4
  %t15 = add i64 0, 1
  %t16 = add i64 %t14, %t15
  store i64 %t16, i64* %L4
  br label %lcond8
lend8:
  %t17 = add i64 0, 0
  %t18 = load i64, i64* %L2
  ret i64 %t18
}
define i64 @vhas(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, -1
  store i64 %t3, i64* %L3
  br label %lcond10
lcond10:
  %t4 = load i64, i64* %L3
  %t5 = add i64 0, 1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L3
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__vec_len(i64 %t8)
  %t10.c = icmp slt i64 %t7, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = load i64, i64* %L2
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  store i64 %t13, i64* %L4
  %t14 = load i64, i64* %L4
  %t15 = load i64, i64* %L1
  %t16.c = icmp eq i64 %t14, %t15
  %t16 = zext i1 %t16.c to i64
  %t20.b = icmp ne i64 %t16, 0
  br i1 %t20.b, label %then20, label %else20
then20:
  %t17 = add i64 0, 1
  store i64 %t17, i64* %L5
  %t18 = load i64, i64* %L5
  ret i64 %t18
ar18:
  br label %thx20
thx20:
  br label %join20
else20:
  %t19 = add i64 0, 0
  br label %elx20
elx20:
  br label %join20
join20:
  %t20 = add i64 0, %t19
  br label %lcond10
lend10:
  %t21 = add i64 0, 0
  %t22 = add i64 0, 0
  ret i64 %t22
}
define i64 @f64_trim(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = ptrtoint [2 x i8]* @.s0000000000151a33 to i64
  %t3 = call i64 @str_find(i64 %t1, i64 %t2)
  store i64 %t3, i64* %L1
  %t4 = load i64, i64* %L1
  %t5 = add i64 0, 0
  %t6.c = icmp slt i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  %t10.b = icmp ne i64 %t6, 0
  br i1 %t10.b, label %then10, label %else10
then10:
  %t7 = load i64, i64* %L0
  store i64 %t7, i64* %L2
  %t8 = load i64, i64* %L2
  ret i64 %t8
ar8:
  br label %thx10
thx10:
  br label %join10
else10:
  %t9 = add i64 0, 0
  br label %elx10
elx10:
  br label %join10
join10:
  %t10 = add i64 0, %t9
  %t11 = load i64, i64* %L0
  %t12p = inttoptr i64 %t11 to i8*
  %t12 = call i64 @strlen(i8* %t12p)
  store i64 %t12, i64* %L2
  br label %lcond17
lcond17:
  %t13 = load i64, i64* %L2
  %t14 = load i64, i64* %L1
  %t15 = add i64 0, 1
  %t16 = add i64 %t14, %t15
  %t17.c = icmp sgt i64 %t13, %t16
  %t17 = zext i1 %t17.c to i64
  %lc17 = icmp ne i64 %t17, 0
  br i1 %lc17, label %lbody17, label %lend17
lbody17:
  %t18 = load i64, i64* %L0
  %t19 = load i64, i64* %L2
  %t20 = add i64 0, 1
  %t21 = sub i64 %t19, %t20
  %t22 = add i64 0, 1
  %t23 = call i64 @__str_sub(i64 %t18, i64 %t21, i64 %t22)
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t25 = call i64 @__streq(i64 %t23, i64 %t24)
  %t31.b = icmp ne i64 %t25, 0
  br i1 %t31.b, label %then31, label %else31
then31:
  %t26 = load i64, i64* %L2
  %t27 = add i64 0, 1
  %t28 = sub i64 %t26, %t27
  store i64 %t28, i64* %L2
  br label %thx31
thx31:
  br label %join31
else31:
  br label %lend17
bk29_0:
  %t30 = add i64 0, 0
  br label %elx31
elx31:
  br label %join31
join31:
  %t31 = phi i64 [ %t28, %thx31 ], [ %t30, %elx31 ]
  br label %lcond17
lend17:
  %t32 = add i64 0, 0
  %t33 = load i64, i64* %L2
  %t34 = load i64, i64* %L1
  %t35 = add i64 0, 1
  %t36 = add i64 %t34, %t35
  %t37.c = icmp eq i64 %t33, %t36
  %t37 = zext i1 %t37.c to i64
  %t40.b = icmp ne i64 %t37, 0
  br i1 %t40.b, label %then40, label %else40
then40:
  %t38 = load i64, i64* %L1
  store i64 %t38, i64* %L2
  br label %thx40
thx40:
  br label %join40
else40:
  %t39 = add i64 0, 0
  br label %elx40
elx40:
  br label %join40
join40:
  %t40 = phi i64 [ %t38, %thx40 ], [ %t39, %elx40 ]
  %t41 = load i64, i64* %L0
  %t42 = add i64 0, 0
  %t43 = load i64, i64* %L2
  %t44 = call i64 @__str_sub(i64 %t41, i64 %t42, i64 %t43)
  ret i64 %t44
}
define i64 @f64_exp10(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L1
  %t3 = add i64 0, 0
  %t4 = add i64 0, 1
  %t5 = call i64 @__str_sub(i64 %t2, i64 %t3, i64 %t4)
  %t6 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t7 = call i64 @__streq(i64 %t5, i64 %t6)
  %t16.b = icmp ne i64 %t7, 0
  br i1 %t16.b, label %then16, label %else16
then16:
  %t8 = load i64, i64* %L1
  %t9 = add i64 0, 1
  %t10 = load i64, i64* %L1
  %t11p = inttoptr i64 %t10 to i8*
  %t11 = call i64 @strlen(i8* %t11p)
  %t12 = add i64 0, 1
  %t13 = sub i64 %t11, %t12
  %t14 = call i64 @__str_sub(i64 %t8, i64 %t9, i64 %t13)
  store i64 %t14, i64* %L1
  br label %thx16
thx16:
  br label %join16
else16:
  %t15 = add i64 0, 0
  br label %elx16
elx16:
  br label %join16
join16:
  %t16 = phi i64 [ %t14, %thx16 ], [ %t15, %elx16 ]
  %t17 = load i64, i64* %L1
  %t18 = ptrtoint [2 x i8]* @.s0000000000151a33 to i64
  %t19 = call i64 @str_find(i64 %t17, i64 %t18)
  store i64 %t19, i64* %L2
  %t20 = load i64, i64* %L1
  %t21 = add i64 0, 0
  %t22 = add i64 0, 1
  %t23 = call i64 @__str_sub(i64 %t20, i64 %t21, i64 %t22)
  %t24 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t25 = call i64 @__streq(i64 %t23, i64 %t24)
  %t64.b = icmp ne i64 %t25, 0
  br i1 %t64.b, label %then64, label %else64
then64:
  %t26 = load i64, i64* %L2
  %t27 = add i64 0, 0
  %t28.c = icmp slt i64 %t26, %t27
  %t28 = zext i1 %t28.c to i64
  %t32.b = icmp ne i64 %t28, 0
  br i1 %t32.b, label %then32, label %else32
then32:
  %t29 = add i64 0, 0
  store i64 %t29, i64* %L3
  %t30 = load i64, i64* %L3
  ret i64 %t30
ar30:
  br label %thx32
thx32:
  br label %join32
else32:
  %t31 = add i64 0, 0
  br label %elx32
elx32:
  br label %join32
join32:
  %t32 = add i64 0, %t31
  %t33 = load i64, i64* %L2
  %t34 = add i64 0, 1
  %t35 = add i64 %t33, %t34
  store i64 %t35, i64* %L3
  %t36 = add i64 0, 0
  store i64 %t36, i64* %L4
  br label %lcond40
lcond40:
  %t37 = load i64, i64* %L3
  %t38 = load i64, i64* %L1
  %t39p = inttoptr i64 %t38 to i8*
  %t39 = call i64 @strlen(i8* %t39p)
  %t40.c = icmp slt i64 %t37, %t39
  %t40 = zext i1 %t40.c to i64
  %lc40 = icmp ne i64 %t40, 0
  br i1 %lc40, label %lbody40, label %lend40
lbody40:
  %t41 = load i64, i64* %L1
  %t42 = load i64, i64* %L3
  %t43 = add i64 0, 1
  %t44 = call i64 @__str_sub(i64 %t41, i64 %t42, i64 %t43)
  %t45 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t46 = call i64 @__streq(i64 %t44, i64 %t45)
  %t55.b = icmp ne i64 %t46, 0
  br i1 %t55.b, label %then55, label %else55
then55:
  %t47 = load i64, i64* %L4
  %t48 = add i64 0, 1
  %t49 = add i64 %t47, %t48
  store i64 %t49, i64* %L4
  %t50 = load i64, i64* %L3
  %t51 = add i64 0, 1
  %t52 = add i64 %t50, %t51
  store i64 %t52, i64* %L3
  br label %thx55
thx55:
  br label %join55
else55:
  br label %lend40
bk53_0:
  %t54 = add i64 0, 0
  br label %elx55
elx55:
  br label %join55
join55:
  %t55 = phi i64 [ %t52, %thx55 ], [ %t54, %elx55 ]
  br label %lcond40
lend40:
  %t56 = add i64 0, 0
  %t57 = add i64 0, 0
  %t58 = load i64, i64* %L4
  %t59 = sub i64 %t57, %t58
  %t60 = add i64 0, 1
  %t61 = sub i64 %t59, %t60
  store i64 %t61, i64* %L5
  %t62 = load i64, i64* %L5
  ret i64 %t62
ar62:
  br label %thx64
thx64:
  br label %join64
else64:
  %t63 = add i64 0, 0
  br label %elx64
elx64:
  br label %join64
join64:
  %t64 = add i64 0, %t63
  %t65 = load i64, i64* %L2
  %t66 = add i64 0, 0
  %t67.c = icmp slt i64 %t65, %t66
  %t67 = zext i1 %t67.c to i64
  %t74.b = icmp ne i64 %t67, 0
  br i1 %t74.b, label %then74, label %else74
then74:
  %t68 = load i64, i64* %L1
  %t69p = inttoptr i64 %t68 to i8*
  %t69 = call i64 @strlen(i8* %t69p)
  %t70 = add i64 0, 1
  %t71 = sub i64 %t69, %t70
  store i64 %t71, i64* %L3
  %t72 = load i64, i64* %L3
  ret i64 %t72
ar72:
  br label %thx74
thx74:
  br label %join74
else74:
  %t73 = add i64 0, 0
  br label %elx74
elx74:
  br label %join74
join74:
  %t74 = add i64 0, %t73
  %t75 = load i64, i64* %L2
  %t76 = add i64 0, 1
  %t77 = sub i64 %t75, %t76
  ret i64 %t77
}
define i64 @f64_sci(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, 0
  store i64 %t1, i64* %L1
  %t2 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t2, i64* %L2
  br label %lcond5
lcond5:
  %t3 = load i64, i64* %L1
  %t4 = add i64 0, 17
  %t5.c = icmp sle i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %lc5 = icmp ne i64 %t5, 0
  br i1 %lc5, label %lbody5, label %lend5
lbody5:
  %t6 = load i64, i64* %L1
  %t7 = load i64, i64* %L0
  %t8b = call i8* @malloc(i64 1400)
  %t8d = bitcast i64 %t7 to double
  %t8w = trunc i64 %t6 to i32
  %t8c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %t8b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfe, i64 0, i64 0), i32 %t8w, double %t8d)
  %t8 = ptrtoint i8* %t8b to i64
  store i64 %t8, i64* %L2
  %t9 = load i64, i64* %L2
  %t10p = inttoptr i64 %t9 to i8*
  %t10d = call double @strtod(i8* %t10p, i8** null)
  %t10 = bitcast double %t10d to i64
  %t11 = load i64, i64* %L0
  %t12.c = icmp eq i64 %t10, %t11
  %t12 = zext i1 %t12.c to i64
  %t16.b = icmp ne i64 %t12, 0
  br i1 %t16.b, label %then16, label %else16
then16:
  br label %lend5
bk13_0:
  %t14 = add i64 0, 0
  br label %thx16
thx16:
  br label %join16
else16:
  %t15 = add i64 0, 0
  br label %elx16
elx16:
  br label %join16
join16:
  %t16 = phi i64 [ %t14, %thx16 ], [ %t15, %elx16 ]
  %t17 = load i64, i64* %L1
  %t18 = add i64 0, 1
  %t19 = add i64 %t17, %t18
  store i64 %t19, i64* %L1
  br label %lcond5
lend5:
  %t20 = add i64 0, 0
  %t21 = load i64, i64* %L2
  %t22 = ptrtoint [2 x i8]* @.s0000000000151a6a to i64
  %t23 = call i64 @str_find(i64 %t21, i64 %t22)
  store i64 %t23, i64* %L3
  %t24 = load i64, i64* %L2
  %t25 = add i64 0, 0
  %t26 = load i64, i64* %L3
  %t27 = call i64 @__str_sub(i64 %t24, i64 %t25, i64 %t26)
  %t28 = call i64 @f64_trim(i64 %t27)
  store i64 %t28, i64* %L4
  %t29 = load i64, i64* %L2
  %t30 = load i64, i64* %L3
  %t31 = add i64 0, 1
  %t32 = add i64 %t30, %t31
  %t33 = load i64, i64* %L2
  %t34p = inttoptr i64 %t33 to i8*
  %t34 = call i64 @strlen(i8* %t34p)
  %t35 = load i64, i64* %L3
  %t36 = sub i64 %t34, %t35
  %t37 = add i64 0, 1
  %t38 = sub i64 %t36, %t37
  %t39 = call i64 @__str_sub(i64 %t29, i64 %t32, i64 %t38)
  store i64 %t39, i64* %L5
  %t40 = add i64 0, 0
  store i64 %t40, i64* %L6
  %t41 = load i64, i64* %L5
  %t42 = add i64 0, 0
  %t43 = add i64 0, 1
  %t44 = call i64 @__str_sub(i64 %t41, i64 %t42, i64 %t43)
  %t45 = ptrtoint [2 x i8]* @.s0000000000151a30 to i64
  %t46 = call i64 @__streq(i64 %t44, i64 %t45)
  %t70.b = icmp ne i64 %t46, 0
  br i1 %t70.b, label %then70, label %else70
then70:
  %t47 = load i64, i64* %L5
  %t48 = add i64 0, 1
  %t49 = load i64, i64* %L5
  %t50p = inttoptr i64 %t49 to i8*
  %t50 = call i64 @strlen(i8* %t50p)
  %t51 = add i64 0, 1
  %t52 = sub i64 %t50, %t51
  %t53 = call i64 @__str_sub(i64 %t47, i64 %t48, i64 %t52)
  store i64 %t53, i64* %L5
  br label %thx70
thx70:
  br label %join70
else70:
  %t54 = load i64, i64* %L5
  %t55 = add i64 0, 0
  %t56 = add i64 0, 1
  %t57 = call i64 @__str_sub(i64 %t54, i64 %t55, i64 %t56)
  %t58 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t59 = call i64 @__streq(i64 %t57, i64 %t58)
  %t69.b = icmp ne i64 %t59, 0
  br i1 %t69.b, label %then69, label %else69
then69:
  %t60 = add i64 0, 1
  store i64 %t60, i64* %L6
  %t61 = load i64, i64* %L5
  %t62 = add i64 0, 1
  %t63 = load i64, i64* %L5
  %t64p = inttoptr i64 %t63 to i8*
  %t64 = call i64 @strlen(i8* %t64p)
  %t65 = add i64 0, 1
  %t66 = sub i64 %t64, %t65
  %t67 = call i64 @__str_sub(i64 %t61, i64 %t62, i64 %t66)
  store i64 %t67, i64* %L5
  br label %thx69
thx69:
  br label %join69
else69:
  %t68 = add i64 0, 0
  br label %elx69
elx69:
  br label %join69
join69:
  %t69 = phi i64 [ %t67, %thx69 ], [ %t68, %elx69 ]
  br label %elx70
elx70:
  br label %join70
join70:
  %t70 = phi i64 [ %t53, %thx70 ], [ %t69, %elx70 ]
  br label %lcond74
lcond74:
  %t71 = load i64, i64* %L5
  %t72p = inttoptr i64 %t71 to i8*
  %t72 = call i64 @strlen(i8* %t72p)
  %t73 = add i64 0, 1
  %t74.c = icmp sgt i64 %t72, %t73
  %t74 = zext i1 %t74.c to i64
  %lc74 = icmp ne i64 %t74, 0
  br i1 %lc74, label %lbody74, label %lend74
lbody74:
  %t75 = load i64, i64* %L5
  %t76 = add i64 0, 0
  %t77 = add i64 0, 1
  %t78 = call i64 @__str_sub(i64 %t75, i64 %t76, i64 %t77)
  %t79 = ptrtoint [2 x i8]* @.s0000000000151a35 to i64
  %t80 = call i64 @__streq(i64 %t78, i64 %t79)
  %t90.b = icmp ne i64 %t80, 0
  br i1 %t90.b, label %then90, label %else90
then90:
  %t81 = load i64, i64* %L5
  %t82 = add i64 0, 1
  %t83 = load i64, i64* %L5
  %t84p = inttoptr i64 %t83 to i8*
  %t84 = call i64 @strlen(i8* %t84p)
  %t85 = add i64 0, 1
  %t86 = sub i64 %t84, %t85
  %t87 = call i64 @__str_sub(i64 %t81, i64 %t82, i64 %t86)
  store i64 %t87, i64* %L5
  br label %thx90
thx90:
  br label %join90
else90:
  br label %lend74
bk88_0:
  %t89 = add i64 0, 0
  br label %elx90
elx90:
  br label %join90
join90:
  %t90 = phi i64 [ %t87, %thx90 ], [ %t89, %elx90 ]
  br label %lcond74
lend74:
  %t91 = add i64 0, 0
  %t92 = load i64, i64* %L4
  %t93 = ptrtoint [2 x i8]* @.s0000000000151a6a to i64
  %t94 = call i64 @__str_cat(i64 %t92, i64 %t93)
  store i64 %t94, i64* %L7
  %t95 = load i64, i64* %L6
  %t96 = add i64 0, 1
  %t97.c = icmp eq i64 %t95, %t96
  %t97 = zext i1 %t97.c to i64
  %t102.b = icmp ne i64 %t97, 0
  br i1 %t102.b, label %then102, label %else102
then102:
  %t98 = load i64, i64* %L7
  %t99 = ptrtoint [2 x i8]* @.s0000000000151a32 to i64
  %t100 = call i64 @__str_cat(i64 %t98, i64 %t99)
  store i64 %t100, i64* %L7
  br label %thx102
thx102:
  br label %join102
else102:
  %t101 = add i64 0, 0
  br label %elx102
elx102:
  br label %join102
join102:
  %t102 = phi i64 [ %t100, %thx102 ], [ %t101, %elx102 ]
  %t103 = load i64, i64* %L7
  %t104 = load i64, i64* %L5
  %t105 = call i64 @__str_cat(i64 %t103, i64 %t104)
  ret i64 %t105
}
define i64 @f64_str(i64 %p0, i64 %p1) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %L6 = alloca i64
  store i64 0, i64* %L6
  %L7 = alloca i64
  store i64 0, i64* %L7
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = add i64 0, %p1
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3d = bitcast i64 %t2 to double
  %t3c = fcmp uno double %t3d, %t3d
  %t3 = zext i1 %t3c to i64
  %t4 = add i64 0, 1
  %t5.c = icmp eq i64 %t3, %t4
  %t5 = zext i1 %t5.c to i64
  %t9.b = icmp ne i64 %t5, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t6 = ptrtoint [4 x i8]* @.s00000015449d2202 to i64
  store i64 %t6, i64* %L2
  %t7 = load i64, i64* %L2
  ret i64 %t7
ar7:
  br label %thx9
thx9:
  br label %join9
else9:
  %t8 = add i64 0, 0
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = add i64 0, %t8
  %t10 = load i64, i64* %L0
  store i64 %t10, i64* %L2
  %t11 = load i64, i64* %L2
  %t12 = add i64 0, 9218868437227405312
  %t13.c = icmp eq i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t17.b = icmp ne i64 %t13, 0
  br i1 %t17.b, label %then17, label %else17
then17:
  %t14 = ptrtoint [4 x i8]* @.s0000001544b86542 to i64
  store i64 %t14, i64* %L3
  %t15 = load i64, i64* %L3
  ret i64 %t15
ar15:
  br label %thx17
thx17:
  br label %join17
else17:
  %t16 = add i64 0, 0
  br label %elx17
elx17:
  br label %join17
join17:
  %t17 = add i64 0, %t16
  %t18 = load i64, i64* %L2
  %t19 = add i64 0, 0
  %t20 = add i64 0, 4503599627370496
  %t21 = sub i64 %t19, %t20
  %t22.c = icmp eq i64 %t18, %t21
  %t22 = zext i1 %t22.c to i64
  %t26.b = icmp ne i64 %t22, 0
  br i1 %t26.b, label %then26, label %else26
then26:
  %t23 = ptrtoint [5 x i8]* @.s00001559c063f16f to i64
  store i64 %t23, i64* %L3
  %t24 = load i64, i64* %L3
  ret i64 %t24
ar24:
  br label %thx26
thx26:
  br label %join26
else26:
  %t25 = add i64 0, 0
  br label %elx26
elx26:
  br label %join26
join26:
  %t26 = add i64 0, %t25
  %t27 = load i64, i64* %L1
  %t28 = add i64 0, 0
  %t29.c = icmp eq i64 %t27, %t28
  %t29 = zext i1 %t29.c to i64
  %t43.b = icmp ne i64 %t29, 0
  br i1 %t43.b, label %then43, label %else43
then43:
  %t30 = load i64, i64* %L0
  %t31 = ptrtoint [5 x i8]* @.s00001559c46bb802 to i64
  %t32p = inttoptr i64 %t31 to i8*
  %t32d = call double @strtod(i8* %t32p, i8** null)
  %t32 = bitcast double %t32d to i64
  %t33a = bitcast i64 %t30 to double
  %t33b = bitcast i64 %t32 to double
  %t33c = fcmp oge double %t33a, %t33b
  %t33 = zext i1 %t33c to i64
  %t41.b = icmp ne i64 %t33, 0
  br i1 %t41.b, label %then41, label %else41
then41:
  %t34 = add i64 0, 1
  br label %thx41
thx41:
  br label %join41
else41:
  %t35 = load i64, i64* %L0
  %t36 = ptrtoint [5 x i8]* @.s00001559c46bb802 to i64
  %t37p = inttoptr i64 %t36 to i8*
  %t37d = call double @strtod(i8* %t37p, i8** null)
  %t37 = bitcast double %t37d to i64
  %t38 = add i64 0, -9223372036854775808
  %t39 = xor i64 %t37, %t38
  %t40a = bitcast i64 %t35 to double
  %t40b = bitcast i64 %t39 to double
  %t40c = fcmp ole double %t40a, %t40b
  %t40 = zext i1 %t40c to i64
  br label %elx41
elx41:
  br label %join41
join41:
  %t41 = phi i64 [ %t34, %thx41 ], [ %t40, %elx41 ]
  br label %thx43
thx43:
  br label %join43
else43:
  %t42 = add i64 0, 0
  br label %elx43
elx43:
  br label %join43
join43:
  %t43 = phi i64 [ %t41, %thx43 ], [ %t42, %elx43 ]
  %t48.b = icmp ne i64 %t43, 0
  br i1 %t48.b, label %then48, label %else48
then48:
  %t44 = load i64, i64* %L0
  %t45 = call i64 @f64_big(i64 %t44)
  store i64 %t45, i64* %L3
  %t46 = load i64, i64* %L3
  ret i64 %t46
ar46:
  br label %thx48
thx48:
  br label %join48
else48:
  %t47 = add i64 0, 0
  br label %elx48
elx48:
  br label %join48
join48:
  %t48 = add i64 0, %t47
  %t49 = add i64 0, 0
  store i64 %t49, i64* %L3
  %t50 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t50, i64* %L4
  br label %lcond53
lcond53:
  %t51 = load i64, i64* %L3
  %t52 = add i64 0, 340
  %t53.c = icmp sle i64 %t51, %t52
  %t53 = zext i1 %t53.c to i64
  %lc53 = icmp ne i64 %t53, 0
  br i1 %lc53, label %lbody53, label %lend53
lbody53:
  %t54 = load i64, i64* %L3
  %t55 = load i64, i64* %L0
  %t56b = call i8* @malloc(i64 1400)
  %t56d = bitcast i64 %t55 to double
  %t56w = trunc i64 %t54 to i32
  %t56c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* %t56b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfp, i64 0, i64 0), i32 %t56w, double %t56d)
  %t56 = ptrtoint i8* %t56b to i64
  store i64 %t56, i64* %L4
  %t57 = load i64, i64* %L4
  %t58p = inttoptr i64 %t57 to i8*
  %t58d = call double @strtod(i8* %t58p, i8** null)
  %t58 = bitcast double %t58d to i64
  %t59 = load i64, i64* %L2
  %t60.c = icmp eq i64 %t58, %t59
  %t60 = zext i1 %t60.c to i64
  %t64.b = icmp ne i64 %t60, 0
  br i1 %t64.b, label %then64, label %else64
then64:
  br label %lend53
bk61_0:
  %t62 = add i64 0, 0
  br label %thx64
thx64:
  br label %join64
else64:
  %t63 = add i64 0, 0
  br label %elx64
elx64:
  br label %join64
join64:
  %t64 = phi i64 [ %t62, %thx64 ], [ %t63, %elx64 ]
  %t65 = load i64, i64* %L3
  %t66 = add i64 0, 1
  %t67 = add i64 %t65, %t66
  store i64 %t67, i64* %L3
  br label %lcond53
lend53:
  %t68 = add i64 0, 0
  %t69 = load i64, i64* %L0
  %t70 = load i64, i64* %L3
  %t71 = load i64, i64* %L4
  %t72 = call i64 @f64_tieup(i64 %t69, i64 %t70, i64 %t71)
  store i64 %t72, i64* %L4
  %t73 = load i64, i64* %L4
  %t74 = call i64 @f64_trim(i64 %t73)
  store i64 %t74, i64* %L5
  %t75 = load i64, i64* %L1
  %t76 = add i64 0, 0
  %t77.c = icmp eq i64 %t75, %t76
  %t77 = zext i1 %t77.c to i64
  %t81.b = icmp ne i64 %t77, 0
  br i1 %t81.b, label %then81, label %else81
then81:
  %t78 = load i64, i64* %L5
  store i64 %t78, i64* %L6
  %t79 = load i64, i64* %L6
  ret i64 %t79
ar79:
  br label %thx81
thx81:
  br label %join81
else81:
  %t80 = add i64 0, 0
  br label %elx81
elx81:
  br label %join81
join81:
  %t81 = add i64 0, %t80
  %t82 = load i64, i64* %L4
  %t83 = call i64 @f64_exp10(i64 %t82)
  store i64 %t83, i64* %L6
  %t84 = load i64, i64* %L6
  %t85 = add i64 0, 16
  %t86.c = icmp slt i64 %t84, %t85
  %t86 = zext i1 %t86.c to i64
  %t93.b = icmp ne i64 %t86, 0
  br i1 %t93.b, label %then93, label %else93
then93:
  %t87 = add i64 0, 0
  %t88 = add i64 0, 5
  %t89 = sub i64 %t87, %t88
  %t90 = load i64, i64* %L6
  %t91.c = icmp slt i64 %t89, %t90
  %t91 = zext i1 %t91.c to i64
  br label %thx93
thx93:
  br label %join93
else93:
  %t92 = add i64 0, 0
  br label %elx93
elx93:
  br label %join93
join93:
  %t93 = phi i64 [ %t91, %thx93 ], [ %t92, %elx93 ]
  %t108.b = icmp ne i64 %t93, 0
  br i1 %t108.b, label %then108, label %else108
then108:
  %t94 = load i64, i64* %L5
  %t95 = ptrtoint [2 x i8]* @.s0000000000151a33 to i64
  %t96 = call i64 @str_find(i64 %t94, i64 %t95)
  %t97 = add i64 0, 0
  %t98.c = icmp slt i64 %t96, %t97
  %t98 = zext i1 %t98.c to i64
  %t104.b = icmp ne i64 %t98, 0
  br i1 %t104.b, label %then104, label %else104
then104:
  %t99 = load i64, i64* %L5
  %t100 = ptrtoint [3 x i8]* @.s00000000152f4d63 to i64
  %t101 = call i64 @__str_cat(i64 %t99, i64 %t100)
  store i64 %t101, i64* %L7
  %t102 = load i64, i64* %L7
  ret i64 %t102
ar102:
  br label %thx104
thx104:
  br label %join104
else104:
  %t103 = add i64 0, 0
  br label %elx104
elx104:
  br label %join104
join104:
  %t104 = add i64 0, %t103
  %t105 = load i64, i64* %L5
  store i64 %t105, i64* %L7
  %t106 = load i64, i64* %L7
  ret i64 %t106
ar106:
  br label %thx108
thx108:
  br label %join108
else108:
  %t107 = add i64 0, 0
  br label %elx108
elx108:
  br label %join108
join108:
  %t108 = add i64 0, %t107
  %t109 = load i64, i64* %L0
  %t110 = call i64 @f64_sci(i64 %t109)
  ret i64 %t110
}
define i64 @str_chars_vec(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = call i64 @__vec_new()
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  %t3p = inttoptr i64 %t2 to i8*
  %t3 = call i64 @strlen(i8* %t3p)
  store i64 %t3, i64* %L2
  %t4 = add i64 0, 0
  store i64 %t4, i64* %L3
  br label %lcond7
lcond7:
  %t5 = load i64, i64* %L3
  %t6 = load i64, i64* %L2
  %t7.c = icmp slt i64 %t5, %t6
  %t7 = zext i1 %t7.c to i64
  %lc7 = icmp ne i64 %t7, 0
  br i1 %lc7, label %lbody7, label %lend7
lbody7:
  %t8 = load i64, i64* %L0
  %t9 = load i64, i64* %L3
  %t10 = call i64 @__str_at(i64 %t8, i64 %t9)
  store i64 %t10, i64* %L4
  %t11 = load i64, i64* %L4
  %t12 = add i64 0, 128
  %t13.c = icmp slt i64 %t11, %t12
  %t13 = zext i1 %t13.c to i64
  %t112.b = icmp ne i64 %t13, 0
  br i1 %t112.b, label %then112, label %else112
then112:
  %t14 = load i64, i64* %L1
  %t15 = load i64, i64* %L4
  %t16 = call i64 @__vec_push(i64 %t14, i64 %t15)
  %t17 = load i64, i64* %L3
  %t18 = add i64 0, 1
  %t19 = add i64 %t17, %t18
  store i64 %t19, i64* %L3
  br label %thx112
thx112:
  br label %join112
else112:
  %t20 = load i64, i64* %L4
  %t21 = add i64 0, 224
  %t22.c = icmp slt i64 %t20, %t21
  %t22 = zext i1 %t22.c to i64
  %t111.b = icmp ne i64 %t22, 0
  br i1 %t111.b, label %then111, label %else111
then111:
  %t23 = load i64, i64* %L1
  %t24 = load i64, i64* %L4
  %t25 = add i64 0, 192
  %t26 = sub i64 %t24, %t25
  %t27 = add i64 0, 64
  %t28 = mul i64 %t26, %t27
  %t29 = load i64, i64* %L0
  %t30 = load i64, i64* %L3
  %t31 = add i64 0, 1
  %t32 = add i64 %t30, %t31
  %t33 = call i64 @__str_at(i64 %t29, i64 %t32)
  %t34 = add i64 0, 128
  %t35 = sub i64 %t33, %t34
  %t36 = add i64 %t28, %t35
  %t37 = call i64 @__vec_push(i64 %t23, i64 %t36)
  %t38 = load i64, i64* %L3
  %t39 = add i64 0, 2
  %t40 = add i64 %t38, %t39
  store i64 %t40, i64* %L3
  br label %thx111
thx111:
  br label %join111
else111:
  %t41 = load i64, i64* %L4
  %t42 = add i64 0, 240
  %t43.c = icmp slt i64 %t41, %t42
  %t43 = zext i1 %t43.c to i64
  %t110.b = icmp ne i64 %t43, 0
  br i1 %t110.b, label %then110, label %else110
then110:
  %t44 = load i64, i64* %L1
  %t45 = load i64, i64* %L4
  %t46 = add i64 0, 224
  %t47 = sub i64 %t45, %t46
  %t48 = add i64 0, 64
  %t49 = mul i64 %t47, %t48
  %t50 = load i64, i64* %L0
  %t51 = load i64, i64* %L3
  %t52 = add i64 0, 1
  %t53 = add i64 %t51, %t52
  %t54 = call i64 @__str_at(i64 %t50, i64 %t53)
  %t55 = add i64 0, 128
  %t56 = sub i64 %t54, %t55
  %t57 = add i64 %t49, %t56
  %t58 = add i64 0, 64
  %t59 = mul i64 %t57, %t58
  %t60 = load i64, i64* %L0
  %t61 = load i64, i64* %L3
  %t62 = add i64 0, 2
  %t63 = add i64 %t61, %t62
  %t64 = call i64 @__str_at(i64 %t60, i64 %t63)
  %t65 = add i64 0, 128
  %t66 = sub i64 %t64, %t65
  %t67 = add i64 %t59, %t66
  %t68 = call i64 @__vec_push(i64 %t44, i64 %t67)
  %t69 = load i64, i64* %L3
  %t70 = add i64 0, 3
  %t71 = add i64 %t69, %t70
  store i64 %t71, i64* %L3
  br label %thx110
thx110:
  br label %join110
else110:
  %t72 = load i64, i64* %L1
  %t73 = load i64, i64* %L4
  %t74 = add i64 0, 240
  %t75 = sub i64 %t73, %t74
  %t76 = add i64 0, 64
  %t77 = mul i64 %t75, %t76
  %t78 = load i64, i64* %L0
  %t79 = load i64, i64* %L3
  %t80 = add i64 0, 1
  %t81 = add i64 %t79, %t80
  %t82 = call i64 @__str_at(i64 %t78, i64 %t81)
  %t83 = add i64 0, 128
  %t84 = sub i64 %t82, %t83
  %t85 = add i64 %t77, %t84
  %t86 = add i64 0, 64
  %t87 = mul i64 %t85, %t86
  %t88 = load i64, i64* %L0
  %t89 = load i64, i64* %L3
  %t90 = add i64 0, 2
  %t91 = add i64 %t89, %t90
  %t92 = call i64 @__str_at(i64 %t88, i64 %t91)
  %t93 = add i64 0, 128
  %t94 = sub i64 %t92, %t93
  %t95 = add i64 %t87, %t94
  %t96 = add i64 0, 64
  %t97 = mul i64 %t95, %t96
  %t98 = load i64, i64* %L0
  %t99 = load i64, i64* %L3
  %t100 = add i64 0, 3
  %t101 = add i64 %t99, %t100
  %t102 = call i64 @__str_at(i64 %t98, i64 %t101)
  %t103 = add i64 0, 128
  %t104 = sub i64 %t102, %t103
  %t105 = add i64 %t97, %t104
  %t106 = call i64 @__vec_push(i64 %t72, i64 %t105)
  %t107 = load i64, i64* %L3
  %t108 = add i64 0, 4
  %t109 = add i64 %t107, %t108
  store i64 %t109, i64* %L3
  br label %elx110
elx110:
  br label %join110
join110:
  %t110 = phi i64 [ %t71, %thx110 ], [ %t109, %elx110 ]
  br label %elx111
elx111:
  br label %join111
join111:
  %t111 = phi i64 [ %t40, %thx111 ], [ %t110, %elx111 ]
  br label %elx112
elx112:
  br label %join112
join112:
  %t112 = phi i64 [ %t19, %thx112 ], [ %t111, %elx112 ]
  br label %lcond7
lend7:
  %t113 = add i64 0, 0
  %t114 = load i64, i64* %L1
  ret i64 %t114
}
define i64 @vchar_string(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %L2 = alloca i64
  store i64 0, i64* %L2
  %L3 = alloca i64
  store i64 0, i64* %L3
  %L4 = alloca i64
  store i64 0, i64* %L4
  %L5 = alloca i64
  store i64 0, i64* %L5
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = ptrtoint [1 x i8]* @.s0000000000001505 to i64
  store i64 %t1, i64* %L1
  %t2 = load i64, i64* %L0
  store i64 %t2, i64* %L2
  %t3 = add i64 0, -1
  store i64 %t3, i64* %L3
  br label %lcond10
lcond10:
  %t4 = load i64, i64* %L3
  %t5 = add i64 0, 1
  %t6 = add i64 %t4, %t5
  store i64 %t6, i64* %L3
  %t7 = load i64, i64* %L3
  %t8 = load i64, i64* %L2
  %t9 = call i64 @__vec_len(i64 %t8)
  %t10.c = icmp slt i64 %t7, %t9
  %t10 = zext i1 %t10.c to i64
  %lc10 = icmp ne i64 %t10, 0
  br i1 %lc10, label %lbody10, label %lend10
lbody10:
  %t11 = load i64, i64* %L2
  %t12 = load i64, i64* %L3
  %t13 = call i64 @__vec_get(i64 %t11, i64 %t12)
  store i64 %t13, i64* %L4
  %t14 = load i64, i64* %L4
  store i64 %t14, i64* %L5
  %t15 = load i64, i64* %L1
  %t16 = load i64, i64* %L5
  %t17 = call i64 @__chrstr(i64 %t16)
  %t18 = call i64 @__str_cat(i64 %t15, i64 %t17)
  store i64 %t18, i64* %L1
  br label %lcond10
lend10:
  %t19 = add i64 0, 0
  %t20 = load i64, i64* %L1
  ret i64 %t20
}
define i64 @cis_lower(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @ccase_up(i64 %t1)
  %t3 = load i64, i64* %L0
  %t4.c = icmp ne i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  %t9.b = icmp ne i64 %t4, 0
  br i1 %t9.b, label %then9, label %else9
then9:
  %t5 = add i64 0, 1
  br label %thx9
thx9:
  br label %join9
else9:
  %t6 = load i64, i64* %L0
  %t7 = add i64 0, 223
  %t8.c = icmp eq i64 %t6, %t7
  %t8 = zext i1 %t8.c to i64
  br label %elx9
elx9:
  br label %join9
join9:
  %t9 = phi i64 [ %t5, %thx9 ], [ %t8, %elx9 ]
  ret i64 %t9
}
define i64 @cis_upper(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = call i64 @ccase_low(i64 %t1)
  %t3 = load i64, i64* %L0
  %t4.c = icmp ne i64 %t2, %t3
  %t4 = zext i1 %t4.c to i64
  ret i64 %t4
}
define i64 @__char_from_u32(i64 %p0) {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %t0 = add i64 0, %p0
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 0
  %t3.c = icmp sge i64 %t1, %t2
  %t3 = zext i1 %t3.c to i64
  %t8.b = icmp ne i64 %t3, 0
  br i1 %t8.b, label %then8, label %else8
then8:
  %t4 = load i64, i64* %L0
  %t5 = add i64 0, 1114111
  %t6.c = icmp sle i64 %t4, %t5
  %t6 = zext i1 %t6.c to i64
  br label %thx8
thx8:
  br label %join8
else8:
  %t7 = add i64 0, 0
  br label %elx8
elx8:
  br label %join8
join8:
  %t8 = phi i64 [ %t6, %thx8 ], [ %t7, %elx8 ]
  %t18.b = icmp ne i64 %t8, 0
  br i1 %t18.b, label %then18, label %else18
then18:
  %t9 = load i64, i64* %L0
  %t10 = add i64 0, 55296
  %t11.c = icmp slt i64 %t9, %t10
  %t11 = zext i1 %t11.c to i64
  %t16.b = icmp ne i64 %t11, 0
  br i1 %t16.b, label %then16, label %else16
then16:
  %t12 = add i64 0, 1
  br label %thx16
thx16:
  br label %join16
else16:
  %t13 = load i64, i64* %L0
  %t14 = add i64 0, 57343
  %t15.c = icmp sgt i64 %t13, %t14
  %t15 = zext i1 %t15.c to i64
  br label %elx16
elx16:
  br label %join16
join16:
  %t16 = phi i64 [ %t12, %thx16 ], [ %t15, %elx16 ]
  br label %thx18
thx18:
  br label %join18
else18:
  %t17 = add i64 0, 0
  br label %elx18
elx18:
  br label %join18
join18:
  %t18 = phi i64 [ %t16, %thx18 ], [ %t17, %elx18 ]
  %t24.b = icmp ne i64 %t18, 0
  br i1 %t24.b, label %then24, label %else24
then24:
  %t19 = load i64, i64* %L0
  %t20 = add i64 0, %t19
  %t21.mb = call i8* @malloc(i64 16)
  %t21.m = bitcast i8* %t21.mb to [2 x i64]*
  %t21.t = getelementptr [2 x i64], [2 x i64]* %t21.m, i64 0, i64 0
  store i64 1, i64* %t21.t
  %t21.p0 = getelementptr [2 x i64], [2 x i64]* %t21.m, i64 0, i64 1
  store i64 %t20, i64* %t21.p0
  %t21 = ptrtoint [2 x i64]* %t21.m to i64
  br label %thx24
thx24:
  br label %join24
else24:
  %t22 = add i64 0, 0
  %t23.mb = call i8* @malloc(i64 16)
  %t23.m = bitcast i8* %t23.mb to [2 x i64]*
  %t23.t = getelementptr [2 x i64], [2 x i64]* %t23.m, i64 0, i64 0
  store i64 0, i64* %t23.t
  %t23.p0 = getelementptr [2 x i64], [2 x i64]* %t23.m, i64 0, i64 1
  store i64 %t22, i64* %t23.p0
  %t23 = ptrtoint [2 x i64]* %t23.m to i64
  br label %elx24
elx24:
  br label %join24
join24:
  %t24 = phi i64 [ %t21, %thx24 ], [ %t23, %elx24 ]
  ret i64 %t24
}
define i64 @__usrc__() {
entry:
  %t0 = add i64 0, 0
  ret i64 %t0
}
define i64 @user_main() {
entry:
  %L0 = alloca i64
  store i64 0, i64* %L0
  %L1 = alloca i64
  store i64 0, i64* %L1
  %t0 = add i64 0, 40
  store i64 %t0, i64* %L0
  %t1 = load i64, i64* %L0
  %t2 = add i64 0, 2
  %t3 = add i64 %t1, %t2
  store i64 %t3, i64* %L1
  %t4 = load i64, i64* %L1
  %t5c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtdr, i64 0, i64 0), i64 %t4)
  %t5 = sext i32 %t5c to i64
  %t6c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.fmtnl, i64 0, i64 0))
  %t6 = sext i32 %t6c to i64
  %t7 = add i64 0, 0
  ret i64 %t7
}
define i64 @main(i32 %argc, i8** %argv) {
entry:
  %acw = sext i32 %argc to i64
  store i64 %acw, i64* @__argc
  %avw = ptrtoint i8** %argv to i64
  store i64 %avw, i64* @__argv
  %t0 = call i64 @user_main()
  %t1 = add i64 0, 0
  ret i64 %t1
}
