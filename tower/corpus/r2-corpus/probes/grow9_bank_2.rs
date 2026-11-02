struct Account { balance: i64 }
impl Account {
    fn deposit(&mut self, x: i64) { self.balance += x; }
    fn withdraw(&mut self, x: i64) -> bool {
        if self.balance >= x { self.balance -= x; true } else { false }
    }
}
fn main() {
    let mut a = Account { balance: 100 };
    a.deposit(50);
    let ok = a.withdraw(30);
    let bad = a.withdraw(1000);
    println!("{} {} {}", a.balance, ok, bad);
}
