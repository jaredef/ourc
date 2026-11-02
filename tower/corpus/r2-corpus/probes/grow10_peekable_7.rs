fn main() {
    let v = vec![1,2,2,3,3,3];
    let mut it = v.iter().peekable();
    let mut groups = 0;
    while let Some(&x) = it.next() {
        groups += 1;
        while it.peek() == Some(&&x) { it.next(); }
    }
    println!("{}", groups);
}
