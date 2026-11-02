#[derive(Clone, Copy)]
enum N { A, B }
fn same(x: N, y: N) -> bool { match (x, y) { (N::A, N::A) => true, (N::B, N::B) => true, _ => false } }
fn main() {
    println!("{}", same(N::A, N::A));
    println!("{}", same(N::A, N::B));
    println!("{}", same(N::B, N::B));
    println!("{}", same(N::B, N::A));
}
