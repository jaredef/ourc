mod lexer {
    pub struct Token { pub kind: i64 }
    pub fn mk(k: i64) -> Token { Token { kind: k } }
}
use lexer::Token;
fn main() { let t = lexer::mk(9); println!("{}", t.kind); }
