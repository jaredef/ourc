// ourc-bootstrap -- build the compiler tower and ourc from the hand-written hex seed, in Rust.
//
// This is release.sh (and the tower scripts it runs) as one program: the same steps, in the same order, on the
// same inputs, with every produced artifact hashed and checked against pins.txt. It adds no trust -- this
// program is itself compiled by rustc -- but it is a second, independent driver of the from-hex build: when its
// artifacts equal the shell build's byte for byte, both drivers would have to be subverted in agreement.
//
// The hex root is packed here directly: hex0.hand.hex and derived-hex1.labeled.hex are decoded by this
// program's own packers (below), so neither committed seed binary is needed; each packed seed must also
// rebuild itself from its source. Every later stage is run as a pipeline of tower programs (stdin -> stdout),
// exactly as the shell scripts run them. Only the gates' test batteries are left out; the checks the release
// rests on (seed washes out, C2, the ourc fixed point) are kept.
//
// Usage:  ourc-bootstrap OUTDIR                    use the source tree embedded at build time
//         ourc-bootstrap --repo CHECKOUT OUTDIR     use a source checkout instead
//         ourc-bootstrap --vendor CHECKOUT DIR      copy the sources this build reads into DIR (for packaging)
//         --resume                                  keep an existing artifact whose hash already matches its pin
// Needs llc, opt and cc on PATH (the same host tools rustc's own builds use).
mod sha256;
mod ui;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

include!(concat!(env!("OUT_DIR"), "/embed.rs"));

const PINS: &str = include_str!("../pins.txt");
const CRATES: [&str; 18] = ["ospan", "oerrors", "oindex", "oarena", "oabi", "olex", "oast", "oresolve", "ohir", "olower",
    "omir", "omirtrans", "ocodegen", "osession", "oparse", "otypeck", "omono", "oborrowck"];
// the SEM0 staircase (tower/sem0/stages/SEM0-hoist/sem0-stage.sh): hop N compiles hops/hNN.rs; h23 and the dev
// source are iterated to byte fixpoints
const HOPS: usize = 28;
const SEM0D: &str = "tower/sem0/stages/SEM0-hoist/hops";
const DEV: &str = "tower/sem0/stages/S1-lex-parse-ast/s1_parser.rs";
const LINK_RS: &str = "ourc/rt/link.rs";
const RT: &str = "ourc/rt/runtime.rzir";

/// Every source file the build reads, relative to the repository root.
fn sources() -> Vec<String> {
    let mut v: Vec<String> = [
        "tower/L0/hex0/hex0.hand.hex", "tower/L0/hex0/hex0.bin", "tower/L0/hex1/derived-hex1.labeled.hex",
        "tower/L0/hex1/hex1.bin", "tower/L0/hex2/derived-hex2.labeled.hex", "tower/L0/m0/m0.labeled.hex",
        "tower/L0/emitter/rzir-to-elf-cf.m0", "tower/L1/c-frontend/c-to-rzir.m0", "tower/L2/ccc.c",
        "tower/L2/link/rzir-ld.c", "tower/L2/opt/rzir-opt.c", "tower/L2/ra/rzir-ra.c", "tower/tcc/rzcpp.c",
        "tower/backend-toolchain/rzrs-seed/shim.c", "tower/backend-toolchain/rzrs-seed/headers/stdio.h",
        "tower/backend-toolchain/rzrs-seed/headers/stdlib.h", "tower/backend-toolchain/rzrs-seed/headers/string.h",
        "tower/L3/rzrs.c", "tower/L4/L4.rzrs.rs", "tower/L5/L5.rzrs.rs", "tower/L5/cf.hoist.seed",
        "tower/backend-toolchain/rzir-ld.rs", "tower/backend-toolchain/rzir-opt.rs", "tower/backend-toolchain/rzir-ra.rs",
        "tower/backend-cf/cf.rs", "tower/L6/L6.rzrs.rs", DEV, "tower/backend-llvm/rzir2ll.rs", LINK_RS, RT,
        "ourc/src/odrv.rs", "ourc/README.md", "ourc/LIMITATIONS.md", "LICENSE-MIT", "LICENSE-APACHE",
    ].iter().map(|s| s.to_string()).collect();
    for n in 0..HOPS { v.push(format!("{SEM0D}/h{n:02}.rs")); }
    for c in CRATES { v.push(format!("ourc/src/{c}.rs")); }
    v
}

// ---------------------------------------------------------------------------------------------------------------
// The hex packers. hex0: pairs of hex digits (either case) are bytes, `#` comments to end of line, all else is
// ignored. hex1 adds single-character labels: `:X` marks the current output offset, `}X` emits the rel32 from
// the end of that 4-byte field to X; its digits are lowercase only. Both are checked against the seeds below.

fn nib(c: u8, upper: bool) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' if upper => Some(c - b'A' + 10),
        _ => None,
    }
}

fn pack_hex0(src: &[u8]) -> Vec<u8> {
    let (mut out, mut hi, mut i) = (Vec::new(), None::<u8>, 0);
    while i < src.len() {
        let c = src[i];
        i += 1;
        if c == b'#' { while i < src.len() && src[i] != b'\n' { i += 1; } continue; }
        if let Some(n) = nib(c, true) {
            match hi { None => hi = Some(n), Some(h) => { out.push(h << 4 | n); hi = None; } }
        }
    }
    out
}

fn pack_hex1(src: &[u8]) -> Vec<u8> {
    let mut labels = [0i64; 256];
    let mut out = Vec::new();
    for pass in 0..2 {
        out.clear();
        let (mut hi, mut i) = (None::<u8>, 0);
        while i < src.len() {
            let c = src[i];
            i += 1;
            match c {
                b'#' => { while i < src.len() && src[i] != b'\n' { i += 1; } }
                b':' if i < src.len() => { labels[src[i] as usize] = out.len() as i64; i += 1; }
                b'}' if i < src.len() => {
                    let t = labels[src[i] as usize];
                    i += 1;
                    let rel = if pass == 0 { 0 } else { (t - (out.len() as i64 + 4)) as i32 };
                    out.extend_from_slice(&rel.to_le_bytes());
                }
                _ => if let Some(n) = nib(c, false) {
                    match hi { None => hi = Some(n), Some(h) => { out.push(h << 4 | n); hi = None; } }
                },
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------------
// Running the tower: a step feeds a list of parts (files and literal text) to a pipeline of programs.

enum Part { F(PathBuf), T(String) }
use Part::{F, T};

struct Cmd { prog: PathBuf, args: Vec<OsString> }
fn x(p: &Path) -> Cmd { Cmd { prog: p.to_path_buf(), args: vec![] } }
fn xa(p: &Path, args: &[&str]) -> Cmd { Cmd { prog: p.to_path_buf(), args: args.iter().map(OsString::from).collect() } }

/// Run `input | cmds[0] | cmds[1] | ...` with the last stdout going to `out`; true when every stage exited 0.
fn pipe(input: Vec<Part>, cmds: &[Cmd], out: &Path) -> io::Result<bool> {
    // several parts are joined into a file first, as sem0-stage.sh's `{ echo '//@S9'; cat; } > $t` shim does:
    // SEM0 and other tower programs read stdin as a regular file, and misread a pipe
    if input.len() > 1 {
        let joined = PathBuf::from(format!("{}.stdin", out.display()));
        let mut f = fs::File::create(&joined)?;
        for p in &input { match p { F(p) => { io::copy(&mut fs::File::open(p)?, &mut f)?; } T(s) => f.write_all(s.as_bytes())? } }
        drop(f);
        let r = pipe(vec![F(joined.clone())], cmds, out);
        let _ = fs::remove_file(&joined);
        return r;
    }
    let mut kids = Vec::new();
    let mut prev: Option<std::process::ChildStdout> = None;
    let mut feed = None;
    for (k, c) in cmds.iter().enumerate() {
        let mut cmd = Command::new(&c.prog);
        cmd.args(&c.args).stderr(Stdio::null());
        // a lone input file is stdin itself, as `< file` makes it: some tower programs size their input from it
        let lone = match (k, &input[..]) { (0, [F(f)]) => Some(f.clone()), _ => None };
        match (prev.take(), lone) {
            (Some(o), _) => { cmd.stdin(Stdio::from(o)); }
            (None, Some(f)) => { cmd.stdin(Stdio::from(fs::File::open(&f).map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", f.display())))?)); }
            (None, None) => { cmd.stdin(Stdio::piped()); }
        }
        if k + 1 == cmds.len() { cmd.stdout(Stdio::from(fs::File::create(out)?)); } else { cmd.stdout(Stdio::piped()); }
        let mut child = cmd.spawn().map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", c.prog.display())))?;
        if k == 0 && child.stdin.is_some() {
            let mut w = child.stdin.take().unwrap();
            let parts: Vec<Part> = input.iter().map(|p| match p { F(f) => F(f.clone()), T(s) => T(s.clone()) }).collect();
            feed = Some(std::thread::spawn(move || -> io::Result<()> {
                for p in parts {
                    let r = match p { F(f) => fs::read(&f).and_then(|b| w.write_all(&b)), T(s) => w.write_all(s.as_bytes()) };
                    if let Err(e) = r { if e.kind() == io::ErrorKind::BrokenPipe { return Ok(()); } return Err(e); }
                }
                Ok(())
            }));
        }
        if k + 1 < cmds.len() { prev = child.stdout.take(); }
        kids.push(child);
    }
    let mut ok = true;
    for mut kid in kids { ok &= kid.wait()?.success(); }
    if let Some(f) = feed { f.join().unwrap()?; }
    Ok(ok)
}

/// Run a host tool with arguments only (llc, opt, cc).
fn base(p: &Path) -> String { p.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default() }

fn tool(prog: &str, args: &[&Path], flags: &[&str]) -> Result<(), String> {
    ui::running(format!("{prog} {}", args.first().map(|p| base(p)).unwrap_or_default()));
    let mut cmd = Command::new(prog);
    let mut a: Vec<OsString> = flags.iter().map(OsString::from).collect();
    // flags are given with `{}` placeholders for the paths, in order
    let mut it = args.iter();
    for f in a.iter_mut() { if f == "{}" { *f = it.next().unwrap().as_os_str().to_os_string(); } }
    cmd.args(&a).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    match cmd.status() { Ok(s) if s.success() => Ok(()), Ok(s) => Err(format!("{prog} exited {s}")), Err(e) => Err(format!("{prog}: {e}")) }
}

fn set_exec(p: &Path) { if let Ok(m) = fs::metadata(p) { let mut pm = m.permissions(); pm.set_mode(0o755); let _ = fs::set_permissions(p, pm); } }

// RLIMIT_STACK: the big frontends recurse deeply (release.sh runs everything under `ulimit -s unlimited`).
#[repr(C)] struct RLimit { cur: u64, max: u64 }
extern "C" { fn setrlimit(res: i32, lim: *const RLimit) -> i32; fn getrlimit(res: i32, lim: *mut RLimit) -> i32; }
fn unlimit_stack() {
    unsafe {
        let mut l = RLimit { cur: 0, max: 0 };
        getrlimit(3, &mut l);
        let want = RLimit { cur: l.max, max: l.max };
        setrlimit(3, &want);
    }
}

// ---------------------------------------------------------------------------------------------------------------

struct B {
    src: PathBuf,           // repository root (or the extracted embedded tree)
    out: PathBuf,
    sp: PathBuf,            // tower scratch, laid out as build-tower.sh's $SP
    pins: HashMap<String, (String, bool)>, // name -> (sha256, chain class)
    resume: bool,
    log: Vec<String>,       // PROVENANCE lines
    host_drift: usize,
    t0: Instant,
}

type R<T> = Result<T, String>;

impl B {
    fn s(&self, rel: &str) -> PathBuf { self.src.join(rel) }
    fn p(&self, name: &str) -> PathBuf { self.sp.join(name) }
    fn key(&self, path: &Path) -> String {
        path.strip_prefix(&self.out).map(|r| r.to_string_lossy().into_owned()).unwrap_or_else(|_| path.display().to_string())
    }

    fn reusable(&self, out: &Path) -> bool {
        if !self.resume { return false; }
        let Some((want, _)) = self.pins.get(&self.key(out)) else { return false };
        fs::read(out).map(|b| &sha256::hex(&b) == want).unwrap_or(false)
    }

    /// Record an artifact: size floor (a backend overflow leaves a small executable stub), pin check, provenance.
    fn seal(&mut self, out: &Path, min: u64, what: &str) -> R<()> {
        set_exec(out);
        let b = fs::read(out).map_err(|e| format!("{what}: {e}"))?;
        if (b.len() as u64) < min { return Err(format!("{what}: {} is {} bytes (< {min}) -- a stub, not an artifact", out.display(), b.len())); }
        let h = sha256::hex(&b);
        let k = self.key(out);
        let verdict = match self.pins.get(&k) {
            None => "unpinned".to_string(),
            Some((w, _)) if *w == h => "== pin".to_string(),
            Some((w, true)) => return Err(format!("{what}: {k} is {h}, pinned {w} -- the from-hex chain diverged")),
            Some((w, false)) => { self.host_drift += 1; format!("!= pin {} (host llc/cc differ)", &w[..12]) }
        };
        let class = if verdict == "== pin" { "ok" } else if verdict == "unpinned" { "new" } else { "drift" };
        ui::seal(format!("  [{:>6.0}s] {:<26} {}  {:>10} B  {verdict}", self.t0.elapsed().as_secs_f64(), k, &h[..16], b.len()), class);
        self.log.push(format!("{h}  {k}"));
        if ui::stop_requested() { return Err(format!("stopped at your request after {k}; rerun with --resume to continue")); }
        Ok(())
    }

    /// One tower step: `input | cmds > out`, then seal. `need_ok` mirrors the scripts' `|| FAIL` on the exit code.
    fn step(&mut self, out: &Path, input: Vec<Part>, cmds: &[Cmd], min: u64, need_ok: bool, what: &str) -> R<()> {
        if self.reusable(out) { return self.seal(out, min, what); }
        ui::running(format!("{what}  ->  {}", self.key(out)));
        let ok = pipe(input, cmds, out).map_err(|e| format!("{what}: {e}"))?;
        if need_ok && !ok { return Err(format!("{what}: a stage exited non-zero")); }
        self.seal(out, min, what)
    }

    // the backend spines (lib/scratch.sh, sem0-stage.sh)
    fn l2(&self, st: &str) -> PathBuf { self.p(&format!("seeds/L2/{st}.bin")) }
    fn chain_c(&self, cf: &Path) -> Vec<Cmd> { vec![x(&self.l2("link/rzir-ld")), x(&self.l2("opt/rzir-opt")), x(&self.l2("ra/rzir-ra")), x(cf)] }
    fn chain_rs(&self, cf: &Path) -> Vec<Cmd> { vec![x(&self.p("B.ld")), x(&self.p("B.opt")), x(&self.p("B.ra")), x(cf)] }
    fn chain3(&self) -> Vec<Cmd> { vec![x(&self.p("B.ld")), x(&self.l2("opt/rzir-opt")), x(&self.l2("ra/rzir-ra")), x(&self.p("B.cf.hoist"))] }
    fn with(first: Cmd, mut rest: Vec<Cmd>) -> Vec<Cmd> { rest.insert(0, first); rest }

    // ---- 1. seeds (lib/derive-seeds.sh), from the hex packed above -----------------------------------------
    fn seeds(&mut self) -> R<()> {
        ui::phase(1, &format!("seeds from the hand-hex"));
        let w = self.p("seeds/.work");
        fs::create_dir_all(&w).map_err(|e| e.to_string())?;
        fs::create_dir_all(self.p("seeds/L0/m0")).map_err(|e| e.to_string())?;
        let rd = |p: PathBuf| fs::read(&p).map_err(|e| format!("{}: {e}", p.display()));
        // hex0: packed here, must equal the committed seed and must rebuild itself
        let h0 = pack_hex0(&rd(self.s("tower/L0/hex0/hex0.hand.hex"))?);
        if h0 != rd(self.s("tower/L0/hex0/hex0.bin"))? { return Err("hex0: the packed hex0.hand.hex != tower/L0/hex0/hex0.bin".into()); }
        let hex0 = w.join("hex0");
        fs::write(&hex0, &h0).map_err(|e| e.to_string())?;
        set_exec(&hex0);
        let self0 = w.join("hex0.self");
        self.step(&self0, vec![F(self.s("tower/L0/hex0/hex0.hand.hex"))], &[x(&hex0)], 200, false, "hex0 self-host")?;
        if rd(self0)? != h0 { return Err("hex0 does not rebuild itself from hex0.hand.hex".into()); }
        // hex1: packed here (labels resolved), must equal the committed seed and must rebuild itself
        let h1 = pack_hex1(&rd(self.s("tower/L0/hex1/derived-hex1.labeled.hex"))?);
        if h1 != rd(self.s("tower/L0/hex1/hex1.bin"))? { return Err("hex1: the packed derived-hex1.labeled.hex != tower/L0/hex1/hex1.bin".into()); }
        let hex1 = w.join("hex1");
        fs::write(&hex1, &h1).map_err(|e| e.to_string())?;
        set_exec(&hex1);
        let self1 = w.join("hex1.self");
        self.step(&self1, vec![F(self.s("tower/L0/hex1/derived-hex1.labeled.hex"))], &[x(&hex1)], 300, false, "hex1 self-host")?;
        if rd(self1)? != h1 { return Err("hex1 does not rebuild itself from derived-hex1.labeled.hex".into()); }
        ui::note(format!("hex0 ({} B) and hex1 ({} B) packed by this program == the committed seeds, and each self-hosts", h0.len(), h1.len()));
        let hex2 = w.join("hex2");
        self.step(&hex2, vec![F(self.s("tower/L0/hex2/derived-hex2.labeled.hex"))], &[x(&hex1)], 300, false, "hex2")?;
        let m0 = self.p("seeds/L0/m0/m0.bin");
        self.step(&m0, vec![F(self.s("tower/L0/m0/m0.labeled.hex"))], &[x(&hex2)], 500, false, "m0")?;
        let cfs = self.p("seeds/cf.bin");
        self.step(&cfs, vec![F(self.s("tower/L0/emitter/rzir-to-elf-cf.m0"))], &[x(&m0)], 10000, false, "cf (M0 backend)")?;
        let m0cc = w.join("m0cc");
        self.step(&m0cc, vec![F(self.s("tower/L1/c-frontend/c-to-rzir.m0"))], &[x(&m0)], 10000, false, "M0 cc")?;
        for st in ["link/rzir-ld", "opt/rzir-opt", "ra/rzir-ra"] {
            let n = st.split('/').nth(1).unwrap();
            self.step(&w.join(format!("{n}.0")), vec![F(self.s(&format!("tower/L2/{st}.c")))], &[x(&m0cc), x(&cfs)], 10000, false, n)?;
        }
        let chain0 = |w: &Path| vec![x(&w.join("rzir-ld.0")), x(&w.join("rzir-opt.0")), x(&w.join("rzir-ra.0")), x(&cfs)];
        let ccc0 = w.join("ccc.0");
        self.step(&ccc0, vec![F(self.s("tower/L2/ccc.c"))], &B::with(x(&m0cc), chain0(&w)), 100000, false, "ccc gen1")?;
        fs::create_dir_all(self.p("seeds/L2/link")).ok();
        fs::create_dir_all(self.p("seeds/L2/opt")).ok();
        fs::create_dir_all(self.p("seeds/L2/ra")).ok();
        let ccc1 = self.p("seeds/L2/ccc.bin");
        self.step(&ccc1, vec![F(self.s("tower/L2/ccc.c"))], &B::with(x(&ccc0), chain0(&w)), 100000, false, "ccc gen2 (self-host)")?;
        for st in ["link/rzir-ld", "opt/rzir-opt", "ra/rzir-ra"] {
            self.step(&self.l2(st), vec![F(self.s(&format!("tower/L2/{st}.c")))], &B::with(x(&ccc0), chain0(&w)), 4000, false, st)?;
        }
        Ok(())
    }

    // ---- 2. the roots, the Rust backend twins, L5, L6 (scratch.sh, build-live-backend, selfhost, stage) -----
    fn tower(&mut self) -> R<()> {
        ui::phase(2, &format!("tower: rzrs seed -> Rust backend twins -> L5 -> cf.hoist -> L6"));
        let cf = self.p("B.cf.m0");
        self.step(&cf, vec![F(self.s("tower/L0/emitter/rzir-to-elf-cf.m0"))], &[x(&self.p("seeds/L0/m0/m0.bin"))], 10000, false, "cf.bin")?;
        let ccc = self.l2("ccc");
        let rzcpp = self.p("L3.rzcpp");
        self.step(&rzcpp, vec![F(self.s("tower/tcc/rzcpp.c"))], &B::with(x(&ccc), self.chain_c(&cf)), 10000, false, "L3.rzcpp")?;
        let inc = format!("-I{}", self.s("tower/backend-toolchain/rzrs-seed/headers").display());
        let pp = self.p("L3.rzrs.pp.c");
        pipe(vec![F(self.s("tower/L3/rzrs.c"))], &[xa(&rzcpp, &[&inc])], &pp).map_err(|e| e.to_string())?;
        let rsz = self.p("L3.rzrs");
        self.step(&rsz, vec![F(self.s("tower/backend-toolchain/rzrs-seed/shim.c")), F(pp)], &B::with(x(&ccc), self.chain_c(&cf)), 100000, false, "rzrs.rsz (C-seed Rust frontend)")?;
        // build-live-backend: frontend images exceed M0 cf's input ceiling, so they lower through the cf.hoist seed
        let seed = self.s("tower/L5/cf.hoist.seed");
        let (l4, l5) = (self.s("tower/L4/L4.rzrs.rs"), self.s("tower/L5/L5.rzrs.rs"));
        self.step(&self.p("L4.rzrs"), vec![F(l4.clone())], &B::with(x(&rsz), self.chain_c(&seed)), 10000, false, "L4 (built by L3)")?;
        self.step(&self.p("L5.rzrs"), vec![F(l5.clone())], &B::with(x(&self.p("L4.rzrs")), self.chain_c(&seed)), 100000, false, "bootstrap frontend")?;
        for st in ["ld", "opt", "ra"] {
            let src = self.s(&format!("tower/backend-toolchain/rzir-{st}.rs"));
            self.step(&self.p(&format!("B.{st}")), vec![F(src)], &B::with(x(&self.p("L5.rzrs")), self.chain_c(&seed)), 10000, false, st)?;
        }
        // selfhost.sh: L4 lowered by M0 cf -> L5 lowered by the cf.hoist seed -> L5 mints its own cf.hoist from cf.rs
        let t0 = self.p("L4.rzrs.m0");
        self.step(&t0, vec![F(l4.clone())], &B::with(x(&rsz), self.chain_rs(&cf)), 10000, false, "L4 via M0 cf")?;
        let rr = self.p("L5.rzrs.g1.rzir");
        self.step(&rr, vec![F(l5.clone())], &[x(&t0)], 1000, true, "L4 compiles L5")?;
        let rself = self.p("L5.rzrs.g1");
        self.step(&rself, vec![F(rr.clone())], &self.chain_rs(&seed), 100000, false, "L5 (g1)")?;
        let cfh5 = self.p("B.cf.hoist.g1");
        self.step(&cfh5, vec![F(self.s("tower/backend-cf/cf.rs"))], &B::with(x(&rself), self.chain_rs(&cf)), 20000, false, "cf.hoist (minted by L5)")?;
        // check-cfhoist-seed.sh: re-lowering L5 with the source-minted cf.hoist gives the same compiler
        let wash = self.p("L5.rzrs.washed");
        self.step(&wash, vec![F(rr)], &self.chain_rs(&cfh5), 100000, false, "L5 via source cf.hoist")?;
        if fs::read(&wash).ok() != fs::read(&rself).ok() { return Err("cf.hoist.seed leaks into the produced compiler (L5.rzrs.g1 differs)".into()); }
        ui::note(format!("ok: the committed cf.hoist.seed washes out of L5"));
        // stage.sh: lowering now goes through the run's own cf.hoist
        let t0s = self.p("L4.rzrs.g2");
        self.step(&t0s, vec![F(l4)], &B::with(x(&rsz), self.chain_rs(&cfh5)), 10000, false, "L4 (g2)")?;
        let st0 = self.p("L5.rzrs.g2");
        self.step(&st0, vec![F(l5)], &B::with(x(&t0s), self.chain_rs(&cfh5)), 100000, false, "L5 (g2)")?;
        let cfh = self.p("B.cf.hoist");
        self.step(&cfh, vec![F(self.s("tower/backend-cf/cf.rs"))], &B::with(x(&st0), self.chain_rs(&cfh5)), 20000, false, "B.cf.hoist")?;
        let next = self.p("L6.rzrs.rzir");
        self.step(&next, vec![F(self.s("tower/L6/L6.rzrs.rs"))], &[x(&st0)], 1000, true, "L5 compiles L6")?;
        self.step(&self.p("L6.rzrs"), vec![F(next)], &self.chain_rs(&cfh), 100000, false, "L6")?;
        Ok(())
    }

    // ---- 3. the SEM0 staircase (sem0-stage.sh) ---------------------------------------------------------------
    /// cur compiles src (with the //@S9 header) to `<name>.rzir`, lowered to `<name>`.
    fn hop(&mut self, cur: &Path, src: &Path, name: &str, c3: bool) -> R<PathBuf> {
        let rz = self.p(&format!("{name}.rzir"));
        let out = self.p(name);
        if self.reusable(&out) { self.seal(&out, 1000000, name)?; return Ok(out); }
        self.emit(cur, src, &rz)?;
        let lower = if c3 { self.chain3() } else { self.chain_c(&self.p("B.cf.hoist")) };
        self.step(&out, vec![F(rz)], &lower, 1000000, false, name)?;
        Ok(out)
    }
    fn emit(&mut self, cur: &Path, src: &Path, rz: &Path) -> R<()> {
        ui::running(format!("{} compiles {}", base(cur), base(src)));
        let ok = pipe(vec![T("//@S9\n".into()), F(src.to_path_buf())], &[x(cur)], rz).map_err(|e| e.to_string())?;
        let lines = fs::read(rz).map(|b| b.iter().filter(|&&c| c == b'\n').count()).unwrap_or(0);
        if !ok || lines < 100000 { return Err(format!("{} could not compile {} ({lines} rzir lines)", cur.display(), src.display())); }
        Ok(())
    }
    /// sem0-stage.sh's re-freeze: compile src repeatedly until the emit is a byte fixpoint. At the fixpoint the
    /// emit equals the one the current binary was lowered from, so NAME is that binary (copied, not re-lowered)
    /// and NAME.rzir its emit -- what NAME produces from src, which llvm() reuses for SEM0.cf.
    fn fixpoint(&mut self, mut cur: PathBuf, src: &Path, name: &str) -> R<PathBuf> {
        let mut prev: Option<Vec<u8>> = None;
        for it in 1..=5 {
            let rz = self.p(&format!("{name}.i{it}.rzir"));
            self.emit(&cur, src, &rz)?;
            let now = fs::read(&rz).map_err(|e| e.to_string())?;
            if prev.as_ref() == Some(&now) {
                let f = self.p(name);
                fs::copy(&cur, &f).map_err(|e| e.to_string())?;
                fs::rename(&rz, self.p(&format!("{name}.rzir"))).map_err(|e| e.to_string())?;
                self.seal(&f, 1000000, name)?;
                ui::note(format!("{name}: byte fixpoint at iteration {it}"));
                return Ok(f);
            }
            let b = self.p(&format!("{name}.i{it}"));
            self.step(&b, vec![F(rz)], &self.chain3(), 1000000, false, &format!("{name}.i{it}"))?;
            cur = b;
            prev = Some(now);
        }
        Err(format!("{name}: no byte fixpoint in 5 iterations"))
    }
    /// Hops are numbered in build order: SEM0.h00 is the seed, SEM0.hNN is built by SEM0.h(NN-1) from the
    /// NNth source, and the last hop (the dev source, to a fixpoint) is SEM0.cf.
    fn sem0(&mut self) -> R<()> {
        ui::phase(3, &format!("SEM0 staircase (each hop compiles the next, {HOPS} hops)"));
        let rz = self.p("SEM0.h00.rzir");
        if !(self.resume && rz.exists()) {
            let ok = pipe(vec![F(self.s(&format!("{SEM0D}/h00.rs")))], &[x(&self.p("L6.rzrs"))], &rz).map_err(|e| e.to_string())?;
            if !ok { return Err("L6 could not compile hops/h00.rs".into()); }
        }
        let mut cur = self.p("SEM0.h00");
        self.step(&cur, vec![F(rz)], &self.chain_c(&self.p("B.cf.hoist")), 1000000, false, "SEM0.h00")?;
        // h01 and h02 lower through the C ld, later hops through the i64 Rust ld twin (chain3)
        for n in 1..HOPS {
            let (src, name) = (self.s(&format!("{SEM0D}/h{n:02}.rs")), format!("SEM0.h{n:02}"));
            cur = if n == 23 { self.fixpoint(cur, &src, &name)? } else { self.hop(&cur, &src, &name, n >= 3)? };
        }
        self.fixpoint(cur, &self.s(DEV), "SEM0.cf").map(|_| ())
    }

    // ---- 4. the LLVM-minted SEM0 and its C2 admission (sem0-stage-llvm.sh) -----------------------------------
    fn llvm(&mut self) -> R<()> {
        ui::phase(4, &format!("SEM0.llvm (C2: its emit must equal the hex-rooted frozen's)"));
        let fz = self.p("SEM0.cf");
        let r2lrz = self.p("r2l.rzir");
        pipe(vec![T("//@S9\n".into()), F(self.s("tower/backend-llvm/rzir2ll.rs"))], &[x(&fz)], &r2lrz).map_err(|e| e.to_string())?;
        if fs::read(&r2lrz).map(|b| b.windows(9).any(|w| w == b"UNLOWERED")).unwrap_or(true) { return Err("rzir2ll UNLOWERED".into()); }
        let back = |b: &B| vec![x(&b.p("B.ld")), x(&b.l2("opt/rzir-opt")), x(&b.l2("ra/rzir-ra")), x(&b.p("B.cf.hoist"))];
        let r2l = self.p("r2l.cf");
        self.step(&r2l, vec![F(r2lrz.clone())], &back(self), 10000, false, "rzir2ll (cf)")?;
        // the .ll name lands in the ELF symbol table, so it keeps the shell build's spelling
        let mll = self.p("r2l.mmap.ll");
        pipe(vec![T("gdef __mmapheap 1\n".into()), F(r2lrz)], &[x(&r2l)], &mll).map_err(|e| e.to_string())?;
        let mo = self.p("r2l.mmap.o");
        let mel = self.p("r2l.llvm.mmap.elf");
        tool("llc", &[&mll, &mo], &["-O0", "-filetype=obj", "{}", "-o", "{}"])?;
        tool("cc", &[&mo, &mel], &["-no-pie", "{}", "-o", "{}"])?;
        self.seal(&mel, 10000, "rzir2ll (llvm, mmap heap)")?;
        let srz = self.p("SEM0.cf.rzir");   // SEM0.cf compiling the dev source: the fixpoint already emitted it
        let sll = self.p("self.ll");
        if !pipe(vec![F(srz.clone())], &[x(&mel)], &sll).map_err(|e| e.to_string())? { return Err("bridge translate of SEM0 failed".into()); }
        let so = self.p("self.o");
        let lf = self.p("SEM0.llvm");
        tool("llc", &[&sll, &so], &["-O0", "-filetype=obj", "{}", "-o", "{}"])?;
        tool("cc", &[&so, &lf], &["-no-pie", "{}", "-o", "{}"])?;
        self.seal(&lf, 1000000, "SEM0.llvm")?;
        let via = self.p("self.viallvm.rz");
        pipe(vec![T("//@S9\n".into()), F(self.s(DEV))], &[x(&lf)], &via).map_err(|e| e.to_string())?;
        if fs::read(&via).ok() != fs::read(&srz).ok() { return Err("C2: the LLVM frozen's emit != the hex-rooted frozen's".into()); }
        ui::note(format!("C2 ok: SEM0.llvm emits SEM0 byte-identically to the hex-rooted frozen"));
        Ok(())
    }

    // ---- 5. r2l and the SEM0-built ourc (semc.sh, build-ourc.sh) ---------------------------------------------
    fn semcbk(&mut self, r2l: &Path, rz: &Path, out: &Path, w: &Path, min: u64, what: &str) -> R<()> {
        let ll = w.join("x.ll");
        pipe(vec![F(rz.to_path_buf()), F(self.s(RT))], &[x(r2l)], &ll).map_err(|e| e.to_string())?;
        let (bc, o) = (w.join("x.bc"), w.join("x.o"));
        tool("opt", &[&ll, &bc], &["-passes=mem2reg", "{}", "-o", "{}"])?;
        tool("llc", &[&bc, &o], &["-O2", "-filetype=obj", "{}", "-o", "{}"])?;
        tool("cc", &[&o, out], &["-no-pie", "{}", "-o", "{}"])?;
        self.seal(out, min, what)
    }
    fn ourc(&mut self) -> R<PathBuf> {
        ui::phase(5, &format!("r2l.llvm and the SEM0-built ourc"));
        let lf = self.p("SEM0.llvm");
        let w = self.out.join("work");
        fs::create_dir_all(&w).map_err(|e| e.to_string())?;
        let r2lrz = w.join("rzir2ll.rzir");
        ui::running("SEM0.llvm compiles rzir2ll.rs".into());
        pipe(vec![T("//@S9\n".into()), F(self.s("tower/backend-llvm/rzir2ll.rs"))], &[x(&lf)], &r2lrz).map_err(|e| e.to_string())?;
        let ll = w.join("rzir2ll.ll");
        ui::running("the bridge translates rzir2ll".into());
        pipe(vec![F(r2lrz)], &[x(&self.p("r2l.llvm.mmap.elf"))], &ll).map_err(|e| e.to_string())?;
        let o = w.join("rzir2ll.o");
        let r2l = self.out.join("r2l.llvm");
        tool("llc", &[&ll, &o], &["-O2", "-filetype=obj", "{}", "-o", "{}"])?;
        tool("cc", &[&o, &r2l], &["-no-pie", "{}", "-o", "{}"])?;
        self.seal(&r2l, 10000, "r2l.llvm")?;
        // the cross-crate linker, then each crate's metadata, then each crate, then link + lower
        let lrz = w.join("link.rz");
        ui::running("SEM0.llvm compiles link.rs".into());
        pipe(vec![T("//@S9\n".into()), F(self.s(LINK_RS))], &[x(&lf)], &lrz).map_err(|e| e.to_string())?;
        let lsemc = w.join("link.semc");
        self.semcbk(&r2l, &lrz, &lsemc, &w, 10000, "link.semc")?;
        let mh = w.join("mh.txt");
        fs::write(&mh, "").map_err(|e| e.to_string())?;
        for c in CRATES {
            let m = w.join(format!("{c}.meta"));
            ui::running(format!("SEM0.llvm reads the metadata of ourc/src/{c}.rs"));
            pipe(vec![T(format!("//@METADATA crate={c}\n")), F(mh.clone()), F(self.s(&format!("ourc/src/{c}.rs")))], &[x(&lf)], &m).map_err(|e| e.to_string())?;
            let mut acc = fs::read(&mh).map_err(|e| e.to_string())?;
            for line in fs::read(&m).map_err(|e| e.to_string())?.split(|&b| b == b'\n') {
                if line.starts_with(b"META ") { acc.extend_from_slice(b"//"); acc.extend_from_slice(line); acc.push(b'\n'); }
            }
            fs::write(&mh, acc).map_err(|e| e.to_string())?;
        }
        let mut linked_in = Vec::new();
        for c in std::iter::once("odrv").chain(CRATES) {
            let rz = w.join(format!("{c}.rz"));
            ui::running(format!("SEM0.llvm compiles ourc/src/{c}.rs"));
            let ok = pipe(vec![T(format!("//@SUPERSET crate={c}\n")), F(mh.clone()), F(self.s(&format!("ourc/src/{c}.rs")))], &[x(&lf)], &rz).map_err(|e| e.to_string())?;
            if !ok { return Err(format!("SEM0 rejected crate {c}")); }
            if c != "odrv" { linked_in.push(T("@@@CRATE@@@\n".into())); }
            linked_in.push(F(rz));
        }
        let linked = w.join("linked.rz");
        ui::running("link.semc links the 19 crates".into());
        pipe(linked_in, &[x(&lsemc)], &linked).map_err(|e| e.to_string())?;
        let st1 = self.out.join("ourc.g0");
        self.semcbk(&r2l, &linked, &st1, &w, 500000, "ourc.g0 (built by SEM0)")?;
        Ok(st1)
    }

    // ---- 6. ourc compiles ourc: ourc.g1, ourc.g2, the fixed point (release.sh step 5) -------------------------
    fn selfc(&mut self, st1: &Path) -> R<PathBuf> {
        ui::phase(6, &format!("ourc compiles itself (the fixed point)"));
        let w = self.out.join("self");
        fs::create_dir_all(&w).map_err(|e| e.to_string())?;
        for c in CRATES {
            let mut s = String::new();
            for d in CRATES { if d != c { s.push_str(&format!("use crate::{d};\nuse crate::{d}::*;\n")); } }
            s.push_str(&fs::read_to_string(self.s(&format!("ourc/src/{c}.rs"))).map_err(|e| e.to_string())?);
            fs::write(w.join(format!("{c}.rs")), s).map_err(|e| e.to_string())?;
        }
        let mut m = String::new();
        for c in CRATES { m.push_str(&format!("mod {c};\n")); }
        for c in CRATES { m.push_str(&format!("use {c}::*;\n")); }
        m.push_str(&fs::read_to_string(self.s("ourc/src/odrv.rs")).map_err(|e| e.to_string())?);
        let main = w.join("main.rs");
        fs::write(&main, m).map_err(|e| e.to_string())?;
        let ms = main.to_str().unwrap();
        let irs = [w.join("ir1.ll"), w.join("ir2.ll"), w.join("ir3.ll")];
        let bins = [w.join("ourc.g1"), w.join("ourc.g2")];
        ui::running("ourc.g0 compiles ourc".into());
        if !pipe(vec![], &[xa(st1, &[ms, "--emit-ir"])], &irs[0]).map_err(|e| e.to_string())? { return Err("the SEM0-built ourc rejected ourc".into()); }
        self.link_ir(&irs[0], &bins[0], "ourc.g1 (release binary)")?;
        ui::running("ourc.g1 compiles ourc".into());
        if !pipe(vec![], &[xa(&bins[0], &["--mem-limit", "0", "--time-limit", "0", ms, "--emit-ir"])], &irs[1]).map_err(|e| e.to_string())? { return Err("ourc.g1 rejected ourc".into()); }
        if fs::read(&irs[0]).ok() != fs::read(&irs[1]).ok() { return Err("IR1 != IR2: ourc.g1 compiles ourc differently from ourc.g0".into()); }
        self.link_ir(&irs[1], &bins[1], "ourc.g2")?;
        ui::running("ourc.g2 compiles ourc".into());
        if !pipe(vec![], &[xa(&bins[1], &["--mem-limit", "0", "--time-limit", "0", ms, "--emit-ir"])], &irs[2]).map_err(|e| e.to_string())? { return Err("ourc.g2 rejected ourc".into()); }
        if fs::read(&irs[1]).ok() != fs::read(&irs[2]).ok() { return Err("not a fixed point (IR2 != IR3)".into()); }
        if fs::read(&bins[0]).ok() != fs::read(&bins[1]).ok() { return Err("ourc.g1 and ourc.g2 differ as executables".into()); }
        ui::note(format!("fixed point: IR1 == IR2 == IR3, ourc.g1 == ourc.g2"));
        Ok(bins[0].clone())
    }
    fn link_ir(&mut self, ll: &Path, out: &Path, what: &str) -> R<()> {
        let o = ll.with_extension("o");
        tool("llc", &[ll, &o], &["-O2", "-filetype=obj", "{}", "-o", "{}"])?;
        tool("cc", &[&o, out], &["-no-pie", "{}", "-o", "{}", "-lm"])?;
        self.seal(out, 100000, what)
    }

    fn package(&mut self, ourc: &Path) -> R<()> {
        let p = self.out.join("ourc-v0.1-x86_64-linux");
        fs::create_dir_all(&p).map_err(|e| e.to_string())?;
        let dst = p.join("ourc");
        fs::copy(ourc, &dst).map_err(|e| e.to_string())?;
        for f in ["ourc/README.md", "ourc/LIMITATIONS.md", "LICENSE-MIT", "LICENSE-APACHE"] {
            fs::copy(self.s(f), p.join(Path::new(f).file_name().unwrap())).map_err(|e| e.to_string())?;
        }
        let k = self.out.join("ourc");
        fs::copy(&dst, &k).map_err(|e| e.to_string())?;
        self.seal(&k, 100000, "ourc (release binary)")?;
        let h = sha256::hex(&fs::read(&dst).map_err(|e| e.to_string())?);
        fs::write(self.out.join("SHA256SUMS"), format!("{h}  ourc-v0.1-x86_64-linux/ourc\n")).map_err(|e| e.to_string())?;
        let mut prov = String::from("# ourc-bootstrap: every artifact of the from-hex build, in build order\n");
        for l in &self.log { prov.push_str(l); prov.push('\n'); }
        fs::write(self.out.join("PROVENANCE"), prov).map_err(|e| e.to_string())
    }
}

fn read_tree_manifest(src: &Path) -> Option<HashMap<String, String>> {
    let m = fs::read_to_string(src.join("MANIFEST")).ok()?;
    Some(m.lines().filter_map(|l| l.split_once("  ")).map(|(h, p)| (p.to_string(), h.to_string())).collect())
}

fn vendor(repo: &Path, dir: &Path) -> R<()> {
    // replace an earlier vendored tree whole, so a renamed source leaves nothing stale to embed
    if dir.join("MANIFEST").exists() { fs::remove_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?; }
    else if fs::read_dir(dir).map(|mut d| d.next().is_some()).unwrap_or(false) {
        return Err(format!("{} is not empty and holds no MANIFEST; refusing to vendor into it", dir.display()));
    }
    let mut man = String::new();
    let mut total = 0;
    let chain = delta_chain();
    for rel in sources() {
        let b = fs::read(repo.join(&rel)).map_err(|e| format!("{rel}: {e}"))?;
        let d = dir.join(&rel);
        fs::create_dir_all(d.parent().unwrap()).map_err(|e| e.to_string())?;
        match chain.iter().position(|c| *c == rel) {
            Some(k) if k > 0 => {
                // the delta is made by the host's diff and accepted only if apply_diff reproduces the file exactly
                let base = repo.join(&chain[k - 1]);
                let o = Command::new("diff").arg(&base).arg(repo.join(&rel)).output().map_err(|e| format!("diff: {e}"))?;
                let old = fs::read(&base).map_err(|e| e.to_string())?;
                if apply_diff(&old, &o.stdout)? != b { return Err(format!("{rel}: the delta does not reproduce the file")); }
                fs::write(PathBuf::from(format!("{}.diff", d.display())), &o.stdout).map_err(|e| e.to_string())?;
            }
            _ => fs::write(&d, &b).map_err(|e| e.to_string())?,
        }
        man.push_str(&format!("{}  {rel}\n", sha256::hex(&b)));
        total += b.len();
    }
    fs::write(dir.join("MANIFEST"), &man).map_err(|e| e.to_string())?;
    println!("vendored {} files ({total} bytes) from {} into {}", sources().len(), repo.display(), dir.display());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut repo = None;
    let mut resume = false;
    let mut tui = true;
    let mut rest = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => { i += 1; repo = args.get(i).cloned(); }
            "--resume" => resume = true,
            "--plain" => tui = false,
            "--vendor" => {
                let (Some(r), Some(d)) = (args.get(i + 1), args.get(i + 2)) else { eprintln!("usage: --vendor CHECKOUT DIR"); std::process::exit(2) };
                if let Err(e) = vendor(Path::new(r), Path::new(d)) { eprintln!("ourc-bootstrap: {e}"); std::process::exit(1) }
                return;
            }
            "-h" | "--help" => { println!("usage: ourc-bootstrap [--repo CHECKOUT] [--resume] [--plain] OUTDIR | --vendor CHECKOUT DIR\n  on a terminal it shows a dashboard; --plain (or a pipe) prints one line per step"); return; }
            a => rest.push(a.to_string()),
        }
        i += 1;
    }
    let Some(outd) = rest.first() else { eprintln!("usage: ourc-bootstrap [--repo CHECKOUT] [--resume] [--plain] OUTDIR"); std::process::exit(2) };
    if let Err(e) = run(repo, resume, tui, Path::new(outd)) {
        if ui::active() { ui::finish(Some(&e), &[]) } else { eprintln!("ourc-bootstrap: FAIL -- {e}") }
        std::process::exit(1);
    }
}

fn run(repo: Option<String>, resume: bool, tui: bool, outd: &Path) -> R<()> {
    fs::create_dir_all(outd).map_err(|e| e.to_string())?;
    let out = fs::canonicalize(outd).map_err(|e| e.to_string())?;
    for t in ["llc", "opt", "cc"] {
        if Command::new(t).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() { return Err(format!("{t} not found on PATH")); }
    }
    let from_repo = repo.is_some();
    let src = match repo {
        Some(r) => fs::canonicalize(&r).map_err(|e| format!("{r}: {e}"))?,
        None => {
            if TREE.is_empty() { return Err("this build has no embedded source tree; pass --repo <ourc checkout>".into()); }
            let d = out.join("src");
            for (rel, b) in TREE {
                if rel.ends_with(".diff") { continue; }
                let p = d.join(rel);
                fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
                fs::write(&p, b).map_err(|e| e.to_string())?;
            }
            let chain = delta_chain();
            for k in 1..chain.len() {
                let key = format!("{}.diff", chain[k]);
                let Some((_, diff)) = TREE.iter().find(|(p, _)| *p == key) else { return Err(format!("embedded tree lacks {key}")) };
                let old = fs::read(d.join(&chain[k - 1])).map_err(|e| e.to_string())?;
                let p = d.join(&chain[k]);
                fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
                fs::write(&p, apply_diff(&old, diff)?).map_err(|e| e.to_string())?;
            }
            // the one source file the build executes (the capacity seed L5 lowers itself with)
            set_exec(&d.join("tower/L5/cf.hoist.seed"));
            d
        }
    };
    // the pins hold for one source tree; say so when the sources differ from the vendored manifest
    let embedded: HashMap<String, String> = TREE.iter().find(|(p, _)| *p == "MANIFEST")
        .map(|(_, b)| String::from_utf8_lossy(b).lines().filter_map(|l| l.split_once("  ")).map(|(h, p)| (p.to_string(), h.to_string())).collect())
        .or_else(|| read_tree_manifest(&src)).unwrap_or_default();
    let mut drift = Vec::new();
    for rel in sources() {
        let b = fs::read(src.join(&rel)).map_err(|e| format!("source {rel}: {e}"))?;
        if let Some(h) = embedded.get(&rel) { if *h != sha256::hex(&b) { drift.push(rel); } }
    }
    if !from_repo && !drift.is_empty() { return Err(format!("the embedded sources do not match their MANIFEST: {}", drift.join(", "))); }
    let pins = PINS.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).filter_map(|l| {
        let mut f = l.split_whitespace();
        Some((f.nth(2)?.to_string(), (l[..64].to_string(), l[65..].starts_with("chain"))))
    }).collect();
    unlimit_stack();
    let mut b = B { src, out: out.clone(), sp: out.join("sp"), pins, resume, log: Vec::new(), host_drift: 0, t0: Instant::now() };
    ui::init(tui, format!("sources {} -> {}", b.src.display(), out.display()), b.pins.len());
    if !drift.is_empty() { ui::note(format!("note: {} source file(s) differ from the vendored manifest, so their pins may not hold: {}", drift.len(), drift.join(", "))); }
    b.seeds()?;
    b.tower()?;
    b.sem0()?;
    b.llvm()?;
    let st1 = b.ourc()?;
    let o = b.selfc(&st1)?;
    b.package(&o)?;
    let mut summary = vec![format!("ourc-bootstrap: OK in {:.0}s -- {}", b.t0.elapsed().as_secs_f64(), out.join("ourc-v0.1-x86_64-linux/ourc").display())];
    if b.host_drift > 0 { summary.push(format!("  {} host-built artifact(s) differ from the pins: this host's llc/opt/cc are not the reference ones (the tower itself matched)", b.host_drift)); }
    summary.extend(fs::read_to_string(out.join("SHA256SUMS")).unwrap_or_default().lines().map(String::from));
    ui::finish(None, &summary);
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// The SEM0 staircase sources are successive versions of one file (37 MB in full), so the vendored tree keeps the
// first in full and each later one as a normal-format `diff` against its predecessor; MANIFEST pins the results.

/// The staircase in order: each file is stored as a delta against the one before it.
fn delta_chain() -> Vec<String> {
    let mut v: Vec<String> = (0..HOPS).map(|n| format!("{SEM0D}/h{n:02}.rs")).collect();
    v.push(DEV.to_string());
    v
}

/// Split into lines, each keeping its `\n` (the last may lack one).
fn lines(b: &[u8]) -> Vec<&[u8]> { b.split_inclusive(|&c| c == b'\n').collect() }

/// Apply a normal-format diff (`5a6,8`, `3,4c3`, `7,9d6`, `< `/`> ` lines, `\ No newline`) to `old`.
fn apply_diff(old: &[u8], diff: &[u8]) -> R<Vec<u8>> {
    let ol = lines(old);
    let dl = lines(diff);
    let mut out: Vec<u8> = Vec::new();
    let mut at = 0usize; // old lines copied so far
    let range = |s: &str| -> R<(usize, usize)> {
        let mut it = s.split(',');
        let a: usize = it.next().unwrap().parse().map_err(|_| format!("bad diff range {s}"))?;
        let b: usize = it.next().map(|x| x.parse().map_err(|_| format!("bad diff range {s}"))).transpose()?.unwrap_or(a);
        Ok((a, b))
    };
    let mut i = 0;
    while i < dl.len() {
        let h = std::str::from_utf8(dl[i]).map_err(|_| "non-utf8 diff header")?.trim_end();
        i += 1;
        let op = h.find(|c| c == 'a' || c == 'c' || c == 'd').ok_or(format!("bad diff header {h}"))?;
        let (a0, a1) = range(&h[..op])?;
        let kind = h.as_bytes()[op];
        // old lines before the hunk are kept; for `a` the hunk follows old line a0, else it replaces a0..=a1
        let keep_to = if kind == b'a' { a0 } else { a0 - 1 };
        for l in &ol[at..keep_to] { out.extend_from_slice(l); }
        at = if kind == b'a' { a0 } else { a1 };
        while i < dl.len() && (dl[i].starts_with(b"< ") || dl[i].starts_with(b"---") || dl[i].starts_with(b"\\")) { i += 1; }
        while i < dl.len() && dl[i].starts_with(b"> ") {
            out.extend_from_slice(&dl[i][2..]);
            i += 1;
            if i < dl.len() && dl[i].starts_with(b"\\") { if out.last() == Some(&b'\n') { out.pop(); } i += 1; }
        }
    }
    for l in &ol[at..] { out.extend_from_slice(l); }
    Ok(out)
}
