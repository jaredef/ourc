// R38: a `let v: Vec<Enum> = Vec::new()` built by .push() prints its variant names under {:?},
// not raw pointers. R23b/R34 carried the element enum-type only from an inline vec![Enum]
// literal (VELENUM); Vec::new()+push carries no element in the RHS, so the annotation's element
// enum-type feeds SVELENUM. Covers pushes inside control flow (the lexer idiom).
#[derive(Debug)]
enum Token {
    Num(i32),
    Plus,
    Ident(String),
}

fn lex(s: &str) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    for w in s.split_whitespace() {
        if w == "+" { out.push(Token::Plus); }
        else if let Ok(n) = w.parse::<i32>() { out.push(Token::Num(n)); }
        else { out.push(Token::Ident(String::from(w))); }
    }
    out
}

fn main() {
    println!("{:?}", lex("3 + foo"));

    let mut v: Vec<Token> = Vec::new();
    v.push(Token::Num(1));
    v.push(Token::Plus);
    println!("{:?}", v);
}
