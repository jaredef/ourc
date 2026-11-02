// for p in s.split(SEP) -- regression guard for TI3e-n (N-split-forloop):
// the split for-loop (variable receiver) was unhandled -> iterated the string
// itself yielding garbage; .collect() worked but the direct for-loop did not.
// Now yields sep-delimited &str fields (single-char sep, keeps empty fields).
fn main() {
    let s = "1,2,3,4";
    let mut sum = 0;
    for p in s.split(',') { let n: i32 = p.parse().unwrap(); sum += n; }
    println!("{}", sum);
    let line = "name age city";
    let mut fields = 0;
    for _f in line.split(' ') { fields += 1; }
    println!("{}", fields);
    let owned = String::from("a:b:c");
    let mut last = String::new();
    for p in owned.split(':') { last = p.to_string(); }
    println!("{}", last);
    // trailing separator keeps the empty field
    let t = "x,y,";
    let mut c = 0;
    for _p in t.split(',') { c += 1; }
    println!("{}", c);
}
