struct Acc { sum: i32, cnt: i32 }
fn main() {
    let v = vec![2, 4, 6, 8];
    let a = v.iter().fold(Acc { sum: 0, cnt: 0 }, |acc, &x| Acc { sum: acc.sum + x, cnt: acc.cnt + 1 });
    println!("{}", a.sum + a.cnt);
}
