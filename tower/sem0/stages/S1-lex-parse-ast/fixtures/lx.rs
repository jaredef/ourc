pub enum Tok {
    Ident,
    Number,
    Plus,
    Minus,
    Eof,
}
pub fn classify(c: char) -> Tok {
    match c {
        'a'..='z' => Tok::Ident,
        '0'..='9' => Tok::Number,
        '+' => Tok::Plus,
        '-' => Tok::Minus,
        _ => Tok::Eof,
    }
}
pub fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}
pub fn scan(s: &str, i: usize) -> usize {
    let b = s.as_bytes();
    let mut j = i;
    while j < b.len() {
        j = j + 1;
    }
    j
}
