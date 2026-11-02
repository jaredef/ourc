// matches!(scrut, pat) -- P-MATCH-PATTERN rung 3: matches! only handled bare int
// literals + variant disc (which eload'd a scalar scrutinee -> segfault on a binding).
// Now: int/char/byte literals, `lo..=hi` ranges, variant disc, binding `n`, `if GUARD`,
// and the result is bool-tagged (was printed 0/1).
enum E { A, B }
fn main() {
    let x = 7;
    println!("{}", matches!(x, 5));            // false
    println!("{}", matches!(x, 5..=10));       // true
    println!("{}", matches!(x, n if n > 5));   // true
    println!("{}", matches!(x, 0..=5 | 20));   // false
    let v = E::A;
    println!("{}", matches!(v, E::A));         // true
    let c = 'e';
    println!("{}", matches!(c, 'a' | 'e' | 'i')); // true
    let n = "aeiou b c".bytes()
        .filter(|&b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')).count();
    println!("{}", n);                          // 5
}
