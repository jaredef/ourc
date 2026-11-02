fn main() {
    let a: i32 = 2_147_483_647;
    let b = a.wrapping_add(1);
    println!("{}", b);
}
