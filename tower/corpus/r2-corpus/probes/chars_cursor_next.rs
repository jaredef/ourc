// L4-3 P-CHARS-ITER: a STORED `str::Chars` cursor -- `let mut it = s.chars(); it.next()` -> Option<char>,
// advancing a retained position. This is the lexer cursor primitive (rustc_lexer's Cursor wraps Chars and
// pulls chars off one at a time via .next()). Bare `s.chars()` now builds a [buf,bytelen,pos] cursor (SK=13,
// dispatch_char_iter) instead of returning the length; .next() loads the byte at pos, advances, yields
// Some(char)/None. Scope: comparison + `as u32` + while-let (what the lexer does); Displaying an unwrapped
// char via `if let Some(c)` is a separate PRE-EXISTING Option-payload-char-tag gap (holds inline too).
fn first_slash(s: &str) -> i32 {
    let mut it = s.chars();
    let mut i = 0;
    while let Some(c) = it.next() {
        if c == '/' {
            return i;
        }
        i = i + 1;
    }
    0 - 1
}
fn main() {
    let s = "ab/cd";
    let mut it = s.chars();
    // first char as codepoint
    if let Some(c) = it.next() {
        println!("{}", c as u32);
    }
    // second char, advanced
    if let Some(c) = it.next() {
        println!("{}", c as u32);
    }
    // count remaining via the cursor
    let mut n = 0;
    while let Some(_c) = it.next() {
        n = n + 1;
    }
    println!("{}", n);
    // None at end
    println!("{}", it.next().is_none());
    // cursor-driven scan
    println!("{}", first_slash(s));
    println!("{}", first_slash("nope"));
}
