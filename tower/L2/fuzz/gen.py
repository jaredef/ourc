#!/usr/bin/env python3
# Differential C generator for the derived chain (ccc|rzir-ld|rzir-opt|rzir-ra|cf) vs gcc.
# Emits UB-free programs within ccc's supported subset that stress the paths that
# historically break tcc: local arrays, subscript load/store, pointers/deref, struct
# copy, helper calls, nested loops, conditionals, compound assignment.
# Determinism: seeded PRNG; all arithmetic kept in a safe small range; array indices
# always in-bounds; no division/modulo by zero; every variable initialized before use.
import random, sys

MOD = 100  # keep every value in [0,MOD) so the final `return` is < 256 and gcc/chain agree exactly

class Gen:
    def __init__(self, seed):
        self.r = random.Random(seed)
        self.vars = []      # scalar int vars in scope, all initialized
        self.arrs = []      # (name, size)
        self.ptrs = []      # pointer vars pointing at a scalar var
        self.aptrs = []     # (name, size) pointer aliasing an array base
        self.structs = []   # (varname, [field names]) local struct vars in scope
        self.nid = 0
        self.lines = []
        self.helpers = []
        self.helper_arity = {}
        self.decls = []     # top-level struct type decls

    def fresh(self, p):
        self.nid += 1
        return f"{p}{self.nid}"

    def emit(self, s):
        self.lines.append("  " + s)

    # a safe rvalue expression (value kept small via caller masking)
    def rexpr(self, depth=0):
        c = []
        c.append(lambda: str(self.r.randint(0, MOD - 1)))
        if self.vars:
            c.append(lambda: self.r.choice(self.vars))
        if self.arrs:
            def sub():
                a, n = self.r.choice(self.arrs)
                idx = self.r.randint(0, n - 1)
                return f"{a}[{idx}]"
            c.append(sub)
        if self.ptrs:
            c.append(lambda: "*" + self.r.choice(self.ptrs))
        if self.aptrs:
            c.append(self.rexpr_ptrindex)
        if self.structs:
            c.append(self.rexpr_field)
        if depth < 2:
            def bino():
                op = self.r.choice(["+", "-", "*"])
                return f"(({self.rexpr(depth+1)} {op} {self.rexpr(depth+1)}) % {MOD} + {MOD}) % {MOD}"
            c.append(bino)
            def bitop():
                op = self.r.choice(["&", "|", "^"])
                return f"(({self.rexpr(depth+1)} {op} {self.rexpr(depth+1)}) % {MOD} + {MOD}) % {MOD}"
            c.append(bitop)
            def shiftop():
                # small shift counts keep the value bounded; both operands non-negative
                return f"(({self.rexpr(depth+1)} {self.r.choice(['<<','>>'])} {self.r.randint(0,3)}) % {MOD} + {MOD}) % {MOD}"
            c.append(shiftop)
            def divop():
                # divisor forced non-zero (1..9) -> no UB
                op = self.r.choice(["/", "%"])
                return f"({self.rexpr(depth+1)} {op} ({self.rexpr(depth+1)} % 9 + 1))"
            c.append(divop)
            def relop():
                op = self.r.choice(["<", ">", "==", "<=", ">=", "!=", "&&", "||"])
                return f"(({self.rexpr(depth+1)}) {op} ({self.rexpr(depth+1)}))"
            c.append(relop)
            def ternary():
                op = self.r.choice(["<", ">", "==", "!="])
                return f"((({self.rexpr(depth+1)}) {op} ({self.rexpr(depth+1)})) ? ({self.rexpr(depth+1)}) : ({self.rexpr(depth+1)}))"
            c.append(ternary)
            # NOTE: compound-assignment-as-sub-expression (R101) is deliberately NOT generated here:
            # mutating a var inside a larger expression that also reads/mutates it is unspecified
            # evaluation order (esp. in call-arg lists), which diverges gcc vs chain for non-bug reasons.
            # That path is covered by dedicated KATs and the fib/big.c far-gate instead.
            def cast():
                # narrowing cast then re-normalize into [0,MOD); MOD<128 so (char) is value-preserving,
                # but the cast still exercises the truncation path (differential vs gcc catches R99-class bugs)
                ct = self.r.choice(["(char)", "(unsigned char)", "(short)", "(unsigned short)"])
                return f"(({ct}({self.rexpr(depth+1)})) % {MOD} + {MOD}) % {MOD}"
            c.append(cast)
        return self.r.choice(c)()

    def rexpr_field(self):
        sv, fields = self.r.choice(self.structs)
        return f"{sv}.{self.r.choice(fields)}"

    def new_scalar(self):
        v = self.fresh("v")
        self.emit(f"int {v} = {self.rexpr()};")
        self.vars.append(v)
        return v

    def new_array(self):
        a = self.fresh("a")
        n = self.r.randint(2, 6)
        self.emit(f"int {a}[{n}];")
        for i in range(n):
            self.emit(f"{a}[{i}] = {self.rexpr()};")
        self.arrs.append((a, n))
        return a

    def new_ptr(self):
        if not self.vars:
            self.new_scalar()
        v = self.r.choice(self.vars)
        p = self.fresh("p")
        self.emit(f"int *{p} = &{v};")
        self.ptrs.append(p)
        return p

    def new_aptr(self):
        # a pointer into an array base: int *q = a;  then q[i] / *(q+i) alias a[i]
        if not self.arrs:
            self.new_array()
        a, n = self.r.choice(self.arrs)
        q = self.fresh("q")
        self.emit(f"int *{q} = {a};")
        self.aptrs.append((q, n))
        return q

    def rexpr_ptrindex(self):
        # read through an array-pointer: q[i] or *(q+i), in bounds
        q, n = self.r.choice(self.aptrs)
        i = self.r.randint(0, n - 1)
        return self.r.choice([f"{q}[{i}]", f"*({q}+{i})", f"*({q}+{self.rexpr()}%{n})"])

    def new_struct(self):
        # a local struct with 2-3 int fields, all initialized; enables field read/write + struct copy
        tag = self.fresh("T")
        nf = self.r.randint(2, 3)
        fields = [f"f{k}" for k in range(nf)]
        self.decls.append(f"struct {tag} {{ {' '.join('int '+f+';' for f in fields)} }};")
        sv = self.fresh("s")
        self.emit(f"struct {tag} {sv};")
        for f in fields:
            self.emit(f"{sv}.{f} = {self.rexpr()};")
        self.structs.append((sv, fields))
        return sv

    def stmt_while(self):
        # count-down while over a fresh counter, touching an array element in bounds
        n = self.r.randint(2, 5)
        w = self.fresh("w")
        self.emit(f"int {w} = {n};")
        self.emit(f"while ({w} > 0) {{")
        self.emit(f"  {w} = {w} - 1;")
        if self.vars:
            v = self.r.choice(self.vars)
            self.emit(f"  {v} = (({v} + {w}) % {MOD} + {MOD}) % {MOD};")
        self.emit("}")

    def stmt_structcopy(self):
        # whole-struct copy between two vars of the same type is hard to guarantee same-type here;
        # instead copy a field and re-read (exercises field load/store ordering)
        sv, fields = self.r.choice(self.structs)
        f1, f2 = self.r.choice(fields), self.r.choice(fields)
        self.emit(f"{sv}.{f1} = {sv}.{f2};")

    def stmt_assign(self):
        if self.vars and self.r.random() < 0.5:
            v = self.r.choice(self.vars)
            op = self.r.choice(["=", "+=", "-=", "*="])
            if op == "=":
                self.emit(f"{v} = {self.rexpr()};")
            else:
                self.emit(f"{v} = (({v} {op[0]} ({self.rexpr()})) % {MOD} + {MOD}) % {MOD};")
        elif self.arrs:
            a, n = self.r.choice(self.arrs)
            i = self.r.randint(0, n - 1)
            self.emit(f"{a}[{i}] = {self.rexpr()};")
        elif self.aptrs and self.r.random() < 0.5:
            q, n = self.r.choice(self.aptrs)
            i = self.r.randint(0, n - 1)
            self.emit(self.r.choice([f"{q}[{i}] = {self.rexpr()};",
                                     f"*({q}+{i}) = {self.rexpr()};"]))
        elif self.ptrs:
            p = self.r.choice(self.ptrs)
            self.emit(f"*{p} = {self.rexpr()};")

    def stmt_if(self):
        v = self.rexpr()
        w = self.rexpr()
        op = self.r.choice(["<", ">", "==", "<=", ">=", "!="])
        self.emit(f"if (({v}) {op} ({w})) {{")
        self.stmt_assign()
        self.emit("}")

    def stmt_loop(self):
        # bounded counting loop that touches an array
        if not self.arrs:
            self.new_array()
        a, n = self.r.choice(self.arrs)
        i = self.fresh("i")
        self.emit(f"for (int {i} = 0; {i} < {n}; {i}++) {{")
        self.emit(f"  {a}[{i}] = (({a}[{i}] + {i}) % {MOD} + {MOD}) % {MOD};")
        self.emit("}")

    def gen_helper(self):
        # a small pure helper with 2-5 params (exercises the call ABI / register pressure at the boundary)
        h = self.fresh("h")
        np = self.r.randint(2, 5)
        ps = [f"x{k}" for k in range(np)]
        expr = ps[0]
        for p in ps[1:]:
            expr = f"(({expr} {self.r.choice(['+','-','*'])} {p}) % {MOD} + {MOD}) % {MOD}"
        self.helpers.append(
            f"int {h}({', '.join('int '+p for p in ps)}) {{ return {expr}; }}")
        self.helper_arity[h] = np
        return h

    def gen_recursive(self):
        # a bounded recursive helper: countdown sum, no UB (n kept small at the call site)
        h = self.fresh("rec")
        self.helpers.append(
            f"int {h}(int n) {{ if (n <= 0) return 0; return ((n + {h}(n - 1)) % {MOD} + {MOD}) % {MOD}; }}")
        self.helper_arity[h] = 1
        return h

    def stmt_bigexpr(self):
        # high register pressure: assign a var from an expression referencing many live vars
        if len(self.vars) < 2:
            return
        v = self.r.choice(self.vars)
        terms = self.r.sample(self.vars, min(len(self.vars), self.r.randint(2, 5)))
        e = terms[0]
        for t in terms[1:]:
            e = f"(({e} {self.r.choice(['+','-','*','^','&','|'])} {t}) % {MOD} + {MOD}) % {MOD}"
        self.emit(f"{v} = {e};")

    def stmt_nestedloop(self):
        if not self.arrs:
            self.new_array()
        a, n = self.r.choice(self.arrs)
        i, j = self.fresh("i"), self.fresh("j")
        self.emit(f"for (int {i} = 0; {i} < {n}; {i}++) {{")
        self.emit(f"  for (int {j} = 0; {j} < {n}; {j}++) {{")
        self.emit(f"    {a}[{j}] = (({a}[{j}] + {i}) % {MOD} + {MOD}) % {MOD};")
        self.emit("  }")
        self.emit("}")

    def build(self):
        nvars = self.r.randint(1, 3)
        for _ in range(nvars):
            self.new_scalar()
        if self.r.random() < 0.8:
            self.new_array()
        if self.r.random() < 0.5:
            self.new_ptr()
        if self.r.random() < 0.7:
            self.new_aptr()
        if self.r.random() < 0.6:
            self.new_struct()
        helper = self.gen_helper() if self.r.random() < 0.6 else None
        rec = self.gen_recursive() if self.r.random() < 0.4 else None

        nstmt = self.r.randint(6, 14)
        for _ in range(nstmt):
            k = self.r.random()
            if k < 0.22:
                self.stmt_assign()
            elif k < 0.36:
                self.stmt_if()
            elif k < 0.48:
                self.stmt_loop()
            elif k < 0.58:
                self.stmt_nestedloop()
            elif k < 0.68:
                self.stmt_while()
            elif k < 0.76:
                self.stmt_bigexpr()
            elif k < 0.84 and self.structs:
                self.stmt_structcopy()
            elif k < 0.90 and rec and self.vars:
                v = self.r.choice(self.vars)
                self.emit(f"{v} = {rec}({self.rexpr()} % 7);")
            elif k < 0.95 and helper and self.vars:
                v = self.r.choice(self.vars)
                args = ", ".join(self.rexpr() for _ in range(self.helper_arity[helper]))
                self.emit(f"{v} = {helper}({args});")
            else:
                self.new_scalar()

        ret = self.rexpr()
        self.emit(f"int _r = ({ret}) % {MOD};")
        self.emit("return _r < 0 ? -_r : _r;")

        body = "\n".join(self.lines)
        hs = "\n".join(self.helpers)
        ds = "\n".join(self.decls)
        return f"{ds}\n{hs}\nint main() {{\n{body}\n}}\n"

if __name__ == "__main__":
    seed = int(sys.argv[1])
    print(Gen(seed).build())
