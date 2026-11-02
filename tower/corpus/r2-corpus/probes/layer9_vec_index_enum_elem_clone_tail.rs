#[derive(Clone)]
enum Ty { Bool, I32, Str, Ref(Box<Ty>), Infer(u32), Error }
fn name(t: &Ty) -> &str { match t { Ty::I32 => "i32", _ => "?" } }
fn res(table: &Vec<Ty>, t: &Ty) -> Ty {
    match t {
        Ty::Infer(v) => { let b = table[*v as usize].clone(); match b { Ty::Infer(w) => Ty::Infer(w), _ => b } }
        _ => t.clone(),
    }
}
fn main() {
    let mut table: Vec<Ty> = vec![Ty::Bool];
    println!("{}", name(&res(&table, &Ty::Infer(0))));
}
