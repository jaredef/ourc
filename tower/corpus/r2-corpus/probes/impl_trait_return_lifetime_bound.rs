fn doubled(v: &Vec<i64>) -> impl Iterator<Item = i64> + '_ {
    v.iter().map(|x| x * 2)
}
fn main() {
    let v: Vec<i64> = vec![1, 2, 3, 4];
    let mut s: i64 = 0;
    for x in doubled(&v) { s = s + x; }
    println!("{}", s);
}
