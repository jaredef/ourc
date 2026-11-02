enum K { A, B }
struct C;
impl C {
    fn first(&self) -> char { '"' }
    fn go(
        &self,
        mk_kind: impl FnOnce(bool) -> K,
        mk_kind_raw: impl FnOnce(u8) -> K,
        single_quoted: Option<fn(bool) -> K>,
    ) -> i64 {
        let k = match (self.first(), single_quoted) {
            ('\'', Some(mk_kind)) => mk_kind(true),
            ('"', _) => mk_kind(true),
            _ => mk_kind_raw(9),
        };
        match k { K::A => 1, K::B => 2 }
    }
}
fn main() { let c = C; println!("{}", c.go(|t| if t { K::B } else { K::A }, |_n| K::A, None)); }
