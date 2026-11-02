/* rzc -- the derived toolchain's build driver (Phase K), written in the rzrs subset.
   Runs each compile stage as a child process and CHECKS ITS EXIT CODE before advancing, so a
   frontend compile error surfaces (the child's stderr passes through, and rzc returns its code)
   instead of being swallowed by a bare pipe.

   usage:
     rzc build <src.rs> <out.elf>
     rzc run   <src.rs>
     rzc test  <src.rs>

   The five stage binaries are found NEXT TO rzc itself (via readlink /proc/self/exe): a sibling
   `rzrs.self`, `rzir-ld`, `rzir-opt`, `rzir-ra`, `cf`. So a self-contained toolchain directory holds
   rzc + its stages and `rzc build foo.rs foo.elf` just works.

   `build` lowers <src.rs> to <out.elf>; `run` builds to a temp ELF and execs it (the program's
   exit code becomes rzc's); `test` builds + runs and reports ok/FAILED.

   Pipeline -- each stage runs with stdin<-file, stdout->file, and its exit code checked:
     front:  src.rs      -> /tmp/rzc.a
     ld:     /tmp/rzc.a  -> /tmp/rzc.b
     opt:    /tmp/rzc.b  -> /tmp/rzc.c
     ra:     /tmp/rzc.c  -> /tmp/rzc.d
     cf:     /tmp/rzc.d  -> out                                                                */

/* one contiguous scratch; each string lives at a fixed offset (256B slabs for paths, 64B for temps) */
static mut MEM:  [u8; 4096] = [0; 4096];
static mut ARGV: [i64; 2]   = [0, 0];
static mut ENVP: [i64; 1]   = [0];
static mut ST:   [i32; 4]   = [0, 0, 0, 0];
static mut CUT:  i32        = 0;               /* length of rzc's own directory prefix (incl. trailing '/') */
static mut SBUF: [u8; 400000] = [0; 400000];   /* combined source (main + inlined `mod NAME;` files) */
static mut SLEN: i32        = 0;               /* current length of SBUF */
static mut SRCCUT: i32      = 0;               /* directory-prefix length of the source path (for module paths) */

fn off_src() -> i32 { 0 }
fn off_out() -> i32 { 256 }
fn off_front() -> i32 { 512 }
fn off_ld() -> i32 { 768 }
fn off_opt() -> i32 { 1024 }
fn off_ra() -> i32 { 1280 }
fn off_cf() -> i32 { 1536 }
fn off_ta() -> i32 { 1792 }
fn off_tb() -> i32 { 1856 }
fn off_tc() -> i32 { 1920 }
fn off_td() -> i32 { 1984 }
fn off_msg() -> i32 { 2048 }
fn off_self() -> i32 { 2304 }                  /* readlink(/proc/self/exe) result + derived dir prefix (512B) */
fn off_plit() -> i32 { 2816 }                  /* "/proc/self/exe" NUL-terminated (32B) */
fn off_modp() -> i32 { 3200 }                  /* a module's resolved path srcdir+NAME+".rs" (256B) */
fn off_csrc() -> i32 { 3456 }                  /* "/tmp/rzc.src" -- the combined source path (32B) */

fn setb(off: i32, j: i32, c: i32) { unsafe { MEM[(off + j) as usize] = c as u8; } }
fn addr(off: i32) -> i64 { unsafe { return &MEM[off as usize] as i64; } }

/* copy argv[argi] (NUL-terminated) into MEM at off */
fn argcopy(off: i32, argi: i32) {
    unsafe {
        let mut j = 0;
        let mut c = __argb(argi, j);
        while c != 0 { setb(off, j, c); j = j + 1; c = __argb(argi, j); }
        setb(off, j, 0);
    }
}

/* copy a NUL-terminated byte-string literal into MEM at off */
fn litcopy(off: i32, lit: &[u8]) {
    unsafe {
        let mut j = 0;
        while j < lit.len() { setb(off, j, lit[j]); j = j + 1; }
        setb(off, j, 0);
    }
}

/* 1 iff argv[argi] equals the literal `lit` exactly (both NUL-terminated) */
fn argeq(argi: i32, lit: &[u8]) -> i32 {
    unsafe {
        let mut j = 0;
        while j < lit.len() { if __argb(argi, j) != lit[j] { return 0; } j = j + 1; }
        if __argb(argi, lit.len()) != 0 { return 0; }
        return 1;
    }
}

/* resolve rzc's own path (readlink /proc/self/exe -> MEM@off_self), and set CUT to the length of the
   directory prefix (through the last '/') so stage siblings can be built as prefix + name */
fn locate() {
    unsafe {
        litcopy(off_plit(), b"/proc/self/exe");
        let n = __syscall(89, addr(off_plit()), addr(off_self()), 500);   /* readlink -> byte count (no NUL) */
        CUT = 0;
        let mut k = n - 1;
        while k >= 0 {
            if MEM[(off_self() + k) as usize] == 47 { CUT = k + 1; k = 0 - 1; } else { k = k - 1; }
        }
    }
}

/* write rzc's directory prefix + `name` (+ NUL) into MEM at off -> a sibling stage-binary path */
fn stagepath(off: i32, name: &[u8]) {
    unsafe {
        let mut j = 0;
        while j < CUT { setb(off, j, MEM[(off_self() + j) as usize]); j = j + 1; }
        let mut m = 0;
        while m < name.len() { setb(off, CUT + m, name[m]); m = m + 1; }
        setb(off, CUT + name.len(), 0);
    }
}

fn temp_paths() {
    unsafe {
        litcopy(off_ta(), b"/tmp/rzc.a");
        litcopy(off_tb(), b"/tmp/rzc.b");
        litcopy(off_tc(), b"/tmp/rzc.c");
        litcopy(off_td(), b"/tmp/rzc.d");
    }
}

/* write a byte-string literal + newline to stderr */
fn emsgln(lit: &[u8]) {
    unsafe {
        litcopy(off_msg(), lit);
        setb(off_msg(), lit.len(), 10);
        __syscall(1, 2, addr(off_msg()), lit.len() + 1);
    }
}

fn eline()  { unsafe { emsgln(b"rzc: stage failed"); } }
fn eusage() { unsafe { emsgln(b"usage: rzc build <src.rs> <out.elf> | run <src.rs> | test <src.rs>"); } }

/* fork a child running MEM@pathoff with stdin<-MEM@inoff, stdout->MEM@outoff; wait; return exit code */
fn spawn(pathoff: i32, inoff: i32, outoff: i32) -> i32 {
    unsafe {
        let pid = __syscall(57, 0, 0, 0);                        /* fork */
        if pid == 0 {
            let fdin = __syscall(2, addr(inoff), 0, 0);          /* open(in, O_RDONLY) */
            __syscall(33, fdin, 0, 0);                           /* dup2(fdin, 0) */
            let fdout = __syscall(2, addr(outoff), 577, 493);    /* open(out, O_WRONLY|O_CREAT|O_TRUNC, 0755) */
            __syscall(33, fdout, 1, 0);                          /* dup2(fdout, 1) */
            ARGV[0] = addr(pathoff);
            ARGV[1] = 0;
            ENVP[0] = 0;
            __syscall(59, addr(pathoff), &ARGV[0], &ENVP[0]);    /* execve */
            __syscall(60, 127, 0, 0);                            /* exec failed */
        }
        __syscall(61, pid, &ST[0], 0, 0);                        /* wait4(pid, &status, 0, 0) */
        return (ST[0] / 256) % 256;
    }
}

/* run the 5-stage pipeline src(off_src) -> out(off_out); 0 on success, else the failing stage's code */
fn do_build() -> i32 {
    unsafe {
        temp_paths();
        let r1 = spawn(off_front(), off_src(), off_ta());
        if r1 != 0 { eline(); return r1; }
        let r2 = spawn(off_ld(),  off_ta(), off_tb());
        if r2 != 0 { eline(); return r2; }
        let r3 = spawn(off_opt(), off_tb(), off_tc());
        if r3 != 0 { eline(); return r3; }
        let r4 = spawn(off_ra(),  off_tc(), off_td());
        if r4 != 0 { eline(); return r4; }
        let r5 = spawn(off_cf(),  off_td(), off_out());
        if r5 != 0 { eline(); return r5; }
        return 0;
    }
}

/* exec MEM@off_out with no args; on success this replaces rzc and the program's exit becomes rzc's */
fn exec_out() -> i32 {
    unsafe {
        ARGV[0] = addr(off_out());
        ARGV[1] = 0;
        ENVP[0] = 0;
        __syscall(59, addr(off_out()), &ARGV[0], &ENVP[0]);
        return 126;                                             /* exec failed */
    }
}

/* fork + exec MEM@off_out, wait, return its exit code (so the caller can report a result) */
fn run_capture() -> i32 {
    unsafe {
        let pid = __syscall(57, 0, 0, 0);
        if pid == 0 {
            ARGV[0] = addr(off_out());
            ARGV[1] = 0;
            ENVP[0] = 0;
            __syscall(59, addr(off_out()), &ARGV[0], &ENVP[0]);
            __syscall(60, 126, 0, 0);
        }
        __syscall(61, pid, &ST[0], 0, 0);
        return (ST[0] / 256) % 256;
    }
}

/* --- mod resolution: inline every top-level `mod NAME;` from the source's directory --- */
fn mlen(off: i32) -> i32 { unsafe { let mut k = 0; while MEM[(off + k) as usize] != 0 { k = k + 1; } return k; } }
fn dir_cut(off: i32, len: i32) -> i32 {                    /* prefix length through the last '/', else 0 */
    unsafe { let mut k = len - 1; while k >= 0 { if MEM[(off + k) as usize] == 47 { return k + 1; } k = k - 1; } return 0; }
}
fn is_idchar(c: i32) -> i32 {
    let mut r = 0;
    if c >= 97 && c <= 122 { r = 1; }
    if c >= 65 && c <= 90 { r = 1; }
    if c >= 48 && c <= 57 { r = 1; }
    if c == 95 { r = 1; }
    r
}
/* open+read the whole file at MEM@pathoff into SBUF at SLEN; 0 ok, 1 open-fail */
fn read_into_sbuf(pathoff: i32) -> i32 {
    unsafe {
        let fd = __syscall(2, addr(pathoff), 0, 0);        /* open(path, O_RDONLY) */
        if fd < 0 { return 1; }
        loop {
            let n = __syscall(0, fd, &SBUF[SLEN as usize], 65536);   /* read */
            if n <= 0 { break; }
            SLEN = SLEN + n;
        }
        __syscall(3, fd, 0, 0);                            /* close */
        return 0;
    }
}
/* write SBUF[0..SLEN] to the file at MEM@pathoff */
fn write_sbuf(pathoff: i32) {
    unsafe {
        let fd = __syscall(2, addr(pathoff), 577, 493);    /* open(O_WRONLY|O_CREAT|O_TRUNC, 0755) */
        __syscall(1, fd, &SBUF[0], SLEN);                  /* write */
        __syscall(3, fd, 0, 0);
    }
}
/* MEM@dest = srcdir + SBUF[ns..ns+nl] + ".rs" + NUL  (the path of module NAME) */
fn build_mod_path(dest: i32, ns: i32, nl: i32) {
    unsafe {
        let mut j = 0;
        while j < SRCCUT { setb(dest, j, MEM[(off_src() + j) as usize]); j = j + 1; }
        let mut m = 0;
        while m < nl { setb(dest, SRCCUT + m, SBUF[(ns + m) as usize]); m = m + 1; }
        setb(dest, SRCCUT + nl, 46); setb(dest, SRCCUT + nl + 1, 114); setb(dest, SRCCUT + nl + 2, 115);   /* ".rs" */
        setb(dest, SRCCUT + nl + 3, 0);
    }
}
/* a top-level `mod ` at SBUF[p] (start-of-file or preceded by a newline) */
fn at_mod(p: i32) -> i32 {
    unsafe {
        if SBUF[p as usize] != 109 { return 0; }
        if SBUF[(p + 1) as usize] != 111 { return 0; }
        if SBUF[(p + 2) as usize] != 100 { return 0; }
        if SBUF[(p + 3) as usize] != 32 { return 0; }
        if p == 0 { return 1; }
        if SBUF[(p - 1) as usize] == 10 { return 1; }
        return 0;
    }
}
/* read the main source, inline each top-level `mod NAME;` (single level), write /tmp/rzc.src, point off_src at
   it. Returns the number of modules that could not be opened (0 = ok). */
fn resolve_mods() -> i32 {
    unsafe {
        let mut missing = 0;
        SLEN = 0;
        SRCCUT = dir_cut(off_src(), mlen(off_src()));
        if read_into_sbuf(off_src()) != 0 { return 0; }    /* can't open the main source -> let the build fail normally */
        let mainlen = SLEN;                                /* scan only the original main source for `mod` decls */
        let mut p = 0;
        while p + 4 < mainlen {
            if at_mod(p) == 1 {
                let ns = p + 4;
                let mut e = ns;
                while e < mainlen && is_idchar(SBUF[e as usize]) == 1 { e = e + 1; }
                let mut q = e;
                while q < mainlen && (SBUF[q as usize] == 32 || SBUF[q as usize] == 9) { q = q + 1; }
                if q < mainlen && SBUF[q as usize] == 59 {  /* `mod NAME;` */
                    build_mod_path(off_modp(), ns, e - ns);
                    SBUF[SLEN as usize] = 10; SLEN = SLEN + 1;   /* newline before the appended module */
                    if read_into_sbuf(off_modp()) != 0 {
                        litcopy(off_msg(), b"rzc: cannot open module: ");
                        __syscall(1, 2, addr(off_msg()), 25);
                        __syscall(1, 2, addr(off_modp()), mlen(off_modp()));
                        setb(off_msg(), 0, 10); __syscall(1, 2, addr(off_msg()), 1);
                        missing = missing + 1;
                    }
                }
                p = e;
            } else { p = p + 1; }
        }
        litcopy(off_csrc(), b"/tmp/rzc.src");
        write_sbuf(off_csrc());
        litcopy(off_src(), b"/tmp/rzc.src");               /* build the combined source */
        return missing;
    }
}

fn main() -> i32 {
    unsafe {
        if __argc() < 3 { eusage(); return 2; }
        locate();                                          /* find the sibling stage binaries next to rzc */
        stagepath(off_front(), b"rzrs.self");
        stagepath(off_ld(),    b"rzir-ld");
        stagepath(off_opt(),   b"rzir-opt");
        stagepath(off_ra(),    b"rzir-ra");
        stagepath(off_cf(),    b"cf");
        argcopy(off_src(), 2);
        if resolve_mods() != 0 { return 3; }               /* inline `mod NAME;` files; a missing module fails the build */

        if argeq(1, b"build") == 1 {
            if __argc() < 4 { eusage(); return 2; }
            argcopy(off_out(), 3);
            return do_build();
        }
        if argeq(1, b"run") == 1 {
            litcopy(off_out(), b"/tmp/rzc.out");
            let rc = do_build();
            if rc != 0 { return rc; }
            return exec_out();
        }
        if argeq(1, b"test") == 1 {
            litcopy(off_out(), b"/tmp/rzc.out");
            let rc = do_build();
            if rc != 0 { return rc; }
            let tr = run_capture();
            if tr == 0 { emsgln(b"test result: ok"); return 0; }
            emsgln(b"test result: FAILED"); return 1;
        }

        eusage();
        return 2;
    }
}
