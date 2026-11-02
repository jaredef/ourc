#[derive(Debug, Clone, Copy, PartialEq)] enum DefKind { Fn, Mod }
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(DefKind, u32), Err }
fn look(x: u32) -> Option<Res> { if x == 0 { Some(Res::Def(DefKind::Mod, 7)) } else if x == 1 { Some(Res::Def(DefKind::Fn, 2)) } else { None } }
fn f(x: u32) -> Res { let m = match look(x) { Some(Res::Def(DefKind::Mod, id)) => id as usize, _ => return Res::Err }; Res::Def(DefKind::Fn, m as u32) }
fn main() { println!("{:?} {:?} {:?}", f(0), f(1), f(2)); }
