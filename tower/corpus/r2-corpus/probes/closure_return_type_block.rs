// LAYER3 M2: closure with an explicit return type + block body `|x| -> T { .. }`. parse_closure now skips `-> T`
// (a return type has no `{`, and such a closure always has a block body) so body detection lands on the `{`.
// Scoped to direct closures + impl-Fn args; the iter-adapter map/filter path parses closures separately
// (`.map(|x| -> T { .. })` is a pre-existing separate gap, recorded).
fn apply(f: impl Fn(i32) -> i32, v: i32) -> i32 {
    f(v)
}
fn main() {
    let triple = |x: i32| -> i32 { x * 3 };
    println!("{}", triple(7));
    let poly = |a: i32, b: i32| -> i32 {
        let s = a + b;
        s * s
    };
    println!("{}", poly(2, 3));
    println!("{}", apply(|x| -> i32 { x + 100 }, 5));
    let neg = |x: i32| -> i32 {
        if x < 0 { 0 - x } else { x }
    };
    println!("{}", neg(-8));
}
