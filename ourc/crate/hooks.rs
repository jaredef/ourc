
// ---- the host hooks, for the rustc build (vendor.sh appends this file and drops odrv's stubs) ----
// When ourc compiles itself, olower swaps these calls for libc (system, a file write, alarm + setrlimit); under rustc
// they are plain std, with the same answers and the same diagnostics.

// libc `system`: the command's exit code; a command killed by a signal, or one that cannot start, is a failure
fn __ourc_system(c: &str) -> i64 {
    match std::process::Command::new("/bin/sh").arg("-c").arg(c).status() {
        Ok(s) => match s.code() {
            Some(x) => (x & 255) as i64,
            None => 1,
        },
        Err(_) => 1,
    }
}
// 0 = written, 1 = could not write
fn __ourc_writefile(p: &str, t: &str) -> i64 {
    if std::fs::write(p, t).is_ok() {
        0
    } else {
        1
    }
}
// --time-limit: a watchdog thread; --mem-limit: RLIMIT_AS, and BudgetAlloc turns a refused allocation into the message
fn __ourc_budget(secs: i64, gb: i64) -> i64 {
    if secs > 0 {
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(secs as u64));
            eprintln!("ourc: compile time limit exceeded -- refused (raise it with --time-limit N, 0 = none)");
            std::process::exit(1);
        });
    }
    if gb > 0 {
        let lim = [(gb as u64) << 30, (gb as u64) << 30];
        unsafe {
            setrlimit(9, lim.as_ptr()); // RLIMIT_AS
        }
        BUDGET_ON.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    0
}
extern "C" {
    fn setrlimit(resource: i32, rlim: *const u64) -> i32;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
    fn _exit(code: i32) -> !;
}
static BUDGET_ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
struct BudgetAlloc;
unsafe impl std::alloc::GlobalAlloc for BudgetAlloc {
    unsafe fn alloc(&self, l: std::alloc::Layout) -> *mut u8 {
        oom_check(std::alloc::System.alloc(l))
    }
    unsafe fn alloc_zeroed(&self, l: std::alloc::Layout) -> *mut u8 {
        oom_check(std::alloc::System.alloc_zeroed(l))
    }
    unsafe fn realloc(&self, p: *mut u8, l: std::alloc::Layout, n: usize) -> *mut u8 {
        oom_check(std::alloc::System.realloc(p, l, n))
    }
    unsafe fn dealloc(&self, p: *mut u8, l: std::alloc::Layout) {
        std::alloc::System.dealloc(p, l)
    }
}
// write + _exit only: nothing here may allocate
unsafe fn oom_check(p: *mut u8) -> *mut u8 {
    if p.is_null() && BUDGET_ON.load(std::sync::atomic::Ordering::Relaxed) {
        let m = b"ourc: memory limit exceeded -- refused (raise it with --mem-limit GB, 0 = none)\n";
        write(2, m.as_ptr(), m.len());
        _exit(1);
    }
    p
}
#[global_allocator]
static GLOBAL: BudgetAlloc = BudgetAlloc;
