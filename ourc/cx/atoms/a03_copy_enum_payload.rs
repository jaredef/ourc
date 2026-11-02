#[derive(Debug, Clone, Copy, PartialEq)] enum DefKind { Fn, Mod }
#[derive(Debug, Clone, Copy, PartialEq)] enum Res { Def(DefKind, u32), Local(u32), Err }
fn main() { let v = vec![Res::Def(DefKind::Mod, 4), Res::Local(2), Res::Err]; let r = v[0]; let s = match r { Res::Def(DefKind::Mod, id) => id as usize, _ => 99 }; println!("{:?} {} {:?}", r, s, v.iter().rev().find(|x| **x != Res::Err)); }
