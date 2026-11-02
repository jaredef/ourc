#[derive(Clone)]
struct P { name: i64, age: i64 }
fn main() {
    let mut people = vec![P{name:1,age:30}, P{name:2,age:20}, P{name:3,age:25}];
    people.sort_by_key(|p| p.age);
    let mut acc = 0;
    for p in &people { acc = acc * 10 + p.name; }
    println!("{}", acc);
}
