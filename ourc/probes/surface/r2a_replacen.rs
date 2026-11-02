// R2a P4 guard: str::replacen (first n matches) and the empty-pattern rule for replace / replacen (a match at every char
// boundary, end included; UTF-8 aware).
fn main() {
    let n = ".a.b";
    println!("{} {} {}", n.replacen(".", "$", 1), "xxx".replacen("x", "yz", 2), "a-b-c".replacen("-", "+", 9));
    println!("[{}] [{}] [{}]", "aaaa".replacen("aa", "b", 1), "abc".replacen("", "-", 2), "ab".replacen("", "_", 0));
    println!("[{}] [{}] [{}]", "abc".replace("", "-"), "".replace("", "x"), "héé".replace("", "|"));
}
