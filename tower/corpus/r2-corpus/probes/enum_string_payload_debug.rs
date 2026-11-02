// R34: {:?} on an enum whose variant carries a String / &str payload prints the value QUOTED
// (was: a raw heap pointer). Covers tuple-variants, struct-variants, and Vec<Enum> elements,
// alongside i32-payload variants to guard the non-string path.
#[derive(Debug)]
enum Token {
    Num(i32),
    Plus,
    Ident(String),
    Named { name: String, arity: i32 },
}

fn main() {
    let toks = vec![
        Token::Num(3),
        Token::Plus,
        Token::Ident(String::from("foo")),
        Token::Named { name: String::from("f"), arity: 2 }
    ];
    println!("{:?}", toks);
    println!("{:?}", Token::Ident(String::from("bar")));
    println!("{:?}", Token::Num(7));
}
