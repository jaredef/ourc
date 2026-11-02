struct C { p: i64 }
impl C {
    fn first(&self) -> char { '7' }
    fn depth(&self) -> i64 { -5 }
    fn go(&self) -> i64 {
        let mut n = 0i64;
        if self.first().is_ascii_digit() { n = n + 1; }
        if self.first().is_alphabetic() { n = n + 10; }
        n = n + self.depth().abs();
        n
    }
}
fn main() { let c = C { p: 0 }; println!("{}", c.go()); }
