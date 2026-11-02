fn main() {
    let data = [1i64, 2, 3, 4, 5, 6, 7, 8];
    let mut h: i64 = 5381;
    let mut i = 0;
    while i < data.len() {
        h = h.wrapping_mul(33).wrapping_add(data[i]);
        i += 1;
    }
    println!("{}", h);
}
