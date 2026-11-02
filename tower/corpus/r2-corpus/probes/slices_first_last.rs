fn main() {
    let a = [5, 6, 7, 8];
    let f = *a.first().unwrap();
    let l = *a.last().unwrap();
    println!("{}", f * 100 + l);
}
