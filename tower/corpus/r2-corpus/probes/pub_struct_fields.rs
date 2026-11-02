// LAYER4 (L4-2 root): `pub` on struct FIELDS (`pub struct T { pub kind: K, pub len: u32 }`). The named-field
// prepass read `pub` as the field name -> desync/FE-reject; now skips `pub`/`pub(crate)` before the field name.
// This is rustc_lexer's ubiquitous shape (pub struct Token { pub kind, pub len }). (The mis-attributed "module
// resolution" origin: the failures were pub-fields, not modules; qualified m::path is the separate next rung.)
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}
#[derive(PartialEq)]
pub enum TokenKind {
    Ident,
    Whitespace,
    Other,
}
pub struct Named {
    pub name: String,
    pub(crate) count: i32,
    weight: i32,
}
fn main() {
    let t = Token { kind: TokenKind::Ident, len: 5 };
    println!("{} {}", matches!(t.kind, TokenKind::Ident), t.len);
    let n = Named { name: String::from("cruft"), count: 3, weight: 2 };
    println!("{} {} {}", n.name, n.count, n.weight);
}
