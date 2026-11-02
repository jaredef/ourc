fn main() {
    let v = vec![1, 5, 3, 9, 2];
    let found = v.iter().find_map(|&x| {
        if x > 4 { return Some(x * 2); }
        None
    });
    println!("{}", found.unwrap_or(0));
}
