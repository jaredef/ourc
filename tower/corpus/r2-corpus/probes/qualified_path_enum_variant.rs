// LAYER4 module-resolution: qualified-path enum VARIANT values on a module-nested enum -- `m::E::A`,
// `crate::m::E::A`, `m::E::A(x)`. The module-path handler stripped to the last segment for calls/structs; a
// stripped VARIANT name now resolves via var_find (module-nested enums are registered by the linear prepass).
// SCOPE: value + tuple construction. Qualified-path variant PATTERNS `match x { m::E::A(n) => }` are a separate
// residual (the match/if-let pattern parser needs the same strip); rustc_lexer uses `use self::E::*` glob
// patterns (bare, already supported), not qualified ones -- so this value-side win is the load-bearing part.
mod tok {
    #[derive(PartialEq, Clone, Copy)]
    pub enum Kind {
        Line,
        Block,
        Doc,
    }
}
use tok::Kind::*;
fn code(k: tok::Kind) -> i32 {
    match k {
        Line => 1,
        Block => 2,
        Doc => 3,
    }
}
fn main() {
    let a = tok::Kind::Line;
    let b = crate::tok::Kind::Doc;
    println!("{} {}", code(a), code(b));
    println!("{}", a == tok::Kind::Line);
    println!("{}", b == tok::Kind::Block);
}
