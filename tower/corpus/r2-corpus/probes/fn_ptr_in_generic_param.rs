// LAYER4: a fn-pointer type nested in a generic as a method parameter -- `Option<fn(bool) -> K>`. parse_pb's
// param-type generic <..> skip must be ->-aware: `fn(bool) -> K` contains `->`, and the `>` of `->` must NOT close
// the `Option<` early (was: closed at the `->`, leaving `K>` -> parse drift, which desynced EVERY method AFTER
// such a param). rustc_lexer's c_or_byte_string has `single_quoted: Option<fn(bool) -> LiteralKind>`. SCOPE: the
// method with the fn-ptr-in-generic param PARSES and the FOLLOWING methods register + run == rustc (calling a
// fn-ptr from an Option payload is a separate feature, not this rung).
enum Kind { Word, Sym }
struct S { tag: i32 }
impl S {
    fn classify(&self, _mk: Option<fn(bool) -> Kind>, _mk2: impl Fn(u8) -> Kind, n: i32) -> i32 { n + 1 }
    fn after_a(&self) -> i32 { self.tag + 10 }
    fn after_b(&self, x: i32) -> i32 { x * 2 }
}
fn main() {
    let s = S { tag: 7 };
    println!("{}", s.after_a());
    println!("{}", s.after_b(5));
}
