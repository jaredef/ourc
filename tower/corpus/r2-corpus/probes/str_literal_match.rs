// &str-literal match patterns -- regression guard for rung277 P-STR-MATCH: SEM0 compiled a `"lit" =>` arm as a
// WILDCARD (no comparison; the first arm always won), in top-level, or-pattern, guarded, nested-tuple and
// Option-payload position alike. Also guards the two siblings found with it: a whole-scrutinee binding
// (`other => other.len()`) of a &str/&String scrutinee deref-loaded the header pointer (SEGV), and a by-value
// tuple scrutinee containing a reference ((&str, i32)) was treated as by-ref, binding `n` to an address.
fn rw(w: &str) -> bool { match w { "abstract" => true, "macro" => true, _ => false } }
fn orp(w: &str) -> i32 { match w { "a" | "bb" | "ccc" => 1, "d" => 2, _ => 3 } }
fn grd(w: &str, n: i32) -> i32 { match w { "x" if n > 5 => 10, "x" => 11, other => other.len() as i32 } }
fn name(k: i32) -> String { let s = if k == 0 { String::from("zero") } else { String::from("one") }; match s.as_str() { "zero" => String::from("Z"), "one" => String::from("O"), _ => String::from("?") } }
fn tup(t: (&str, i32)) -> i32 { match t { ("x", n) => n, ("y", _) => 100, _ => 0 } }
fn opt(o: Option<&str>) -> i32 { match o { Some("a") => 1, Some(_x) => 2, None => 3 } }
fn rr(ws: &Vec<&str>) -> i32 { let mut c = 0; for w in ws.iter() { match *w { "fn" | "let" => { c += 1; } _ => {} } } c }
fn wb(w: &String) -> usize { match w { o => o.len() } }
fn it(v: &Vec<String>) -> usize { let mut t = 0; for x in v.iter() { match x { o => { t += o.len(); } } } t }
fn main() {
    println!("{} {} {} {}", rw("abstract"), rw("macro"), rw("x"), rw("abstrac"));
    println!("{} {} {} {} {}", orp("a"), orp("bb"), orp("ccc"), orp("d"), orp("e"));
    println!("{} {} {}", grd("x", 9), grd("x", 1), grd("hello", 0));
    println!("{} {}", name(0), name(1));
    println!("{} {} {}", tup(("x", 7)), tup(("y", 7)), tup(("z", 7)));
    println!("{} {} {}", opt(Some("a")), opt(Some("b")), opt(None));
    let v = vec!["fn", "x", "let", "y", "fn"]; println!("{}", rr(&v));
    println!("{} {}", wb(&String::from("abc")), it(&vec![String::from("ab"), String::from("cde")]));
}
