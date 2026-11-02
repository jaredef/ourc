struct C { p: i64 }
impl C {
    fn first(&self) -> char { 'a' }
    fn second(&self) -> char { 'b' }
    fn tag(&self) -> i64 { 9 }
    fn go(&self) -> i64 {
        let k = match self.first() {
            '/' => 1,
            c if !c.is_ascii() && c.is_alphabetic() => 2,
            _ => 0,
        };
        let m = match self.second() {
            'b' => 10,
            _ => 0,
        };
        k + m + self.tag()
    }
}
fn main() { let c = C { p: 0 }; println!("{}", c.go()); }
