// Two enums share a variant NAME (`Let`), as olex's token enum and oast's expression enum do. A match
// whose scrutinee type is not recorded -- here a `&E` parameter -- must still find the right enum, and
// one variant name cannot do it: all the arms' names together can.
mod tok { pub enum Tk { Let, Ident, Int }
  pub fn kind(t:&Tk)->i64{ match t { Tk::Let=>1, Tk::Ident=>2, _=>3 } } }
mod ast { pub enum E { Int(i64), While(Box<E>), Let(i64,Box<E>), If(Box<E>) } }
mod q { use crate::ast::*;
  pub fn block_form(e:&E)->bool{ match e { E::Let(_v,b)=>block_form(b), E::While(_b)=>true, E::If(_c)=>true, _=>false } }
  pub fn mk()->E{ E::Let(1, Box::new(E::While(Box::new(E::Int(0))))) }
  pub fn mk2()->E{ E::Let(1, Box::new(E::Int(0))) } }
fn main(){
    let a=q::mk(); let b=q::mk2();
    println!("{} {} {}", q::block_form(&a), q::block_form(&b), tok::kind(&tok::Tk::Ident));
}
