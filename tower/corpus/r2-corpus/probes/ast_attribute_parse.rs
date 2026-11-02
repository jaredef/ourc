type ThinVec<T> = Vec<T>;
enum AttrKind { Normal, Doc }
struct Attribute { kind: AttrKind, name: String, args: ThinVec<String> }
struct Item { attrs: ThinVec<Attribute>, name: String }
struct Parser { src: Vec<u8>, pos: usize }
impl Parser {
    fn peek(&self) -> u8 { if self.pos < self.src.len() { self.src[self.pos] } else { 0 } }
    fn bump(&mut self) -> u8 { let c = self.peek(); self.pos += 1; c }
    fn skip_ws(&mut self) { while self.pos < self.src.len() && (self.src[self.pos] == 32 || self.src[self.pos] == 10) { self.pos += 1; } }
    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || (c >= 48 && c <= 57) || c == 95 {
                s.push(c as char); self.pos += 1;
            } else { break; }
        }
        s
    }
    fn parse_attrs(&mut self) -> ThinVec<Attribute> {
        let mut out: ThinVec<Attribute> = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == 35 {
                self.bump();
                self.bump();
                let name = self.read_ident();
                let mut args: ThinVec<String> = Vec::new();
                if self.peek() == 40 {
                    self.bump();
                    loop {
                        self.skip_ws();
                        if self.peek() == 41 { self.bump(); break; }
                        let a = self.read_ident();
                        args.push(a);
                        self.skip_ws();
                        if self.peek() == 44 { self.bump(); } else { self.bump(); break; }
                    }
                }
                if self.peek() == 93 { self.bump(); }
                out.push(Attribute { kind: AttrKind::Normal, name: name, args: args });
            } else { break; }
        }
        out
    }
}
fn dump_attr(a: &Attribute) -> String {
    let mut s = String::new();
    s.push_str("#[");
    s.push_str(&a.name);
    if a.args.len() > 0 {
        s.push_str("(");
        let mut first = true;
        for arg in a.args.iter() { if !first { s.push_str(","); } first = false; s.push_str(arg); }
        s.push_str(")");
    }
    s.push_str("]");
    s
}
fn main() {
    let src = "#[derive(Clone, Copy)] #[inline]";
    let mut p = Parser { src: src.as_bytes().to_vec(), pos: 0 };
    let attrs = p.parse_attrs();
    for a in attrs.iter() { println!("{}", dump_attr(a)); }
    println!("{}", attrs.len());
}
