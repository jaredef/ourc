// NAMERES conservative-strip witness: forward/recursive/mutual calls, drop, iterator adapters, and a
// LEAKED method name (`.len()` on an un-propagated receiver) must ALL compile (not be rejected as E0425).
// Only a bare undefined free call (the separate recurse_undefined_call probe) rejects. rustc-parity below.
fn later(x: i32) -> i32 { x + 1 }
fn ev(n: i32) -> bool { if n == 0 { true } else { od(n - 1) } }
fn od(n: i32) -> bool { if n == 0 { false } else { ev(n - 1) } }
fn main() {
    let s = String::from("hi");
    drop(s);
    let a = [2, 3, 4];
    println!("{} {}", later(3), ev(4));
    println!("{}", a.iter().product::<i32>());
}
