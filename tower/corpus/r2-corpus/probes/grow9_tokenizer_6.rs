fn main() {
    let s = "the quick brown fox";
    let mut count = 0;
    let mut maxlen = 0;
    for w in s.split(' ') {
        count += 1;
        if w.len() > maxlen { maxlen = w.len(); }
    }
    println!("{} {}", count, maxlen);
}
