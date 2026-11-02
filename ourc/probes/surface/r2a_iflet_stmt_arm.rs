// R2a P4 guard: an `if let` WITHOUT else whose block ends in a str-valued statement is a statement (`()`), not a
// match whose arms disagree (str vs the `_ => ()` arm); ourc refused it as "match arms have incompatible types".
enum E {
    Var(String),
    Num(i64),
}
fn name(o: &Box<E>) -> String {
    let mut out = String::from("?");
    if let E::Var(n) = o.as_ref() {
        if n.len() > 0 {
            out = n.clone();
        }
    }
    out
}
fn main() {
    println!("{} {}", name(&Box::new(E::Var(String::from("x")))), name(&Box::new(E::Num(3))));
    check2();
}
// (2) a match whose arms are a diverging `return` of a String and a `{}` statement arm
fn first_word(s: &str) -> String {
    match s.find(' ') {
        Some(i) => return String::from(&s[..i]),
        None => {}
    }
    String::from(s)
}
fn check2() {
    println!("{} {}", first_word("ab cd"), first_word("xy"));
}
