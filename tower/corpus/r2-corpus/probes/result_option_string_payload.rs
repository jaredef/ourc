// LAYER2 P1 (built-in slice): Option/Result String payload restored at the match/if-let binder via the
// scrutinee's EOPTK, applied ONLY to the arm that carries it -- Option-Some or Result-Err, never Result-Ok
// (whose payload is the Ok type). Was: the Some(s)/Err(e) payload printed the descriptor pointer, and a
// naive fix tagged the Result-Ok i32 arm as a String. Residual: match on a `?`-propagating call's return.
fn parse(x: i32) -> Result<i32, String> {
    if x >= 0 { Ok(x * 2) } else { Err(format!("negative: {}", x)) }
}
fn main() {
    match parse(-3) {
        Ok(v) => println!("ok {}", v),
        Err(e) => println!("err {}", e),
    }
    let r = parse(4);
    match r {
        Ok(v) => println!("ok {}", v),
        Err(e) => println!("err {}", e),
    }
    let name: Option<String> = Some(String::from("cruft"));
    if let Some(s) = name {
        println!("name {}", s);
    }
    let empty: Option<String> = None;
    match empty {
        Some(s) => println!("some {}", s),
        None => println!("nothing"),
    }
}
