// RUSTC-TARGET (P-STMT-ATTRIBUTE): a statement-position attribute `#[allow(..)]` / `#[cfg(..)]` on a `let`
// or expression inside a fn body. rzrs skipped item-level attrs but not statement-level ones, so
// `#[allow(unused)] let x = 5;` mis-parsed the attribute as an `allow(unused)` call -> E0425. Pervasive
// (either uses `#[allow(invalid_from_utf8)]` / `#[allow(deprecated)]` on inner `let`s). parse_stmt now
// skips a leading `#[..]` (and stacked attrs).
fn main() {
    #[allow(unused)]
    let x = 5;
    #[allow(a)] #[allow(b)]
    let y = 9;
    let v = vec![1, 2, 3];
    #[allow(deprecated)]
    let s: i32 = v.iter().sum();
    println!("{} {} {}", x, y, s);
}
