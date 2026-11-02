// P-CX-ORPAT-BIND witness (ourc construct-generation, 2026-10-24): or-patterns inside a variant payload
// (`Punct('+' | '-')`, `Punct(c @ ('*' | '/'))`, by value and by reference) and tuple or-patterns whose
// alternatives bind the same names at different positions (`(0, x) | (x, 0)`, `(V(x), t) | (t, V(x))`).
#[derive(Debug, Clone, PartialEq)] enum Tok { Punct(char), Eof }
#[derive(Debug, Clone, PartialEq)] enum Ty { Int, Var(u32) }
enum T { A(u32), B(u32), C }
fn prec(t: &Tok) -> i32 { match t { Tok::Punct('+' | '-') => 1, Tok::Punct('*' | '/') => 2, _ => 0 } }
fn op(t: &Tok) -> String { match t { Tok::Punct(c @ ('+' | '-')) => format!("a{}", c), Tok::Punct(c @ ('*' | '/')) => format!("m{}", c), _ => "o".to_string() } }
fn opv(t: Tok) -> String { match t { Tok::Punct(c @ ('+' | '-')) => format!("a{}", c), Tok::Punct(c @ ('*' | '/')) => format!("m{}{}", c, c == '/'), _ => "o".to_string() } }
fn zero(a: (u32, u32)) -> u32 { match a { (0, x) | (x, 0) => x, (x, y) => x * y } }
fn pair(a: &T, b: &T) -> String { match (a, b) { (T::A(x), T::A(y)) | (T::B(x), T::B(y)) => format!("same{}{}", x, y), (T::A(x), _) | (_, T::A(x)) => format!("one{}", x), _ => "o".into() } }
fn var(a: &Ty, b: &Ty) -> String { match (a, b) { (Ty::Var(x), t) | (t, Ty::Var(x)) => format!("v{}{:?}", x, t), _ => "o".into() } }
fn main() {
    println!("{} {} {} {}", prec(&Tok::Punct('-')), prec(&Tok::Punct('/')), prec(&Tok::Punct('x')), prec(&Tok::Eof));
    println!("{} {} {} {}", op(&Tok::Punct('-')), op(&Tok::Punct('/')), op(&Tok::Punct('x')), opv(Tok::Punct('/')));
    println!("{} {} {}", zero((0, 5)), zero((6, 0)), zero((3, 4)));
    println!("{} {} {} {} {}", pair(&T::A(1), &T::A(2)), pair(&T::B(3), &T::B(4)), pair(&T::C, &T::A(5)), pair(&T::A(6), &T::C), pair(&T::C, &T::C));
    println!("{} {} {}", var(&Ty::Int, &Ty::Var(5)), var(&Ty::Var(6), &Ty::Int), var(&Ty::Int, &Ty::Int));
}
