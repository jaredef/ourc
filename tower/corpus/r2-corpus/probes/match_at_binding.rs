// `x @ range` binding inside a variant payload -- P-MATCH-PATTERN rung 2:
// match_pat_payload bound the payload var but ignored the `@ subpat`, leaving it
// unparsed -> `)` desync -> empty output. Now the range/literal test on the payload
// value is AND'd into the arm (mirrors the scalar @ path).
enum E { N(i32), M(i32) }
fn classify(v: E) -> i32 {
    match v {
        E::N(x @ 1..=10) => x * 3,
        E::N(y) => y + 100,
        E::M(z @ 5..=7) => z * 10,
        E::M(w) => w,
    }
}
fn main() {
    print!("{} ", classify(E::N(4)));
    print!("{} ", classify(E::N(15)));
    print!("{} ", classify(E::M(6)));
    print!("{} ", classify(E::M(20)));
    // scalar @ still works
    let s = 5;
    println!("{}", match s { n @ 1..=10 => n * 2, _ => 0 });
}
