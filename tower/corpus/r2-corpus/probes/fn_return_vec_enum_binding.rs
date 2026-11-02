// R40: `let x = f()` where `f() -> Vec<Enum>` (or Vec<Struct>) now prints variant names / struct
// bodies under {:?}, not raw pointers. The callee's `-> Vec<Enum>` return element-type is recorded
// per-fn (FRVELENUM/FRVELT via scan_ret_kind) and threaded to the caller's binding (SVELENUM/SVELT).
// This is the lexer idiom `let toks = lex(src)`.
#[derive(Debug)]
enum Token { Num(i32), Plus, Ident(String) }

fn lex(s: &str) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    for w in s.split_whitespace() {
        if w == "+" { out.push(Token::Plus); }
        else if let Ok(n) = w.parse::<i32>() { out.push(Token::Num(n)); }
        else { out.push(Token::Ident(String::from(w))); }
    }
    out
}

#[derive(Debug)]
struct Pt { x: i32, y: i32 }

fn origins(n: i32) -> Vec<Pt> {
    let mut v: Vec<Pt> = Vec::new();
    let mut i = 0;
    while i < n { v.push(Pt { x: i, y: i * 2 }); i += 1; }
    v
}

fn main() {
    let toks = lex("3 + foo");
    println!("{:?}", toks);
    let pts = origins(2);
    println!("{:?}", pts);
}
