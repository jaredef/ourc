use std::cmp::Ordering;
fn main() {
    let mut out = String::new();
    for (a,b) in [(1,2),(5,5),(9,3)] {
        let s = match a.cmp(&b) {
            Ordering::Less => "L",
            Ordering::Equal => "E",
            Ordering::Greater => "G",
        };
        out.push_str(s);
    }
    println!("{}", out);
}
