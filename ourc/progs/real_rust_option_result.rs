// O1-O12: the Option/Result surface as prelude functions over the prelude's own enums, and STRUCTURAL
// enum equality -- we compared handles, so `Some(1) == Some(1)` was false (two allocations).
fn id(v: i32) -> i32 { v }
fn get(n: i32) -> Option<i32> { let v = if n > 0 { Some(n) } else { None }; let x = v?; Some(x + 1) }
fn main() {
    let a: Option<i32> = Some(id(3));
    let b: Option<i32> = None;
    println!("{} {}", a.unwrap(), a.expect("nope"));
    println!("{} {}", b.unwrap_or(7), a.unwrap_or(7));
    println!("{} {} {} {}", a.is_some(), a.is_none(), b.is_some(), b.is_none());
    println!("{:?} {:?}", a.map(|x| x * 5), b.map(|x| x * 5));
    let r: Result<i32, i32> = Ok(4);
    let e: Result<i32, i32> = Err(9);
    println!("{} {} {} {} {}", r.unwrap(), r.is_ok(), r.is_err(), e.is_ok(), e.is_err());
    println!("{:?} {}", "42".parse::<i32>(), "x".parse::<i32>().is_err());
    println!("{:?} {:?}", get(5), get(0));
    println!("{} {}", a == Some(3), a == None);
}
