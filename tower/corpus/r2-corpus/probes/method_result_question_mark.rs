// LAYER4 method-? emit sub-arc (keeper 2026-08-01): a METHOD returning `-> Result<T,E>` / `-> Option<T>`,
// called with `?` (`self.inner(prefix_len)?`), now routes through opt_postfix so the `?` early-return emits
// (brz/ret/payload-extract) AND die_qmark passes on Result. Was: dispatch_struct's method path returned the
// non-struct method result WITHOUT opt_postfix -> `?` mis-emitted (no-arg emitted an identity, with-arg died
// "`?` on Option"). Fix: prepass records Result(4)/Option(5) return kind (via mret_str); dispatch_struct routes
// a `?`-followed non-struct method return through opt_postfix carrying EOPTRES. rustc_lexer's
// raw_double_quoted_string (`self.raw_string_unvalidated(prefix_len)?`) needs this.
enum E { Bad }
struct S { base: u32 }
impl S {
    fn inner(&self, p: u32) -> Result<u32, E> {
        if p > 0 { Ok(p + self.base) } else { Err(E::Bad) }
    }
    fn outer(&self, p: u32) -> Result<u32, E> {
        let n = self.inner(p)?;
        Ok(n + 1)
    }
    fn opt_inner(&self, p: u32) -> Option<u32> {
        if p > 0 { Some(p) } else { None }
    }
    fn opt_outer(&self, p: u32) -> Option<u32> {
        let n = self.opt_inner(p)?;
        Some(n * 2)
    }
    // no-arg form (was pre-broken too)
    fn a(&self) -> Result<u32, E> { Ok(3) }
    fn b(&self) -> Result<u32, E> { Ok(self.a()? + 10) }
}
fn main() {
    let s = S { base: 100 };
    match s.outer(2) { Ok(n) => println!("ok {}", n), Err(_) => println!("err") }
    match s.outer(0) { Ok(n) => println!("ok {}", n), Err(_) => println!("err") }
    match s.opt_outer(5) { Some(n) => println!("some {}", n), None => println!("none") }
    match s.opt_outer(0) { Some(n) => println!("some {}", n), None => println!("none") }
    match s.b() { Ok(n) => println!("b {}", n), Err(_) => println!("berr") }
}
