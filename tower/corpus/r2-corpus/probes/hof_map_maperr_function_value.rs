// RUSTC-TARGET S1 (P-HIGHER-ORDER-METHOD-FUNCTION-VALUE-ARG, first rung): map/map_err on Option/Result
// given a FUNCTION VALUE arg (a bare fn name OR a variant constructor like `Either::Left`), not a
// `|x| body` closure literal. Pervasive: either`s `l.map_err(Either::Left)`. rzrs's HOF handlers assumed
// a closure literal and desynced on a fn value (running POS to the next `|` in the file). Now a shared
// emit_fnval_apply applies the fn value: a variant ctor -> [disc, payload]; a fn name -> call.
enum E { A(i32), B(i32) }
use E::{A, B};
fn dbl(x: i32) -> i32 { x * 2 }
fn main() {
    let r: Result<i32, i32> = Err(7);
    let m = r.map_err(E::A);          // Err(7) -> Err(E::A(7))
    match m { Ok(v) => println!("ok{}", v), Err(e) => match e { A(n) => println!("{}", n), B(n) => println!("b{}", n), } }
    let o: Option<i32> = Some(5);
    if let Some(v) = o.map(dbl) { println!("{}", v); }   // Some(5) -> Some(10)
    let k: Result<i32, i32> = Ok(3);
    if let Ok(v) = k.map(dbl) { println!("{}", v); }      // Ok(3) -> Ok(6)
}
