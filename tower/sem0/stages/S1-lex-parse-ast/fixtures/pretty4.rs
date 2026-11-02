// A reduced Wadler-style pretty printer, modeled on rustc_ast_pretty::pp.
// Exercises a spread of pretty-printer idioms for the S1 AST fixture.

const SIZE_INFINITY: isize = 0xffff;
const INDENT_UNIT: usize = 4;

#[derive(Clone)]
enum Token {
    Word(String),
    Break(isize),
}

impl Token {
    fn width(&self) -> isize {
        match self {
            Token::Word(s) => s.len() as isize,
            Token::Break(n) => *n,
            _ => 0,
        }
    }

    fn is_hardbreak(&self) -> bool {
        match self {
            Token::Break(n) => *n == SIZE_INFINITY,
            _ => false,
        }
    }
}

struct Printer {
    out: String,
    margin: isize,
    space: isize,
    buf: Vec<Token>,
    col: usize,
}

impl Printer {
    fn new() -> Self {
        let margin = 78;
        Printer {
            out: String::new(),
            margin: margin,
            space: margin,
            buf: Vec::with_capacity(16),
            col: 0,
        }
    }

    fn scan(&mut self, tok: Token) {
        self.space = self.space - tok.width();
        self.buf.push(tok);
    }

    fn word(&mut self, s: &str) {
        self.scan(Token::Word(s.to_string()));
    }

    fn space(&mut self) {
        self.scan(Token::Break(1));
    }

    fn hardbreak(&mut self) {
        self.scan(Token::Break(SIZE_INFINITY));
    }

    fn commasep(&mut self, items: &[String]) {
        let mut first = true;
        for it in items {
            if !first {
                self.word(",");
                self.space();
            }
            self.word(it);
            first = false;
        }
    }

    fn render(&mut self) -> String {
        for tok in &self.buf {
            if tok.is_hardbreak() {
                self.out.push('\n');
                self.col = 0;
            } else if let Token::Word(s) = tok {
                self.out.push_str(s);
                self.col = self.col + s.chars().count();
            }
        }
        self.out.clone()
    }
}

fn main() {
    let mut p = Printer::new();
    p.space = INDENT_UNIT as isize;
    p.word("hello");
    p.space();
    let parts = vec![String::from("a"), String::from("b")];
    p.commasep(&parts);
    p.hardbreak();
    let rendered = p.render();
    println!("{}", rendered);
}
