struct S { t: &'static str }
impl S { fn get(&self) -> &str { self.t } }
fn main() { let s = S { t: "hello" }; println!("{}", s.get().len()); }
