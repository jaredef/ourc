// R2a P4 guard: Option::take on a local / a Vec slot / in a loop (the place becomes None, the old value is returned);
// iterator take(n) is untouched. (A user `fn take` is r2a_user_take.rs: it switches the desugar off program-wide.)
fn main() {
    let mut o = Some(3);
    let a = o.take();
    let mut v: Vec<Option<String>> = vec![Some(String::from("x")), None];
    let got = match v[0].take() { Some(s) => s, None => String::from("-") };
    let mut w: Vec<Option<i64>> = vec![Some(1), Some(2), None];
    let mut n = 0;
    let mut k = 0;
    while k < w.len() { match w[k].take() { Some(x) => n = n + x, None => n = n + 100 } k += 1; }
    let s: i64 = vec![1, 2, 3, 4].iter().take(2).sum();
    let mut e: Option<String> = None;
    println!("{:?} {:?} {} {} {} {:?} {} {:?}", a, o, got, v[0].is_none(), n, w, s, e.take());
}
