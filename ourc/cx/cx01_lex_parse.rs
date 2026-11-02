// cx01 lex/parse/ast: rustc_lexer-shaped cursor over chars (incl. non-ASCII idents/strings),
// a recursive-descent Pratt parser into Box<Expr>, nested-pattern matches (PATPARSE-nested).
#[derive(Debug, Clone, PartialEq)]
enum Tok { Ident(String), Int(i64), Str(String), Punct(char), Eof }
struct Cursor<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
impl<'a> Cursor<'a> {
    fn new(s: &'a str) -> Self { Cursor { chars: s.chars().peekable() } }
    fn next_tok(&mut self) -> Tok {
        while let Some(&c) = self.chars.peek() { if c.is_whitespace() { self.chars.next(); } else { break; } }
        let c = match self.chars.next() { Some(c) => c, None => return Tok::Eof };
        if c.is_alphabetic() || c == '_' {
            let mut s = String::new(); s.push(c);
            while let Some(&d) = self.chars.peek() { if d.is_alphanumeric() || d == '_' { s.push(d); self.chars.next(); } else { break; } }
            Tok::Ident(s)
        } else if c.is_ascii_digit() {
            let mut n = c.to_digit(10).unwrap() as i64;
            while let Some(&d) = self.chars.peek() { if let Some(v) = d.to_digit(10) { n = n * 10 + v as i64; self.chars.next(); } else { break; } }
            Tok::Int(n)
        } else if c == '"' {
            let mut s = String::new();
            while let Some(d) = self.chars.next() { if d == '"' { break; } s.push(d); }
            Tok::Str(s)
        } else { Tok::Punct(c) }
    }
}
#[derive(Debug)]
enum Expr { Lit(i64), Var(String), Str(String), Bin(char, Box<Expr>, Box<Expr>), Neg(Box<Expr>) }
struct Parser { toks: Vec<Tok>, pos: usize }
impl Parser {
    fn peek(&self) -> &Tok { &self.toks[self.pos] }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
    fn prec(t: &Tok) -> Option<(u8, char)> { match t { Tok::Punct(c @ ('+' | '-')) => Some((1, *c)), Tok::Punct(c @ ('*' | '/')) => Some((2, *c)), _ => None } }
    fn atom(&mut self) -> Expr {
        match self.bump() {
            Tok::Int(n) => Expr::Lit(n), Tok::Ident(s) => Expr::Var(s), Tok::Str(s) => Expr::Str(s),
            Tok::Punct('-') => Expr::Neg(Box::new(self.atom())),
            Tok::Punct('(') => { let e = self.expr(0); self.bump(); e }
            t => panic!("unexpected {:?}", t),
        }
    }
    fn expr(&mut self, min: u8) -> Expr {
        let mut lhs = self.atom();
        while let Some((p, op)) = Parser::prec(self.peek()) {
            if p <= min && min != 0 && p < min + 1 { break; }
            if p < min { break; }
            self.bump(); let rhs = self.expr(p + 1);
            lhs = Expr::Bin(op, Box::new(lhs), Box::new(rhs));
        }
        lhs
    }
}
fn eval(e: &Expr) -> i64 { match e { Expr::Lit(n) => *n, Expr::Var(s) => s.chars().count() as i64, Expr::Str(s) => s.len() as i64,
    Expr::Neg(x) => -eval(x), Expr::Bin(op, a, b) => { let (x, y) = (eval(a), eval(b)); match op { '+' => x + y, '-' => x - y, '*' => x * y, _ => x / y } } } }
fn shape(e: &Expr) -> &'static str {
    match e {
        Expr::Bin('+', l, r) => match (&**l, &**r) { (Expr::Lit(_), Expr::Bin('*', _, _)) => "lit+mul", (Expr::Var(_), _) => "var+any", _ => "add" },
        Expr::Bin(_, l, _) if matches!(**l, Expr::Neg(_)) => "neg-lhs",
        Expr::Neg(inner) => match &**inner { Expr::Lit(n) if *n > 5 => "neg-big", _ => "neg" },
        _ => "other",
    }
}
fn main() {
    let srcs = ["1 + 2 * 3", "(1 + 2) * 3", "-4 * 2 - 1", "größe + 3", "\"héllo\" * 2", "-9", "λx + 1 - 2"];
    for s in srcs.iter() {
        let mut c = Cursor::new(s); let mut toks = Vec::new();
        loop { let t = c.next_tok(); let end = t == Tok::Eof; toks.push(t); if end { break; } }
        let n = toks.len();
        let mut p = Parser { toks, pos: 0 }; let e = p.expr(0);
        println!("{:<14} toks={} val={} shape={} chars={} bytes={}", s, n, eval(&e), shape(&e), s.chars().count(), s.len());
    }
    let pairs: Vec<(Option<(i64, Tok)>, bool)> = vec![(Some((1, Tok::Ident("a".to_string()))), true), (Some((2, Tok::Int(7))), false), (None, true)];
    for p in &pairs { let r = match p { (Some((k, Tok::Ident(ref s))), true) => format!("id{}{}", k, s), (Some((_, Tok::Int(v))), _) => format!("int{}", v), (None, b) => format!("none{}", b), _ => "?".to_string() }; println!("{}", r); }
}
