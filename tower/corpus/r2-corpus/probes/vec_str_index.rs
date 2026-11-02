// R27b: `v[i]` on a bound Vec<&str>/Vec<String> must Display/dispatch as a string. The element was carried via
// dvelty0 (TYTAG 3/4), not dvelt0==-2, so the index read printed a pointer. Now it tags &str/String.
fn main() {
    let v = vec!["a", "bb", "ccc"];
    let s = vec![String::from("x"), String::from("yy")];
    println!("{} {} {} {}", v[0], v[2].len(), v[1].to_uppercase(), s[1]);
}
