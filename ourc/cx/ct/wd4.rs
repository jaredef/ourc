#[derive(Debug, Clone, Copy, PartialEq, Eq)] enum Q { T, S, L }
fn f(w: &Vec<((Q, u32), (Q, u32))>, node: (Q, u32)) -> usize { w.iter().filter(|(p, _)| *p == node).count() }
fn main() { let w = vec![((Q::L, 1), (Q::S, 1)), ((Q::S, 1), (Q::T, 1))]; println!("{} {} {}", f(&w, (Q::S, 1)), f(&w, (Q::L, 1)), f(&w, (Q::T, 1))); let a = (Q::L, 1u32); let b = (Q::S, 1u32); println!("{}", a == b); }
