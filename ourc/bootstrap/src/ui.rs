// ui.rs -- what the build reports, and how. Plain mode prints one line per event (the format the build always
// had; pipes and CI get it). On a terminal the same events drive a dashboard, redrawn by a ticker thread so
// elapsed times keep moving through the multi-minute steps. Display only: the one key, q, asks to stop after the
// running step, which --resume can pick up. No dependencies -- ANSI escapes, and termios through libc.

use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Phase titles, and their share of a reference run (seconds, from a 1049 s from-hex build) for the ETA.
const PHASES: [(&str, u64); 6] = [
    ("seeds from the hand-hex", 1),
    ("tower: rzrs seed -> Rust backend twins -> L5 -> cf.hoist -> L6", 3),
    ("SEM0 staircase (each hop compiles the next)", 320),
    ("SEM0.llvm, admitted by C2", 300),
    ("r2l.llvm and the SEM0-built ourc", 380),
    ("ourc compiles itself (the fixed point)", 45),
];

#[derive(Default)]
struct State {
    title: String,
    phase: usize,                  // 1-based; 0 = not started
    phase_t: Vec<f64>,             // seconds each finished phase took
    phase_start: f64,
    now: String,                   // what is running
    now_start: f64,
    seals: Vec<(String, String)>,  // (rendered line, verdict class: ok / drift / new / bad)
    notes: Vec<String>,
    pins_ok: usize,
    drift: usize,
    unpinned: usize,
    pins_total: usize,
    confirm: bool,                 // q pressed, waiting for y
    failed: Option<String>,
}

struct Ui {
    tty: bool,
    t0: Instant,
    st: Mutex<State>,
    stop: AtomicBool,
    closed: AtomicBool,            // the dashboard is gone; the ticker must not draw again
}

static UI: OnceLock<Ui> = OnceLock::new();

extern "C" {
    fn isatty(fd: i32) -> i32;
    fn tcgetattr(fd: i32, t: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, act: i32, t: *const Termios) -> i32;
    fn ioctl(fd: i32, req: u64, ...) -> i32;
    fn signal(sig: i32, h: extern "C" fn(i32)) -> usize;
    fn write(fd: i32, b: *const std::ffi::c_void, n: usize) -> isize;
    fn _exit(code: i32) -> !;
}

/// struct termios on x86-64 Linux (glibc): 4 flag words, c_line, 32 control chars, 2 speeds.
#[repr(C)]
#[derive(Clone, Copy)]
struct Termios { iflag: u32, oflag: u32, cflag: u32, lflag: u32, line: u8, cc: [u8; 32], ispeed: u32, ospeed: u32 }
const ICANON: u32 = 0o2;
const ECHO: u32 = 0o10;
const VTIME: usize = 5;
const VMIN: usize = 6;

static mut SAVED: Option<Termios> = None;
const LEAVE: &[u8] = b"\x1b[?25h\x1b[?1049l";

fn restore_terminal() {
    unsafe {
        let saved = std::ptr::addr_of!(SAVED).read();
        if let Some(t) = saved { tcsetattr(0, 0, &t); }
        write(1, LEAVE.as_ptr().cast(), LEAVE.len());
    }
}
extern "C" fn on_signal(_: i32) {
    restore_terminal();
    let m = b"\nourc-bootstrap: interrupted -- rerun with --resume to continue\n";
    unsafe { write(2, m.as_ptr().cast(), m.len()); _exit(130) }
}

fn ui() -> &'static Ui { UI.get().expect("ui::init first") }
fn secs() -> f64 { ui().t0.elapsed().as_secs_f64() }

/// Start reporting. `tui` asks for the dashboard; it is used only when stdout is a terminal.
pub fn init(tui: bool, title: String, pins_total: usize) {
    let tty = tui && unsafe { isatty(1) == 1 };
    let st = State { title, pins_total, ..State::default() };
    let _ = UI.set(Ui { tty, t0: Instant::now(), st: Mutex::new(st), stop: AtomicBool::new(false), closed: AtomicBool::new(false) });
    if !tty { println!("ourc-bootstrap: {}", u_title()); return; }
    unsafe {
        if isatty(0) == 1 {
            let mut t: Termios = std::mem::zeroed();
            if tcgetattr(0, &mut t) == 0 {
                SAVED = Some(t);
                let mut r = t;
                r.lflag &= !(ICANON | ECHO);
                r.cc[VMIN] = 0;
                r.cc[VTIME] = 2;
                tcsetattr(0, 0, &r);
            }
        }
        signal(2, on_signal);
        signal(15, on_signal);
    }
    print!("\x1b[?1049h\x1b[?25l");
    std::thread::spawn(|| loop { draw(); std::thread::sleep(Duration::from_millis(250)); });
    std::thread::spawn(|| {
        let mut b = [0u8; 1];
        loop {
            match std::io::stdin().read(&mut b) {
                Ok(1) => key(b[0]),
                Ok(_) => std::thread::sleep(Duration::from_millis(100)),
                Err(_) => return,
            }
        }
    });
}

fn key(k: u8) {
    let u = ui();
    let mut s = u.st.lock().unwrap();
    if s.confirm {
        s.confirm = false;
        if k == b'y' || k == b'Y' {
            u.stop.store(true, Ordering::SeqCst);
            s.notes.push("stop requested: the build stops after the running step".into());
        }
    } else if k == b'q' {
        s.confirm = true;
    }
}

/// True once the user asked to stop; the build checks it between steps.
pub fn stop_requested() -> bool { ui().stop.load(Ordering::SeqCst) }

pub fn phase(n: usize, title: &str) {
    let u = ui();
    if !u.tty { println!("== {n}. {title}"); return; }
    let mut s = u.st.lock().unwrap();
    let t = secs();
    if s.phase > 0 { let d = t - s.phase_start; s.phase_t.push(d); }
    s.phase = n;
    s.phase_start = t;
}

/// What is running now (shown live; plain mode is silent, as before).
pub fn running(what: String) {
    let u = ui();
    if !u.tty { return; }
    let mut s = u.st.lock().unwrap();
    s.now = what;
    s.now_start = secs();
}

/// An artifact was sealed. `class` is ok (== pin), drift (host pin differs), new (unpinned).
pub fn seal(line: String, class: &str) {
    let u = ui();
    if !u.tty { println!("{line}"); return; }
    let mut s = u.st.lock().unwrap();
    match class { "ok" => s.pins_ok += 1, "drift" => s.drift += 1, _ => s.unpinned += 1 }
    s.seals.push((line, class.to_string()));
}

/// A check passed, or something worth saying.
pub fn note(text: String) {
    let u = ui();
    if !u.tty { println!("  {text}"); return; }
    u.st.lock().unwrap().notes.push(text);
}

/// The build ended. The dashboard is closed first, so the summary lands in the scrollback.
pub fn finish(err: Option<&str>, summary: &[String]) {
    let u = ui();
    if u.tty {
        {
            let mut s = u.st.lock().unwrap();
            if let Some(e) = err { s.failed = Some(e.to_string()); }
            let t = secs();
            if s.phase > 0 && err.is_none() { let d = t - s.phase_start; s.phase_t.push(d); }
        }
        draw();
        std::thread::sleep(Duration::from_millis(400));
        { let _g = u.st.lock().unwrap(); u.closed.store(true, Ordering::SeqCst); }   // under the lock: no draw is mid-flight
        restore_terminal();
        let s = u.st.lock().unwrap();
        for (i, d) in s.phase_t.iter().enumerate() { println!("== {}. {}  ({:.0}s)", i + 1, PHASES[i].0, d); }
        for n in &s.notes { println!("  {n}"); }
        println!("  pins: {} match, {} host drift, {} unpinned (of {} pinned)", s.pins_ok, s.drift, s.unpinned, s.pins_total);
    }
    for l in summary { println!("{l}"); }
    if let Some(e) = err { eprintln!("ourc-bootstrap: {} -- {e}", if e.starts_with("stopped") { "STOPPED" } else { "FAIL" }); }
}

fn hms(t: f64) -> String {
    let t = t.max(0.0) as u64;
    if t >= 3600 { format!("{}h{:02}m", t / 3600, t / 60 % 60) } else if t >= 60 { format!("{}m{:02}s", t / 60, t % 60) } else { format!("{t}s") }
}

fn width() -> (usize, usize) {
    let mut ws = [0u16; 4];
    let ok = unsafe { ioctl(1, 0x5413, ws.as_mut_ptr()) } == 0;
    if ok && ws[1] > 0 { (ws[1] as usize, ws[0] as usize) } else { (100, 40) }
}

fn clip(s: &str, w: usize) -> String { s.chars().take(w).collect() }

fn draw() {
    let u = ui();
    if u.closed.load(Ordering::SeqCst) { return; }
    let s = u.st.lock().unwrap();
    if u.closed.load(Ordering::SeqCst) { return; }
    let (w, h) = width();
    let t = secs();
    let mut o = String::from("\x1b[H");
    let line = |o: &mut String, text: &str| { o.push_str(&clip(text, w)); o.push_str("\x1b[K\r\n"); };
    line(&mut o, &format!("\x1b[1m ourc-bootstrap\x1b[0m  {}", s.title));
    line(&mut o, &format!(" {}", "─".repeat(w.saturating_sub(2).min(96))));
    // phases, with a progress bar weighted by the reference run
    let total: u64 = PHASES.iter().map(|p| p.1).sum();
    let mut done = 0.0;
    for (i, (name, est)) in PHASES.iter().enumerate() {
        let n = i + 1;
        let (mark, info) = if n < s.phase || (n == s.phase && s.failed.is_none() && s.phase_t.len() >= n) {
            done += *est as f64;
            ("\x1b[32m✓\x1b[0m", hms(*s.phase_t.get(i).unwrap_or(&0.0)))
        } else if n == s.phase {
            let el = t - s.phase_start;
            done += el.min(*est as f64 * 0.98);
            if s.failed.is_some() { ("\x1b[31m✗\x1b[0m", hms(el)) } else { ("\x1b[33m▶\x1b[0m", format!("{} / ~{}", hms(el), hms(*est as f64))) }
        } else { (" ", String::new()) };
        line(&mut o, &format!("  [{mark}] {n} {:<62} {info}", name));
    }
    let frac = (done / total as f64).min(1.0);
    let bw = w.saturating_sub(40).clamp(10, 50);
    let fill = (frac * bw as f64) as usize;
    let eta = if frac > 0.01 && s.failed.is_none() { format!("eta ~{}", hms(t / frac - t)) } else { String::new() };
    line(&mut o, "");
    line(&mut o, &format!("  {}{}  {:>3.0}%   elapsed {}   {eta}", "█".repeat(fill), "░".repeat(bw - fill), frac * 100.0, hms(t)));
    if let Some(e) = &s.failed { line(&mut o, &format!("  \x1b[31mFAIL\x1b[0m {e}")); }
    else if !s.now.is_empty() { line(&mut o, &format!("  now: {}  \x1b[2m({})\x1b[0m", s.now, hms(t - s.now_start))); }
    else { line(&mut o, ""); }
    line(&mut o, "");
    line(&mut o, &format!(" ── artifacts: \x1b[32m{} == pin\x1b[0m · \x1b[33m{} host drift\x1b[0m · {} unpinned   ({} pinned in all)",
        s.pins_ok, s.drift, s.unpinned, s.pins_total));
    // the bottom block: the last notes, then the key hint; the artifacts fill what is left
    let notes: Vec<&String> = s.notes.iter().rev().take(4).collect();
    let used = 12 + PHASES.len() + notes.len();
    let room = h.saturating_sub(used).max(3);
    let from = s.seals.len().saturating_sub(room);
    for (l, c) in &s.seals[from..] {
        let col = match c.as_str() { "ok" => "", "drift" => "\x1b[33m", _ => "\x1b[2m" };
        line(&mut o, &format!("{col}{}\x1b[0m", clip(l, w.saturating_sub(1))));
    }
    for _ in s.seals.len() - from..room { line(&mut o, ""); }
    line(&mut o, &format!(" ── checks {}", "─".repeat(w.saturating_sub(12).min(86))));
    for n in notes.iter().rev() {
        let mark = if n.starts_with("note:") || n.starts_with("stop") { "\x1b[33m!\x1b[0m" } else { "\x1b[32m✓\x1b[0m" };
        line(&mut o, &format!("  {mark} {n}"));
    }
    line(&mut o, "");
    let hint = if s.confirm { "\x1b[7m stop after the running step? y / n \x1b[0m" } else { "\x1b[2m q: stop after the running step (rerun with --resume to continue)\x1b[0m" };
    o.push_str(&clip(hint, w + 16));
    o.push_str("\x1b[K\x1b[J");
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(o.as_bytes());
    let _ = out.flush();
}

/// Whether init has run (errors before it are reported plainly by main).
pub fn active() -> bool { UI.get().is_some() }
fn u_title() -> String { ui().st.lock().unwrap().title.clone() }
