// LAYER4 (keeper 2026-08-01): release-mode #[cfg(debug_assertions)] conditional compilation, applied as a
// SOURCE-PREPROCESSING pass over SRC before any prepass/parse (Doc 731 alphabet-purity). debug_assertions=FALSE
// (rustc ships the lexer in release): strip #[cfg(debug_assertions)] items (field/stmt/block), keep
// #[cfg(not(debug_assertions))] items. This is exactly rustc_lexer's Cursor: a debug-only `prev` char cache
// (cfg'd field + cfg'd blocks in prev()/bump()). The token behavior is release-mode; prev is debug-only
// introspection never in the token stream. Non-cfg attrs (#[derive], #[allow]) are untouched.
const EOF_CHAR: char = '\0';
#[derive(Clone, Copy)]
struct Cursor {
    len_remaining: usize,
    #[cfg(debug_assertions)]
    prev: char,
}
impl Cursor {
    fn new(n: usize) -> Cursor {
        Cursor {
            len_remaining: n,
            #[cfg(debug_assertions)]
            prev: EOF_CHAR,
        }
    }
    fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }
        #[cfg(not(debug_assertions))]
        {
            EOF_CHAR
        }
    }
    fn bump(&mut self, c: char) {
        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }
        self.len_remaining = self.len_remaining + 1;
    }
}
fn main() {
    let mut cur = Cursor::new(10);
    cur.bump('x');
    cur.bump('y');
    println!("{}", cur.prev() as u32);
    println!("{}", cur.len_remaining);
}
