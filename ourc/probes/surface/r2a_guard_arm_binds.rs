// R2a P4 guard: a match with ANY guarded arm still types every arm's payload bindings -- a range slice of a String payload
// (`&ty[1..]`) in an unguarded arm, guards that read payload fields, an int match with a guard.
enum Op { CallOp(u32, String, Vec<u32>), CastOp(u32, u32, String) }
fn g(s: &str) -> usize { s.len() }
fn f(o: &Op) -> usize {
    match o {
        Op::CallOp(_t, n, av) if av.len() == 1 && n.as_str() == "p" => av[0] as usize + 100,
        Op::CastOp(t, _sx, ty) if ty.len() > 2 => g(&ty[1..]) + *t as usize,
        Op::CastOp(_t, _sx, ty) => { let w = &ty[0..]; g(w) * 10 }
        _ => 0,
    }
}
fn h(x: i64, s: &str) -> usize { match x { 1 => s.len(), n if n > 5 => (&s[1..]).len(), _ => 0 } }
fn main() {
    println!("{} {} {} {} {}", f(&Op::CastOp(1, 2, String::from("abcd"))), f(&Op::CallOp(1, String::from("p"), vec![7])),
        f(&Op::CastOp(1, 2, String::from("a"))), h(9, "abc"), h(1, "abc"));
}
