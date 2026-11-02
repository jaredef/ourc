// cx02 macro expansion: recursive macro_rules (MACRORX) + a rustc_expand-shaped token-tree
// matcher/transcriber with nested Delimited groups and $x:tt repetition, re-expanding output.
macro_rules! count { () => { 0usize }; ($h:tt $($t:tt)*) => { 1usize + count!($($t)*) }; }
macro_rules! rev { ([] $($r:expr),*) => { vec![$($r),*] }; ([$h:expr $(, $t:expr)*] $($r:expr),*) => { rev!([$($t),*] $h $(, $r)*) }; }
macro_rules! maxm { ($x:expr) => { $x }; ($x:expr, $($y:expr),+) => { { let a = $x; let b = maxm!($($y),+); if a > b { a } else { b } } }; }
#[derive(Debug, Clone, PartialEq)]
enum TT { Tok(String), Delim(char, Vec<TT>) }
fn tts(s: &str) -> Vec<TT> {
    fn go(it: &mut std::iter::Peekable<std::vec::IntoIter<String>>) -> Vec<TT> {
        let mut out = Vec::new();
        while let Some(t) = it.next() {
            match t.as_str() { "(" | "[" => { let d = t.chars().next().unwrap(); out.push(TT::Delim(d, go(it))); } ")" | "]" => return out, _ => out.push(TT::Tok(t)) }
        }
        out
    }
    let words: Vec<String> = s.split_whitespace().map(|w| w.to_string()).collect();
    go(&mut words.into_iter().peekable())
}
// Rule: name ( $x ... ) => body with $x substituted; a body can call the macro again -> re-expanded.
struct Rule { name: String, params: usize, body: Vec<TT> }
fn subst(body: &[TT], args: &[Vec<TT>]) -> Vec<TT> {
    let mut out = Vec::new();
    for t in body { match t { TT::Tok(s) if s.starts_with('$') => { let i: usize = s[1..].parse().unwrap(); out.extend(args[i].iter().cloned()); }
        TT::Delim(d, inner) => out.push(TT::Delim(*d, subst(inner, args))), _ => out.push(t.clone()) } }
    out
}
fn expand(ts: Vec<TT>, rules: &[Rule], depth: usize) -> Vec<TT> {
    let mut out = Vec::new(); let mut i = 0;
    while i < ts.len() {
        if let TT::Tok(name) = &ts[i] {
            if let Some(r) = rules.iter().find(|r| &r.name == name) {
                if let Some(TT::Delim('(', args)) = ts.get(i + 1) {
                    let groups: Vec<Vec<TT>> = args.split(|t| *t == TT::Tok(",".to_string())).map(|g| g.to_vec()).collect();
                    assert_eq!(groups.len(), r.params);
                    let e = subst(&r.body, &groups);
                    out.extend(if depth < 8 { expand(e, rules, depth + 1) } else { e });
                    i += 2; continue;
                }
            }
        }
        out.push(match &ts[i] { TT::Delim(d, inner) => TT::Delim(*d, expand(inner.clone(), rules, depth)), t => t.clone() });
        i += 1;
    }
    out
}
fn show(ts: &[TT]) -> String { ts.iter().map(|t| match t { TT::Tok(s) => s.clone(), TT::Delim(d, i) => format!("{}{}{}", d, show(i), if *d == '(' { ')' } else { ']' }) }).collect::<Vec<_>>().join(" ") }
fn main() {
    println!("count={} {} {}", count!(), count!(a), count!(a b (c d) [e]));
    println!("rev={:?}", rev!([1, 2, 3, 4]));
    println!("max={}", maxm!(3, 9, 2, 7));
    let rules = vec![
        Rule { name: "dbl".into(), params: 1, body: tts("( $0 + $0 )") },
        Rule { name: "quad".into(), params: 1, body: tts("dbl ( dbl ( $0 ) )") },
        Rule { name: "pair".into(), params: 2, body: tts("[ quad ( $0 ) , $1 ]") },
    ];
    for src in ["dbl ( x )", "quad ( y )", "pair ( a , dbl ( b ) )", "f ( quad ( 1 ) )"] { println!("{} => {}", src, show(&expand(tts(src), &rules, 0))); }
}
