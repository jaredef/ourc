// R2a P4 guard: an Option / Result NESTED in a container keeps its payload type -- index, borrow, clone, unwrap, iterate,
// &Vec param, fn return, struct field, Box, mem::replace on a Vec slot.
struct S { slots: Vec<Option<String>> }
struct P { n: String }
fn f() -> Vec<Option<String>> { vec![None, Some(String::from("r"))] }
fn g(v: &Vec<Option<String>>) -> usize { match &v[0] { Some(s) => s.len(), None => 0 } }
fn main() {
    let mut v: Vec<Option<String>> = vec![Some(String::from("x")), None];
    match &v[0] { Some(s) => print!("{} ", s), None => print!("n ") }
    print!("{} {} ", v[0].clone().unwrap(), g(&vec![Some(String::from("xyz"))]));
    for o in &v { if let Some(s) = o { print!("[{}] ", s); } }
    let w = f();
    if let Some(s) = &w[1] { print!("{} ", s); }
    let st = S { slots: vec![Some(String::from("f"))] };
    if let Some(x) = &st.slots[0] { print!("{} ", x); }
    let ps: Vec<Option<P>> = vec![Some(P { n: String::from("p") })];
    if let Some(p) = &ps[0] { print!("{} ", p.n); }
    let rs: Vec<Result<i64, String>> = vec![Ok(4), Err(String::from("bad"))];
    for r in &rs { match r { Ok(n) => print!("ok{} ", n), Err(e) => print!("err{} ", e) } }
    let b: Box<Option<String>> = Box::new(Some(String::from("bx")));
    match *b { Some(s) => print!("{} ", s), None => print!("n ") }
    let a = std::mem::replace(&mut v[0], None);
    match a { Some(s) => println!("got {} {}", s, v[0].is_none()), None => println!("none") }
}
