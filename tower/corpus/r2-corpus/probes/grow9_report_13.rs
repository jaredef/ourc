struct Item { name: &'static str, qty: i64, price: i64 }
fn main() {
    let items = [Item{name:"a",qty:2,price:3}, Item{name:"b",qty:1,price:10}, Item{name:"c",qty:5,price:2}];
    let mut report = String::new();
    let mut total = 0;
    for it in items.iter() {
        let line = it.qty * it.price;
        total += line;
        report.push_str(&format!("{}:{} ", it.name, line));
    }
    report.push_str(&format!("total={}", total));
    println!("{}", report);
}
