// LAYER4: a free fn returning `-> impl Trait` (e.g. `impl Iterator<Item = T>`) must NOT be mistaken for an impl
// BLOCK by the method-registration prepass. Was: the `impl Iterator` in the return type was treated as
// `impl Iterator { <fn body> }`, consuming the fn body as method registrations and dropping EVERY following
// method (rustc_lexer's `tokenize() -> impl Iterator<Item = Token> + '_` dropped the whole crate's methods).
// SCOPE: the impl-Trait-return fn PARSES and following methods register + run == rustc (ITERATING an impl-Trait
// return is a separate feature, not this rung).
struct Wrap { v: u32 }
fn boxed(n: u32) -> impl Iterator<Item = u32> {
    // exercises the `-> impl Iterator<Item=..>` return-type parse in the prepass outer loop
    std::iter::once(n)
}
struct S { base: i32 }
impl S {
    fn after_a(&self) -> i32 { self.base + 1 }
    fn after_b(&self, x: i32) -> i32 { x * 3 }
    fn after_c(&self, w: Wrap) -> u32 { w.v + 5 }
}
fn main() {
    let s = S { base: 10 };
    println!("{}", s.after_a());
    println!("{}", s.after_b(2));
    println!("{}", s.after_c(Wrap { v: 100 }));
    let _ = boxed;
}
