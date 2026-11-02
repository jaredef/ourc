struct S { t: String }
impl S { fn g(&self) -> &str { &self.t } }
fn main() { let s = S { t: String::from("hello") }; println!("{} {}", s.g().len(), s.g().starts_with("he")); }
