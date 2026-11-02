// P-CX-PEEK-SLICE + P-CX-FNPTR-TABLE witness (ourc construct-generation, 2026-10-24): driver-style option parsing
// over &[&str] with a (peekable) cursor, `it.next().ok_or(..)?`, a catch-all `o =>` str binder, extend(split().map(
// String::from)), {:?} of Result<Vec<String>, String>, and a Vec<fn> table called through a loop binder.
fn p(args: &[&str]) -> Result<Vec<String>, String> { let mut out = Vec::new(); let mut it = args.iter().peekable();
    while let Some(&a) = it.next() { match a { "-n" => out.push(it.next().ok_or("needs value")?.to_string()), s if s.starts_with("--e=") => out.extend(s[4..].split(',').map(String::from)), o => return Err(format!("bad `{}`", o)) } } Ok(out) }
fn inc(x: i64) -> i64 { x + 1 } fn dbl(x: i64) -> i64 { x * 2 }
fn main() {
    println!("{:?} {:?} {:?}", p(&["-n", "x", "--e=a,b"]), p(&["-n"]), p(&["-z"]));
    let v: Vec<String> = "a,b".split(',').map(String::from).collect(); println!("{:?}", v);
    let fs: Vec<fn(i64) -> i64> = vec![inc, dbl]; for f in &fs { println!("{}", f(5)); }
}
