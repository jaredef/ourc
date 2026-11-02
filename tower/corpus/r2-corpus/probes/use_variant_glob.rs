// LAYER4 L4-1: `use self::E::*` / `use E::*` -- bring a crate-level enum's variants into unqualified scope so a
// bare `A` resolves to `E::A` (value + pattern). Was: bare UNIT variant -> NOELF (only tuple-ctor A(..) resolved).
// This is rustc_lexer's `use self::LiteralKind::*; use self::TokenKind::*` shape.
enum Kind {
    Line,
    Block,
    Ident,
    Other,
}
use self::Kind::*;

fn code(k: Kind) -> i32 {
    match k {
        Line => 1,
        Block => 2,
        Ident => 3,
        Other => 0,
    }
}

fn main() {
    let a = Line;
    let b = Ident;
    println!("{} {}", code(a), code(b));
    println!("{}", matches!(Block, Block));
    let seq = [Line, Ident, Other, Block];
    let mut total = 0;
    for k in seq {
        total += code(k);
    }
    println!("{}", total);
}
