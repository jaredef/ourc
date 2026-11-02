// RUSTC-TARGET CONSTEVAL (P-CONSTEVAL-STATICMUT-ARRAY-NO-FOLD): indexing a `static mut`
// zero-filled array must NOT be const-folded as an E0080 candidate. A `static [mut] NAME: [T; N] = [0; N]`
// replays ZERO initializers (the zero-fill is optimized away), so its replay-count is 0 -- which is NOT
// its logical length. Folding `NAME[idx]` against that 0-length (or reading its non-replayed initializer
// value) is doubly wrong: a static mut is runtime-mutated, so its elements are not compile-time constants.
// This is the SELF-HOST shape: rzrs's own `static mut TYTAG: [i32; 4096] = [0; 4096]` indexed by a runtime
// value made the compiler reject its own source with a spurious `index oob`. Only genuine `const` arrays
// (all inits replayed; immutable) are value-foldable.
static mut TAB: [i32; 8] = [0; 8];
fn main() {
    unsafe {
        TAB[0] = 10;
        TAB[3] = 40;
        let mut i = 0;
        let mut sum = 0;
        while i < 8 { sum = sum + TAB[i]; i = i + 1; }
        println!("{} {} {}", TAB[0], TAB[3], sum);
    }
}
