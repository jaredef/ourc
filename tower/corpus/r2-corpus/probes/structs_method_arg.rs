struct Acc { n: i64 }
impl Acc { fn addv(&self, k: i64) -> i64 { self.n + k } }
fn main() {
    let a = Acc { n: 100 };
    println!("{}", a.addv(23));
}
