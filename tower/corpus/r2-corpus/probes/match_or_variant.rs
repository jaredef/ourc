// or-pattern of enum variants `E::A(n) | E::B(n)` -- P-MATCH-PATTERN rung 1:
// the `|` after a variant arm was left unconsumed -> parse desync -> segfault.
// Now each alternative's disc test is OR'd; the shared payload binds from the
// first (same +4 offset). E0408 (a var bound in one alt but not all) is rejected.
enum E { A(i32), B(i32), C(i32) }
fn main() {
    let mut total = 0;
    for v in [E::A(1), E::B(2), E::C(3), E::A(4)] {
        total += match v {
            E::A(n) | E::B(n) => n,
            E::C(n) => n * 100,
        };
    }
    println!("{}", total);
    // three-way alternation
    let w = E::C(5);
    let r = match w { E::A(n) | E::B(n) | E::C(n) => n };
    println!("{}", r);
}
