// R2bx: .map(|s| ...) over a Vec<&str>/Vec<String> must bind the closure param as a str descriptor (SK=1/3)
// so the body's string methods dispatch (s.len(), s.to_uppercase()). The param was bound as a bare scalar
// (str element carried via the interned Ty TYTAG 3/4, not DVELT==-2), so s.len() misfired/segfaulted.
fn main() {
    let v = vec!["ab", "cde", "f"];
    let total: usize = v.iter().map(|s| s.len()).sum();
    let up: Vec<String> = v.iter().map(|s| s.to_uppercase()).collect();
    let lens: Vec<usize> = v.iter().map(|s| s.len()).collect();
    println!("{} {:?} {}", total, up, lens.len());
}
