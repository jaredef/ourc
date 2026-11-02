#[derive(PartialEq, Debug)]
enum Tok { Kw { name: String, line: i64 }, Eof }
fn main() {
    let a = Tok::Kw { name: String::from("fn"), line: 3 };
    let b = Tok::Kw { name: String::from("fn"), line: 3 };
    let c = Tok::Kw { name: String::from("fn"), line: 4 };
    println!("{} {}", a == b, a == c);
    println!("{:?}", a);
}
