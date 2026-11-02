// R37: `for w in "literal".split_whitespace()` (a STRING-LITERAL receiver) now yields the
// whitespace-delimited &str slices. Was: the literal fell through to the generic iterable path
// and produced garbage pointers (only the bound-variable `s.split_whitespace()` was handled).
// This unblocks the canonical word-count idiom (HashMap entry over split_whitespace tokens).
use std::collections::HashMap;

fn main() {
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for w in "the cat sat on the mat the cat".split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("{}", freq["the"]);
    println!("{}", freq["cat"]);
    println!("{}", freq["mat"]);

    let mut n = 0;
    for _w in "a b c d e".split_whitespace() {
        n += 1;
    }
    println!("{}", n);
}
