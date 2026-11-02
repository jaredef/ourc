#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)] enum Tok { Num(i64), Id(String), Op { c: char, prec: u8 }, Eof }
fn main() { let mut t = vec![Tok::Eof, Tok::Id("b".into()), Tok::Num(3), Tok::Op { c: '+', prec: 1 }, Tok::Id("a".into()), Tok::Num(-1)]; t.sort(); println!("{:?}", t); println!("{:?} {:?}", Tok::Num(2).cmp(&Tok::Num(5)), Tok::Id("z".into()).cmp(&Tok::Id("a".into()))); }
