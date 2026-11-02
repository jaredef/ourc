pub enum Ex { Lit(i64), Add(Box<Ex>, Box<Ex>) }
pub struct Fun { pub name: String, pub params: Vec<String>, pub body: Ex }
pub struct Prog { pub funs: Vec<Fun>, pub main: Ex }
pub fn mk()->Prog{
    let mut out:Vec<Fun>=Vec::new();
    let e:Vec<String>=Vec::new();
    out.push(Fun{name:String::from("zero"), params:e, body:Ex::Lit(1)});
    let mut p2:Vec<String>=Vec::new(); p2.push(String::from("x"));
    out.push(Fun{name:String::from("one"), params:p2, body:Ex::Lit(2)});
    Prog{funs:out, main:Ex::Lit(9)}
}
pub fn ex_val(e:&Ex)->i64{ match e { Ex::Lit(v)=>*v, Ex::Add(_a,_b)=>0 } }
