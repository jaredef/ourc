// R39: `.map(|x| { let ..; tail })` with a BLOCK closure body (statements + a tail expr) now works.
// Was NOELF -- the map body was parsed as a single expression, so any `{ }` block bailed. The tail
// expr is captured via the same TAILDST mechanism the if-expr branch capture uses. Covers i32 and
// String (format!) results. The capitalize-map idiom (census2's `caps`) rides on this.
fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|x| {
        let y = x + 1;
        y * 2
    }).collect();
    println!("{:?}", doubled);

    let words = vec!["apple", "banana", "cherry"];
    let caps: Vec<String> = words.iter().map(|w| {
        let first = w.chars().next().unwrap().to_ascii_uppercase();
        format!("{}{}", first, &w[1..])
    }).collect();
    println!("{:?}", caps);
}
