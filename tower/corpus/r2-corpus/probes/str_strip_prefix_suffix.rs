// LAYER4 leaf-crate frontier: str::strip_prefix / strip_suffix -> Option<&str>. rustc_lexer uses
// input.strip_prefix("#!") to skip a shebang line. Modeled on starts_with (__str_match) + split_once's
// Option<&str> construction: Some([hp+preflen, hl-preflen]) / Some([hp, hl-suflen]) if the affix matches,
// else None. Payload is a &str (EOPTK=1) so the if-let/unwrap binder restores &str kind.
fn shebang_rest(s: &str) -> &str {
    match s.strip_prefix("#!") {
        Some(r) => r,
        None => s,
    }
}
fn main() {
    println!("{}", shebang_rest("#!/bin/sh"));
    println!("{}", shebang_rest("no shebang"));
    let name = "module.rs";
    if let Some(stem) = name.strip_suffix(".rs") {
        println!("{}", stem);
    } else {
        println!("none");
    }
    // chained
    let r = "##body".strip_prefix("##").unwrap();
    println!("{}", r.len());
    // no-match suffix
    println!("{}", "a.txt".strip_suffix(".rs").is_none());
}
