// RUSTC-TARGET S6 (P-MACRO-RULES robustness): a `#[..]` / bracketed token inside a macro_rules matcher
// must not hang the matcher scanner (surfaced by censusing the real `itoa` crate, which spun rzrs).
macro_rules! decl { (#[cfg(x = $w:literal)] $t:ident) => { struct $t; }; }
macro_rules! two { ($a:ident as $b:ident) => { struct $a; struct $b; }; }
two!(Foo as Bar);
fn main() { println!("ok"); }
