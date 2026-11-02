fn main() {
    let mut flags: i64 = 0;
    flags |= 1 << 3;
    flags |= 1 << 7;
    let test3 = (flags >> 3) & 1;
    flags &= !(1 << 3);
    let test3b = (flags >> 3) & 1;
    println!("{} {} {}", flags, test3, test3b);
}
