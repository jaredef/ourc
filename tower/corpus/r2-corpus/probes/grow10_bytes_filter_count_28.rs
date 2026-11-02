fn main() {
    let s = "hello world foo";
    let vowels = s.bytes().filter(|&b| matches!(b, b'a'|b'e'|b'i'|b'o'|b'u')).count();
    let spaces = s.bytes().filter(|&b| b == b' ').count();
    println!("{} {}", vowels, spaces);
}
